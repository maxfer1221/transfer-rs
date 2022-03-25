use std::{fs::File, path::PathBuf, collections::HashMap, io::{BufReader, prelude::*, Error}};
use toml;
use serde::{Serialize, Deserialize};
use serde_json;
use tiny_http::{Request, Header, Method, StatusCode};
use crate::common::{ResponseType, HandlerError, open_file};
use crate::config::Config;

pub fn handle(
    request: &Request,
    route: Vec<&str>,
    params: HashMap<&str, &str>,
    config: &Config
) -> Result<ResponseType, HandlerError> {
    match request.method() {
        Method::Get => Ok(get(request, route, params, config)?),
        _ => Err(HandlerError::new(StatusCode::from(404u16), String::from("Path not supported")))
    }
}

fn get(
    request: &Request,
    route: Vec<&str>,
    params: HashMap<&str, &str>,
    config: &Config
) -> Result<ResponseType, HandlerError> {


    println!("{:?}, {:?}", params, route);
    // if get_content_type(&request) == "application/json" {
    //     let mut content = String::new();
    //     request.as_reader().read_to_string(&mut content).unwrap();
    //     let json: Json = content.parse().unwrap();
    // }

    // let chunk = data::get_latest(request)?;

    let headers = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();

    // Ok(ResponseType::from_string_full(
    //     StatusCode::from(200u16),
    //     Some(vec![headers]),
    //     serde_json::json!(chunk).to_string(),
    // ))

    Ok(ResponseType::not_implemented())
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
