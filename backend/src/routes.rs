use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart},
    http::{header, Response, StatusCode},
    response::{AppendHeaders, IntoResponse},
    routing::post,
    Router,
};
use http_body::Frame;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};
use tower_http::cors::CorsLayer;
use zip::write::SimpleFileOptions;

#[derive(Serialize, Deserialize)]
struct FileData {
    content: String,
    name: String,
}

async fn handle_files(mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
    let output_name = "merged_files.zip";
    let zipfile = File::create(&output_name.to_string()).unwrap();
    let zip_options =
        SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    let mut zip = zip::ZipWriter::new(zipfile);

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = {
            let temp = field.file_name().unwrap().to_string();
            if let Some(idx) = temp.find('/') {
                (&temp[idx..]).to_string()
            } else {
                temp
            }
        };
        let bytes = field.bytes().await.unwrap();
        if let Ok(_) = zip.start_file(file_name.clone(), zip_options) {
            if let Err(e) = zip.write(&bytes[..]) {
                println!("Hello {}", e);
                return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)));
            }
        }
    }
    match zip.finish() {
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
            Ok((headers, bytes))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))),
    }
}
pub fn routes() -> Router {
    Router::new()
        .route("/helloworld", post(handle_files))
        .layer(DefaultBodyLimit::max(1 << 24)) //16MB
        .layer(CorsLayer::permissive())
}
