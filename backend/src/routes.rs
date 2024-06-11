use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart},
    http::{header, StatusCode},
    response::{AppendHeaders, IntoResponse},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};
use tower_http::cors::CorsLayer;
use zip::write::{FileOptions, SimpleFileOptions};

#[derive(Serialize, Deserialize)]
struct FileData {
    content: String,
    name: String,
}
async fn create_manifest(zip: &mut zip::ZipWriter<File>, options: FileOptions<'_, ()>) {
    let manifest_preset = std::fs::read("presets/manifest.txt").unwrap();
    zip.start_file("manifest.json", options).unwrap();
    zip.write(&manifest_preset).unwrap();
}
async fn handle_files(mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
    let output_name = "merged_files.zip";
    let zipfile = File::create(output_name).unwrap();
    let zip_options =
        SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    let mut zip = zip::ZipWriter::new(zipfile);
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = {
            let temp = field.file_name().unwrap().to_string();
            let split = temp.split("/").collect::<Vec<&str>>();
            if split[1] == "scripts" {
                format!(
                    "{}_{}{}",
                    split[0],
                    split[1],
                    &temp[(1 + split[0].len() + split[1].len())..]
                )
            } else if let Some(idx) = temp.find('/') {
                (&temp[idx..]).to_string()
            } else {
                temp
            }
        };
        let bytes = field.bytes().await.unwrap();
        if let Ok(_) = zip.start_file(file_name, zip_options) {
            //Creates a file, if not
            //error(duplicated file) writes into it
            if let Err(e) = zip.write(&bytes[..]) {
                println!("Hello {}", e);
                return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)));
            }
            continue;
        }
    }
    create_manifest(&mut zip, zip_options).await;
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
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))
        }
    }
}
pub fn routes() -> Router {
    Router::new()
        .route("/helloworld", post(handle_files))
        .layer(DefaultBodyLimit::max(1 << 24)) //16MB
        .layer(CorsLayer::permissive())
}
