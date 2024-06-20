use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart, Path, Query},
    http::{header, StatusCode},
    response::{AppendHeaders, IntoResponse},
    routing::post,
    Router,
};
use std::{fs::File, io::Write, string::FromUtf8Error};
use tower_http::cors::CorsLayer;
use zip::{
    write::{FileOptions, SimpleFileOptions},
    ZipWriter,
};

use crate::addons::{item_texture::ItemTxt, manifest::Manifest};

fn create_manifest(
    zip: &mut zip::ZipWriter<File>,
    options: FileOptions<'_, ()>,
    mut manifests: Vec<Manifest>,
) {
    if let [first, second, ..] = &mut manifests[..2] {
        if let Ok(result) = Manifest::new(first, second).to_string() {
            zip.start_file("manifest.json", options).unwrap();
            zip.write(result.as_bytes()).unwrap();
        } else {
            println!("Error during parsing manifest struct to json")
        }
    }
}
fn create_item_textures(
    zip: &mut zip::ZipWriter<File>,
    options: FileOptions<'_, ()>,
    textures: ItemTxt,
) {
    zip.start_file("textures/item_texture.json", options)
        .unwrap();
    let _ = zip.write(textures.to_string().unwrap().as_bytes());
}
fn server_error<T: Into<String>>(err: T) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.into())
}

fn gen_initial_file<'a>(file: &str) -> (FileOptions<'a, ()>, ZipWriter<File>) {
    let file = File::create(file).unwrap();
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    (options, ZipWriter::new(file))
}
fn bytes_to_string(bytes: &Bytes) -> Result<String, FromUtf8Error> {
    String::from_utf8(bytes.clone().into_iter().collect::<Vec<u8>>())
}
async fn process_behavior(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let output_name = "merged_files.zip";
    let (options, mut writer) = gen_initial_file(output_name);

    let mut manifests: Vec<Manifest> = vec![];

    while let Ok(Some(field)) = multipart.next_field().await {
        let comp_file_name = field.file_name().unwrap().to_string();
        let bytes = match field.bytes().await {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(server_error(format!("Could not read file {}. By now addons with sound files and mcstructure ain't able to merge", comp_file_name)));
            }
        };
        let content = match String::from_utf16(
            bytes
                .clone()
                .into_iter()
                .map(|b| b as u16)
                .collect::<Vec<u16>>()
                .as_slice(),
        ) {
            Ok(str) => str,
            Err(_) => {
                println!("{}", comp_file_name);
                return Err(server_error(format!(
                    "{comp_file_name} is not a valid utf16 file"
                )));
            }
        };
        let split = comp_file_name.split("/").collect::<Vec<&str>>();
        'file: {
            match split[split.len() - 1] {
                "manifest.json" => {
                    if let Ok(content) = bytes_to_string(&bytes) {
                        if let Ok(manifest) = Manifest::from_string(content) {
                            manifests.push(manifest);
                            break 'file;
                        }
                        return Err(server_error("Manifest file is not valid"));
                    }
                    return Err(server_error(
                        "Could not read the manifest file as a utf8 file",
                    ));
                }
                _ => { /*add about player.json and tick.json files later*/ }
            }
        }
        let file_name = if split[1] == "scripts" {
            format!(
                "{}_{}{}",
                split[0],
                split[1],
                &comp_file_name[(1 + split[0].len() + split[1].len())..] //behFolderName_scripts/*
            ) //update later
        } else if let Some(idx) = comp_file_name.find('/') {
            (&comp_file_name[idx..]).to_string() //remove behFolderName
        } else {
            comp_file_name
        };
        if let Ok(_) = writer.start_file(file_name, options) {
            //Creates a file, if not error(duplicated file) writes into it
            if let Err(e) = writer.write(&bytes[..]) {
                return Err(server_error(format!("Error on write file {}", e)));
            }
        }
    } //end loop

    if manifests.len() < 2 {
        return Err(server_error("There are not enough manifests for merging"));
    }
    create_manifest(&mut writer, options, manifests);
    match writer.finish() {
        //finishes the zip management
        Ok(_) => {
            let buffer = std::fs::read(output_name).unwrap();
            let bytes = Bytes::from(buffer);
            let headers = AppendHeaders([
                (header::CONTENT_TYPE, "application/zip"),
                (
                    header::CONTENT_DISPOSITION,
                    "inline; filename=\"merged_addons.zip\"",
                ),
            ]);
            let _ = std::fs::remove_file(output_name);
            Ok((headers, bytes))
        }
        Err(e) => {
            let _ = std::fs::remove_file(output_name);
            Err(server_error(format!("{}", e)))
        }
    }
}
async fn process_resource(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let output_name = "merged_files.zip";
    let (options, mut writer) = gen_initial_file(output_name);

    let mut manifests: Vec<Manifest> = vec![];
    let mut item_txt: Option<ItemTxt> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let comp_file_name = field.file_name().unwrap().to_string();
        let bytes = match field.bytes().await {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(server_error(format!("Could not read file {}. By now addons with sound files and mcstructure ain't able to merge", comp_file_name)));
            }
        };
        let split = comp_file_name.split("/").collect::<Vec<&str>>();
        'file: {
            match split[split.len() - 1] {
                "manifest.json" => {
                    if let Ok(content) = bytes_to_string(&bytes) {
                        if let Ok(manifest) = Manifest::from_string(content) {
                            manifests.push(manifest);
                            break 'file;
                        }
                        return Err(server_error("Manifest file is not valid"));
                    }
                    return Err(server_error(
                        "Could not read the manifest file as a utf8 file",
                    ));
                }
                "item_texture.json" => {
                    if let Ok(content) = bytes_to_string(&bytes) {
                        if let Ok(txt) = ItemTxt::from_string(content) {
                            if let Some(ref mut item_txt) = item_txt {
                                item_txt.concat(txt);
                            } else {
                                item_txt = Some(txt);
                            }
                            break 'file;
                        }
                        return Err(server_error("Item texture file is not valid"));
                    }
                    return Err(server_error(
                        "Could not read the item texture as a utf8 file",
                    ));
                }
                _ => {
                    //add for entity/player.json
                    //println!("I dont think this was implemented yet {}", split[split.len() - 1]);
                }
            }
        }
        let file_name = if let Some(idx) = comp_file_name.find('/') {
            (&comp_file_name[idx..]).to_string()
        } else {
            comp_file_name
        };
        if let Ok(_) = writer.start_file(file_name, options) {
            //Creates a file, if not error(duplicated file) writes into it
            if let Err(e) = writer.write(&bytes[..]) {
                return Err(server_error(format!("Error on write file {}", e)));
            }
            continue;
        }
    } //end loop

    if manifests.len() < 2 {
        return Err(server_error("There are not enough manifests for merging"));
    }
    create_manifest(&mut writer, options, manifests);
    if let Some(txt) = item_txt {
        create_item_textures(&mut writer, options, txt);
    }
    match writer.finish() {
        //finishes the zip management
        Ok(_) => {
            let buffer = std::fs::read(output_name).unwrap();
            let bytes = Bytes::from(buffer);
            let headers = AppendHeaders([
                (header::CONTENT_TYPE, "application/zip"),
                (
                    header::CONTENT_DISPOSITION,
                    "inline; filename=\"merged_addons.zip\"",
                ),
            ]);
            let _ = std::fs::remove_file(output_name);
            Ok((headers, bytes))
        }
        Err(e) => {
            let _ = std::fs::remove_file(output_name);
            Err(server_error(format!("{}", e)))
        }
    } //separar depois
}

pub fn routes() -> Router {
    Router::new()
        .route("/merge_resource", post(process_resource))
        .route("/merge_behavior", post(process_behavior))
        //.route("/merge_addon", post(process_addon))
        .layer(DefaultBodyLimit::max(1 << 24)) //16MB
        .layer(CorsLayer::permissive())
}
