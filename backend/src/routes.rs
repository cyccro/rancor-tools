use std::{fs::{self, File, OpenOptions}, io::{Read,Write}};
use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart, Path},
    http::{header,StatusCode},
    response::{AppendHeaders,IntoResponse},
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use uuid::Uuid;
use zip::{write::SimpleFileOptions, ZipArchive, ZipWriter};

pub fn server_err<S: Into<String>>(err: S) -> (StatusCode, String) {
    (StatusCode::OK, err.into())
}

async fn delete_files(Path(path):Path<String>) {
    let output_path = format!("output_files/{}.zip",path);
    let dir_path = format!("created_files/{}",path);
    if let Err(e) = fs::remove_dir_all(dir_path){
        println!("{e}");
    };
    if let Err(e) = fs::remove_file(output_path) {
        println!("{e}");
    };
    
}

async fn finish_res(Path(path):Path<String>) -> Result<impl IntoResponse, (StatusCode,String)> {
    let output_path = format!("output_files/{}.zip",path);
    let content = match fs::read(&output_path){
        Ok(content) => content,
        Err(_) => return Err(server_err("internal error: not possible to read file"))
    };
    let bytes = Bytes::from(content);
    let headers = AppendHeaders([
        (header::CONTENT_TYPE, "application/zip"),
        (header::CONTENT_DISPOSITION,"inline; filename=\"merged_addons.zip\"",),
    ]);
    Ok((headers,bytes))

}

fn write_zip_to_zip(file:File, target:File) -> std::io::Result<()> {
    let opts = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    let mut writer = ZipWriter::new(target);
    let mut reader = match ZipArchive::new(file) {
        Ok(file) => file,
        Err(e) => return Err(e.into()),
    };
    for i in 0..reader.len() {
        let mut zipped_file = match reader.by_index(i) {
            Ok(file) => file,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        let mut bytes:Vec<u8> = Vec::new();
        if let Err(e) = zipped_file.read_to_end(&mut bytes){
            println!("{e}");
            continue;
        };
        let name = &zipped_file.name()[1..]; //generally starts with /
        if let Ok(_) = writer.start_file(name, opts){
            if let Err(e) = writer.write(bytes.as_slice()) {
                println!("{e}");
            }
        };
    }
    match writer.finish() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into())
    }
}

async fn process_res(
    Path(path): Path<String>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode,String)>{
    let name = format!("created_files/{}/{}.zip", path, Uuid::new_v4());
    if let Err(e) = fs::create_dir_all(format!("created_files/{}", path)) {
        return Err(server_err(format!("{:#?}", e)));
    };
    let file = match fs::File::create(name.clone()) {
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
            Err(e) => return Err(server_err(format!("{:#?}", e))),
        };
        /*{
            let split = complete_name.split("/").collect::<Vec<&str>>();
            match split[split.len() - 1] {
                "manifest.json" => {}
                "item_texture.json" => {}
                "terrain_texture.json" => {}
                _ => {}
            }
        }*/
        let file_name = if let Some(idx) = complete_name.find("/") {
            (&complete_name[idx..]).to_string()
        } else {
            complete_name
        };
        if let Ok(_) = zip.start_file(file_name, opts) {
            if let Err(e) = zip.write(&bytes[..]) {
                return Err(server_err(format!("Could not read file {}", e)));
            }
        }
    }
    match zip.finish() {
        Ok(_) => {
            let path = format!("output_files/{}.zip", path);
            let target_file = OpenOptions::new().write(true).create(true).append(true).open(path).unwrap();
            if let Err(e) = write_zip_to_zip(File::open(name).unwrap(),target_file) {
                println!("{:?}",e);
            };
            Ok("Sucess writing piece to final result")
        }
        Err(e) => Err(server_err(format!("{:#?}", e))),
    }
}

pub fn routes() -> Router {
    Router::new()
        .route(
            "/merge/:id",
            post(process_res).layer(DefaultBodyLimit::max(1 << 32)),
        )
        .route("/finish_merge/:id", post(finish_res))
        .route("/mergedel/:id", get(delete_files))
        .layer(CorsLayer::permissive())
}