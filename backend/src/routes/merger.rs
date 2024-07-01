use axum::{
    body::Bytes,
    extract::{Multipart, Path},
    http::{header, StatusCode},
    response::{AppendHeaders, IntoResponse},
};
use std::{fs, io::Write};
use zip::{write::SimpleFileOptions, ZipWriter};

use crate::addons::{item_texture::ItemTxt, manifest::Manifest, terrain_texture::BlockTxt};

pub fn server_err<S: Into<String>>(err: S) -> (StatusCode, String) {
    (StatusCode::OK, err.into())
}

fn delete_files(path: &String) {
    if let Err(e) = fs::remove_file(path) {
        println!("{e}");
    };
}

fn write_on_file(
    zip: &mut ZipWriter<&std::fs::File>,
    file_name: String,
    opts: SimpleFileOptions,
    bytes: &[u8],
) -> Result<(), (StatusCode, String)> {
    if let Ok(_) = zip.start_file(file_name, opts) {
        if let Err(e) = zip.write(bytes) {
            return Err(server_err(format!("Could not write file {}", e)));
        }
    }
    Ok(())
}
fn write_item_txt(zip: &mut ZipWriter<&std::fs::File>, opts: SimpleFileOptions, bytes: &[u8]) {
    write_on_file(zip, "textures/item_texture.json".to_string(), opts, bytes);
}
fn write_block_txt(zip: &mut ZipWriter<&std::fs::File>, opts: SimpleFileOptions, bytes: &[u8]) {
    write_on_file(
        zip,
        "textures/terrain_texture.json".to_string(),
        opts,
        bytes,
    );
}
fn write_errs(zip: &mut ZipWriter<&std::fs::File>, opts: SimpleFileOptions, bytes: &[u8]) {
    write_on_file(zip, "log_errs.txt".to_string(), opts, bytes);
}
fn register_by_name(
    name: &str,
    complete_name: &String,
    bytes: &[u8],
    manifests: &mut Vec<Manifest>,
    item_txt: &mut ItemTxt,
    block_txt: &mut BlockTxt,
    errs: &mut Vec<String>,
) -> bool {
    match name {
        "manifest.json" => {
            match std::str::from_utf8(bytes) {
                Ok(str) => match Manifest::from_text(str) {
                    Ok(manifest) => manifests.push(manifest),
                    Err(e) => {
                        errs.push(format!("Error while reading '{}': {:#?}", complete_name, e))
                    }
                },
                Err(e) => errs.push(format!("Error while reading '{}': {:#?}", complete_name, e)),
            };
            true
        }
        "item_texture.json" => {
            match std::str::from_utf8(bytes) {
                Ok(str) => match ItemTxt::from_text(str) {
                    Ok(txt) => item_txt.concat(txt),
                    Err(e) => {
                        errs.push(format!("Error while reading '{}': {:#?}", complete_name, e))
                    }
                },
                Err(e) => errs.push(format!("Error while reading '{}': {:#?}", complete_name, e)),
            };
            true
        }
        "terrain_texture.json" => {
            match std::str::from_utf8(bytes) {
                Ok(str) => match BlockTxt::from_text(str) {
                    Ok(txt) => block_txt.concat(txt),
                    Err(e) => {
                        errs.push(format!("Error while reading '{}': {:#?}", complete_name, e))
                    }
                },
                Err(e) => errs.push(format!("Error while reading '{}': {:#?}", complete_name, e)),
            };
            true
        }
        _ => false,
    }
}

pub async fn process_res(
    Path(path): Path<String>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut manifests: Vec<Manifest> = Vec::new();
    let mut item_txt = ItemTxt::new("merge");
    let mut block_txt = BlockTxt::new("merge");
    let mut log_errs: Vec<String> = Vec::new();
    let name = format!("created_files/{}.zip", path);

    let file = match fs::File::create(&name) {
        Ok(file) => file,
        Err(e) => return Err(server_err(format!("{:#?}", e))),
    };

    let mut zip = zip::ZipWriter::new(&file);
    let opts = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    while let Ok(Some(field)) = multipart.next_field().await {
        let complete_name = match field.file_name() {
            Some(name) => name.to_string(),
            None => continue,
        };
        let bytes = match field.bytes().await {
            Ok(bytes) => bytes,
            Err(_) => continue,
        };
        {
            let split = complete_name.split("/").collect::<Vec<&str>>();
            let last_name = split[split.len() - 1];
            if register_by_name(
                last_name,
                &complete_name,
                bytes.to_vec().as_slice(),
                &mut manifests,
                &mut item_txt,
                &mut block_txt,
                &mut log_errs,
            ) {
                continue;
            };
        }
        let file_name = if let Some(idx) = complete_name.find("/") {
            (&complete_name[idx..]).to_string()
        } else {
            complete_name
        };
        if let Err(e) = write_on_file(&mut zip, file_name, opts, &bytes[..]) {
            return Err(e);
        }
    }
    let manifest_txt = if manifests.len() >= 2 {
        if let [first, second] = &mut manifests[..2] {
            Manifest::create_from(first, second)
        } else {
            Manifest::default()
        }
    } else {
        Manifest::default()
    }
    .to_string()
    .unwrap();
    match write_on_file(
        &mut zip,
        "manifest.json".to_string(),
        opts,
        manifest_txt.as_bytes(),
    ) {
        Ok(_) => {}
        Err(e) => return Err(e),
    };
    {
        let zipref = &mut zip;
        write_item_txt(zipref, opts, item_txt.to_string().unwrap().as_bytes());
        write_block_txt(zipref, opts, block_txt.to_string().unwrap().as_bytes());
        write_errs(zipref, opts, log_errs.join("\n").as_bytes());
    };
    match zip.finish() {
        Ok(_) => {
            let bytes = match fs::read(&name) {
                Err(e) => return Err(server_err(format!("{:#?}", e))),
                Ok(buffer) => Bytes::from(buffer),
            };
            let headers = AppendHeaders([
                (header::CONTENT_TYPE, "application/zip"),
                (
                    header::CONTENT_DISPOSITION,
                    "inline; filename=\"merged_addons.zip\"",
                ),
            ]);
            delete_files(&name);
            Ok((headers, bytes))
        }
        Err(e) => Err(server_err(format!("{:#?}", e))),
    }
}
