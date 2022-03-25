use tiny_http::Request;
use std::sync::Arc;
use std::marker::{Send, Sync};
use std::{collections::HashMap, io::{Error, ErrorKind::NotFound}};
use crate::common::{ResponseType, HandlerError, CONTENT_TYPES as CT};
use crate::config::Config;

// mod list;
mod not_found;
mod root;
mod download;

pub type Func = dyn Sync + Send + Fn(&Request, Vec<&str>, HashMap<&str, &str>, &Config) -> Result<ResponseType, HandlerError>;

pub fn init_handlers() -> HashMap<&'static str, Arc<Func>> {
    let mut map: HashMap<&'static str, Arc<Func>> = HashMap::new();
    // default route
    map.insert("not_found", Arc::new(not_found::handle));
    // root route
    map.insert("", Arc::new(root::handle));

    // others
    map.insert("download", Arc::new(download::handle));
    // map.insert("upload", Box::new(upload::handle));
    // ...
    map
}

// use handlers;
pub fn handle_request(
    r: &mut Request,
    route: Vec<&str>,
    params: HashMap<&str, &str>,
    HANDLERS: &Arc<HashMap<&'static str, Arc<Func>>>,
    config: &Config
) -> Result<ResponseType, HandlerError> {
    HANDLERS.get(&route.join("/")[..])
        .unwrap_or(
            HANDLERS.get("not_found").unwrap()
        )(r, route, params, config)
}

// handler definition
// struct Handler {
//     path: String,
//     handler: Func
// }
//
// impl Handler {
//     fn new(path: String, handler: Func) -> Handler {
//         Handler { path: path, handler: handler }
//     }
// }

// pub fn get_handlers<'a>() -> HashMap<&'a str, Box<Func>> {
//     let map: HashMap<&'a str, Box<Func>> = HashMap::new();
//     map.insert("list", Box::new(list::handle));
//
//     return map;
// }
