use std::{fs::{self, File}, path::PathBuf, io::Cursor};
use std::io::{Error, ErrorKind};
use tiny_http::{Request, Response, StatusCode, Header};

pub struct ContentType {
    pub html: &'static str,
    pub css: &'static str,
    pub js: &'static str,
}

pub static CONTENT_TYPES: ContentType = ContentType {
    html: "text/html; charset=UTF-8",
    css: "text/css; charset=UTF-8",
    js: "text/javascript; charset=UTF-8",
};

pub struct HandlerError {
    pub code: StatusCode,
    pub message: String,
}

impl HandlerError {
    pub fn new(code: StatusCode, message: String) -> HandlerError {
        HandlerError { code: code, message: message }
    }
}

// static content type object to hold 'content-type' header values
pub enum ResponseType {
    File(Response<File>),
    Cursor(Response<Cursor<Vec<u8>>>)
}

// simple Response -> ResponseType wrapping
impl ResponseType {
    pub fn default() -> Self {
        ResponseType::Cursor(Response::from_string(String::from("Default Response")))
    }
    pub fn not_implemented() -> Self {
        ResponseType::Cursor(Response::from_string(String::from("Not Implemented")))
    }
    pub fn from_file(r: Response<File>) -> Self {
        ResponseType::File(r)
    }
    pub fn from_cursor(r: Response<Cursor<Vec<u8>>>) -> Self {
        ResponseType::Cursor(r)
    }
    pub fn from_string(s: String) -> Self {
        ResponseType::Cursor(Response::from_string(s))
    }

    pub fn from_string_full(
        code: StatusCode,
        headers: Option<Vec<Header>>,
        string: String,
    ) -> Self {
        let res = Response::<Cursor<Vec<u8>>>::new(
            code,
            headers.unwrap_or(vec![]),
            Cursor::new(Vec::from(string.as_bytes())),
            Some(Vec::from(string.as_bytes()).len()),
            None,
        );
        ResponseType::Cursor(res)
    }
}

pub fn project_dir() -> Result<PathBuf, std::io::Error> {
    let mut pd = std::env::current_exe()?;
    for _ in 0..3 { pd.pop(); }
    Ok(pd)
}

pub fn open_file(path: PathBuf, method: char) -> Result<File, Error> {
    match method {
        'r' => Ok(File::open(path)?),
        'w' => Ok(File::create(path)?),
        _ => Err(Error::new(ErrorKind::Unsupported, "File open method not supported"))
    }
}

// pub fn get_content_type(request: &Request) -> Option<&Header> {
//     let ct_hf = tiny_http::HeaderField::from_bytes(&b"Content-Type"[..]).unwrap();
//     Vec::from(request.headers()).iter().find(|el| { el.field == ct_hf })
// }
