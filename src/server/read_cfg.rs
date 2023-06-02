use serde::Deserialize;
use std::fs;
use std::process::exit;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerSettings,
    pub routes: Vec<Route>
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub ip: String,
    pub port: u32
}

#[derive(Debug, Deserialize)]
pub struct Route {
    pub path: String,
    pub file: String
}

pub fn read_cfg() -> Config {
    let config_file = "./ruzky.toml";

    let config_content = match fs::read_to_string(config_file) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Archivo no encontrado `{}`", config_file);
            exit(1);
        }
    };

    let data: Config = match toml::from_str(&config_content) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Datos no encontrado desde `{}`", config_file);
            exit(1);
        }
    };

    return data;
}
