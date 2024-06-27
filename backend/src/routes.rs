use std::{fs, io::{Read,Write}};
use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart, Path},
    http::{header,StatusCode},
    response::{AppendHeaders,IntoResponse},
    routing::post,
    Router,
};
use tower_http::cors::CorsLayer;
use uuid::Uuid;
use zip::{ZipWriter,write::SimpleFileOptions};

pub fn create_zip_from_path(path:String) -> std::io::Result<ZipWriter<std::fs::File>> {
    match fs::File::create(path) {
        Ok(file) => Ok(ZipWriter::new(file)),
        Err(e) => Err(e)
    }
}

pub fn server_err<S: Into<String>>(err: S) -> (StatusCode, String) {
    (StatusCode::OK, err.into())
}

async fn finish_res(Path(path):Path<String>) -> Result<impl IntoResponse, (StatusCode,String)> {
    let output_path = format!("output_files/{}.zip",path.clone());
    let dir_path = format!("created_files/{}",path);
    
    let files = match fs::read_dir(dir_path.clone()) {
        Err(e) => return Err(server_err(format!("{:#?}. Didnt find path created_files/{}",e, path))),
        Ok(dir) => dir,
    };
    let mut output = match create_zip_from_path(output_path.clone()) {
        Err(e) => return Err(server_err(format!("{e}"))),
        Ok(zip) => zip
    };
    let opts = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for file in files {
        if let Ok(file) = file {
            let path = file.path();
            let opened = match std::fs::File::open(&path) {
                Ok(file) => file,
                Err(e) => {
                    return Err(server_err(format!("{e} could not read file")));
                }
            };
            let mut reader = match zip::ZipArchive::new(opened) {
                Ok(zip) => zip,
                Err(_) => continue
            };
            for i in 0..reader.len(){
                let mut inzip_file = match reader.by_index(i){
                    Ok(file) => file,
                    Err(_) => continue,
                };
                let mut bytes:Vec<u8> = Vec::new();
                let _ = inzip_file.read_to_end(&mut bytes);
                let name = {
                    let temp = inzip_file.name();
                    if let Some(idx) = temp.find("/") {
                        &temp[idx..]
                    }else{
                        temp
                    }
                };
                if let Ok(_) = output.start_file(name, opts){
                    if let Err(e) = output.write(&bytes[..]){
                        return Err(server_err(format!("Error on write file {}", e)));
                    }
                };
            }
        }
    }
    match output.finish() {
        Ok(_) => {
            let content = match fs::read(output_path.clone()){
                Ok(content) => content,
                Err(_) => return Err(server_err("internal error: not possible to read file"))
            };
            let bytes = Bytes::from(content);
            let headers = AppendHeaders([
                (header::CONTENT_TYPE, "application/zip"),
                (
                    header::CONTENT_DISPOSITION,
                    "inline; filename=\"merged_addons.zip\"",
                ),
            ]);
            fs::remove_dir_all(dir_path).unwrap();
            fs::remove_file(output_path).unwrap();
            Ok((headers,bytes))

        }
        Err(e) => {
            Err(server_err(format!("{e}")))
        }
        
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
    let mut zip = zip::ZipWriter::new(file);
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
        Ok(_) => Ok(format!("Success on creating file: {}", name)),
        Err(e) => Err(server_err(format!("{:#?}", e))),
    }
}

pub fn routes() -> Router {
    Router::new()
        .route(
            "/merge/:id",
            post(process_res).layer(DefaultBodyLimit::max(1 << 24)),
        )
        .route("/finish_merge/:id", post(finish_res))
        .layer(CorsLayer::permissive())
}