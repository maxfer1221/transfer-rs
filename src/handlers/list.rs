use std::{fs::File, path::PathBuf, io::{BufReader, prelude::*, Error}};
use toml;
use serde::{Serialize, Deserialize};
use serde_json;
use tiny_http::{Request, Header, Method, StatusCode};
use crate::common::{ResponseType, HandlerError, open_file};
use crate::config::Config;

#[derive(Serialize, Deserialize)]
struct Manifest {
    video_count: usize,
    videos: Vec<String>
}

impl Manifest {
    fn from_file(file: &File) -> Result<Self, Error> {
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents)?;

        Ok(toml::from_str(&contents)?)
    }
}

pub fn handle(request: &Request, path: Vec<&str>, config: &Config) -> Result<ResponseType, HandlerError> {
    match request.method() {
        Method::Get => Ok(get(request, path, config)?),
        _ => Err(HandlerError::new(StatusCode::from(404u16), String::from("Path not supported")))
    }
}

fn get(request: &Request, path: Vec<&str>, config: &Config) -> Result<ResponseType, HandlerError> {
    let mut path = PathBuf::from(config.get_stroage_dir());
    path.push("manifest.toml");

    let manifest_file = match open_file(path, 'r') {
        Ok(file) => file,
        Err(err) => return Err(HandlerError::new(
            StatusCode::from(500u16), format!("Internal error while opening manifest file: {}", err)
        ))
    };

    let manifest = match Manifest::from_file(&manifest_file) {
        Ok(manifest) => manifest,
        Err(err) => return Err(HandlerError::new(
            StatusCode::from(500u16), format!("Internal error while parsing manifest file: {}", err)
        ))
    };

    let headers = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();

    Ok(ResponseType::from_string_full(
        StatusCode::from(200u16),
        Some(vec![headers]),
        serde_json::json!(manifest).to_string(),
    ))
}

// pub fn file(name: Vec<&str>, code: u16, content_type: &str) -> std::io::Result<ResponseType> {
//     println!("{:?}", name);
//     let mut response = Response::from_file(get_file(name)?);
//     response = response.with_status_code(code);
//     response.add_header(Header::from_bytes(
//         &b"Content-Type"[..], content_type.as_bytes()
//     ).unwrap());
//     Ok(ResponseType::from_file(response))
// }
