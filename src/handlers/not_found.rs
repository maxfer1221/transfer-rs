use std::collections::HashMap;
use tiny_http::{Request, Response, Header};
use crate::common::{ResponseType, HandlerError};
use crate::config::Config;

pub fn handle(
    request: &Request,
    route: Vec<&str>,
    params: HashMap<&str, &str>,
    config: &Config
) -> Result<ResponseType, HandlerError> {
    Ok(ResponseType::from_string(String::from("not found")))
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
