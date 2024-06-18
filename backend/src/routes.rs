use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart},
    http::{header, StatusCode},
    response::{AppendHeaders, IntoResponse},
    routing::post,
    Router,
};
use std::{fs::File, io::Write};
use tower_http::cors::CorsLayer;
use zip::write::{FileOptions, SimpleFileOptions};

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
    zip.write(textures.to_string().unwrap().as_bytes());
}
fn server_error<T: Into<String>>(err: T) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.into())
}
async fn handle_files(mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
    let output_name = "merged_files.zip";
    let zipfile = File::create(output_name).unwrap();

    let zip_options =
        SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    let mut zip = zip::ZipWriter::new(zipfile);

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
        let content = String::from_utf8(bytes.clone().into_iter().collect()).unwrap();
        let split = comp_file_name.split("/").collect::<Vec<&str>>();
        {
            match split[split.len() - 1] {
                "manifest.json" => {
                    match Manifest::from_string(content.clone()) {
                        Ok(manifest) => manifests.push(manifest),
                        Err(e) => {
                            return Err(server_error(format!("Could not read manifest file {}. Given error during serialization: {}",comp_file_name, e)));
                        }
                    };
                }
                "item_texture.json" => {
                    match ItemTxt::from_string(content.clone()) {
                        Ok(mut txt) => if let Some(ref item_txt) = item_txt {
                            txt.concat(item_txt.clone());
                        }else{
                            item_txt = Some(txt);
                        }
                        Err(e) => {
                            return Err(server_error(format!("Could not read item textures file {}. Given error during serialization: {}", comp_file_name, e)))
                        }
                    }
                }
                _ => {
                    println!("I dont think this was implemented yet {}", split[split.len() - 1]);
                }
            }
        }
        let file_name = {
            match Manifest::from_string(content.clone()) {
                Ok(manifest) => manifests.push(manifest),
                Err(_) => {
                    return Err(server_error(
                        "Could not read manifest file. Check if it's a valid one",
                    ));
                }
            };
            if split[1] == "scripts" {
                format!(
                    "{}_{}{}",
                    split[0],
                    split[1],
                    &comp_file_name[(1 + split[0].len() + split[1].len())..]
                )
            } else if let Some(idx) = comp_file_name.find('/') {
                (&comp_file_name[idx..]).to_string()
            } else {
                comp_file_name
            }
        };
        if let Ok(_) = zip.start_file(file_name, zip_options) {
            //Creates a file, if not error(duplicated file) writes into it
            if let Err(e) = zip.write(&bytes[..]) {
                return Err(server_error(format!("Error on write file {}", e)));
            }
            continue;
        }
    }
    if manifests.len() < 2 {
        return Err(server_error("There are not enough manifests for merging"));
    }
    create_manifest(&mut zip, zip_options, manifests);
    create_item_textures(&mut zip, zip_options, item_txt);
    match zip.finish() {
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
pub fn routes() -> Router {
    Router::new()
        .route("/merge", post(handle_files))
        .layer(DefaultBodyLimit::max(1 << 24)) //16MB
        .layer(CorsLayer::permissive())
}
