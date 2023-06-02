use serde::Deserialize;
use std::fs;
use std::process::exit;

/// Structure representing the server configuration.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Server settings.
    pub server: ServerSettings,
    /// Server routes.
    pub routes: Vec<Route>,
}

/// Server settings.
#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    /// Server IP address.
    pub ip: String,
    /// Server port.
    pub port: u32,
    /// Server base directory.
    pub base_dir: String,
}

/// Server route.
#[derive(Debug, Deserialize)]
pub struct Route {
    /// Request path.
    pub path: String,
    /// File associated with the route.
    pub file: String,
}

/// Read and parse the configuration file in TOML format.
///
/// # Example
///
/// ```
/// let config = read_cfg();
/// println!("{:?}", config);
/// ```
///
/// # Panics
///
/// If an error occurs while reading or parsing the configuration file, an error message is printed
/// and the program exits with a return code of 1.
///
/// # Returns
///
/// The server configuration read and parsed from the file.
pub fn read_cfg() -> Config {
    let config_file = "./ruzky.toml";

    let config_content = match fs::read_to_string(config_file) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("File not found: `{}`", config_file);
            exit(1);
        }
    };

    let data: Config = match toml::from_str(&config_content) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Data not found in file: `{}`", config_file);
            exit(1);
        }
    };

    data
}
