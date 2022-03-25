use tiny_http::{Server};
use std::{env, thread, sync::Arc, collections::HashMap};
use std::path::PathBuf;
use clap::{Arg, App, SubCommand};

mod common;
pub mod handlers;
mod config;

fn main() {
    let matches =
        App::new("transfer-rs")
            .version("0.0.1")
            .author("Maximo Fernandez. <maxfer1221@gmail.com>")
            .about("File Transfer Server")
            .arg(Arg::with_name("config-path")
                .short("cp")
                .long("config-path")
                .value_name("PATH")
                .help("Sets a custom config file path (default: \"{project-dir}/config.toml\")")
                .takes_value(true))
            .arg(Arg::with_name("ip")
                .short("i")
                .long("ip")
                .help("Sets the ip for the server (default: \"0.0.0.0:8080\")")
                .takes_value(true))
            .arg(Arg::with_name("worker-count")
                .short("w")
                .long("worker-count")
                .help("Sets the worker (thread) count for the server (default: 1)")
                .takes_value(true))
            .arg(Arg::with_name("storage-directory")
                .short("md")
                .long("storage-directory")
                .help("Sets the storage directory (default: \"{project-dir}/storage/\")")
                .takes_value(true))
            .get_matches();

    let config: config::Config = config::fetch_config(
        matches.value_of("config-path").map(|s| { PathBuf::from(s) }),
        matches.value_of("ip").map(|s| { String::from(s) }),
        matches.value_of("worker-count").map(|s| { s.parse::<usize>().unwrap_or(1) }),
        matches.value_of("master-directory").map(|s| { PathBuf::from(s) }),
    ).unwrap_or_else(|err| {
        println!("Could not fetch config: {:?}\nExiting.", err);
        std::process::exit(7)
    });

    let server = Server::http(config.get_ip()).unwrap();
    let server = Arc::new(server);

    let HANDLERS: HashMap<&'static str, Arc<handlers::Func>> = handlers::init_handlers();
    let HANDLERS = Arc::new(HANDLERS);

    let mut workers = Vec::with_capacity(config.get_worker_count());

    println!("Server listening on ip:\t\tWorker Count:\n{}\t\t\t{}\n", config.get_ip(), config.get_worker_count());

        for _ in 0..config.get_worker_count() {
            let server = server.clone();
            let config = config.clone();
            let HANDLERS = HANDLERS.clone();

            workers.push(thread::spawn(move || {
                for mut request in server.incoming_requests() {
                    println!("Incoming Request: {:?}", request);

                    let s = request.url().to_string();
                    let route  = s.split('?').collect::<Vec<&str>>()[0]
                                  .split('/').collect::<Vec<&str>>()[1..].to_vec();

                    let params_as_vec = s.split('?').collect::<Vec<&str>>();
                    let params_as_str = params_as_vec.get(1).unwrap_or(&"");
                    let params = params_as_str.split(&['&', '=']).step_by(2)
                        .zip(params_as_str.split(&['&', '=']).skip(1).step_by(2))
                        .fold(HashMap::new(), |mut h, (a, b)| { h.insert(a, b); h });

                    let response = handlers::handle_request(&mut request, route, params, &HANDLERS, &config)
                        .unwrap_or_else(|err| { common::ResponseType::from_string_full(
                            err.code,
                            Some(vec![tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap()]),
                            err.message
                        ) });

                    match response {
                        common::ResponseType::File(r) => request.respond(r).unwrap(),
                        common::ResponseType::Cursor(r) => request.respond(r).unwrap(),
                    }


                    // ...
                }
            }));
        }

    for worker in workers {
        worker.join().unwrap();
    }
}
