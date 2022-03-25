use std::io::{self, BufReader, prelude::*, Error, ErrorKind};
use std::fs::{self, File};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use dirs;
use toml;
use crate::common;

const DEFAULT_LOCATION: &'static str = "config.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    ip: String,
    worker_count: usize,
    storage_dir: PathBuf,
}

impl Config {
    fn new(
        ip: Option<String>,
        worker_count: Option<usize>,
        master_dir: Option<PathBuf>,
    ) -> Result<Config, Error> {
        Ok(Config {
            ip: ip.unwrap_or(String::from(Self::defaults().ip.clone())),
            worker_count: worker_count.unwrap_or(Self::defaults().worker_count.clone()),
            storage_dir: master_dir.unwrap_or(Self::defaults().storage_dir.clone()),
        })
    }

    fn defaults() -> Config {
         Config {
            ip: String::from("0.0.0.0:8080"),
            worker_count: 1usize,
            storage_dir: PathBuf::from("{project-dir}/storage")
        }
    }

    pub fn get_ip(&self) -> &String { &self.ip }
    pub fn get_worker_count(&self) -> usize { self.worker_count }
    pub fn get_stroage_dir(&self) -> &PathBuf { &self.storage_dir }

    fn set_ip(&mut self, ip: String) { self.ip = ip }
    fn set_worker_count(&mut self, worker_count: usize) { self.worker_count = worker_count }
    fn set_storage_dir(&mut self, storage_dir: PathBuf) { self.storage_dir = storage_dir }

    pub fn clone(&self) -> Self {
        Config {
            ip: self.get_ip().clone(),
            worker_count: self.get_worker_count(),
            storage_dir: self.get_stroage_dir().clone()
        }
    }
}

pub fn default_config_dir() -> Result<PathBuf, Error> {
    let mut project_dir = common::project_dir()?;
    project_dir.push("config.toml");
    Ok(project_dir)
}

pub fn fetch_config(
    loc: Option<PathBuf>,
    ip: Option<String>,
    worker_count: Option<usize>,
    storage_dir: Option<PathBuf>,
) -> Result<Config, Error> {
    let conf_loc: PathBuf = loc.unwrap_or({
        println!("Configuration file path not provided.");
        println!("Use default file path? ({:?}) (y/n)", default_config_dir()?);

        match parse_yn()? {
            true => default_config_dir()?,
            false => {
                println!("Cannot run server without configuration file.\nExiting.");
                std::process::exit(1)
            },
        }
    });

    let file = File::open(&conf_loc).unwrap_or_else(|err| {

        println!("Error while opening configuration file and parsing contents: {:?}", err);
        println!("Create new configuration file? (y/n)");

        let choice: bool = parse_yn().unwrap_or_else(|err| {
            println!("Failed to read stdin: {:?}.\nExiting.", err);
            std::process::exit(3);
        });

        if !choice {
            println!("Cannot run server without configuration file.\nExiting.");
            std::process::exit(1)
        }

        File::create(conf_loc.clone().into_os_string()).unwrap_or_else(|err| {
            println!("Error while creating configuration file: {:?}\nExiting.", err);
            std::process::exit(4);
        })
    });

    let mut buf_reader = BufReader::new(&file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents).unwrap_or_else(|err| {
        println!("Error reading configuration file: {:?}\nExiting.", err);
        std::process::exit(2);
    });

    let config: Config = if contents.len() == 0 {
        println!("Configuration file is empty, creating new configuration.");
        Config::new(ip, worker_count, storage_dir)?
    } else {
        toml::from_str(&contents)?
    };

    // let config =  Config::new(ip, worker_count, storage_dir).unwrap_or_else(|err| {
    //     println!("Failed to create configuration object: {:?}\nExiting.", err);
    //     std::process::exit(8)
    // });

    // println!("");
    write_config(&conf_loc, &config)?;

    Ok(config)
}

fn write_config(file_loc: &PathBuf, config: &Config) -> Result<(), Error> {
    let toml_as_str: String = toml::to_string(config).map_err(|err| {
        Error::new(
            ErrorKind::Other,
            "Could not serialize config: {:?}".replace("{:?}", &err.to_string()[..])
        )
    })?;

    let mut file = File::create(file_loc)?;

    // println!("{:?}", toml_as_str);

    let bytes: &[u8] = (toml_as_str).as_bytes();
    file.write(bytes)?;

    Ok(())
}

fn parse_yn() -> Result<bool, Error> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;
    let c = &input[0..1];
    Ok(c == "y" || c == "Y")
}
