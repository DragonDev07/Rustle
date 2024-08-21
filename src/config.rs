use directories::BaseDirs;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
/// Configuration structure for the application.
///
/// This structure holds the configuration parameters needed for the application to run.
/// It is derived from the `Deserialize` trait to allow easy loading from configuration files.
pub struct Config {
    /// The URL from which the application will start crawling.
    pub origin_url: String,
    /// The depth to which the application will crawl.
    pub depth: u64,
    /// The name of the database to be used by the crawler to store sites.
    pub database_name: String,
}

impl Config {
    /// Creates a new instance of the configuration by reading from the config file.
    ///
    /// ## Panics
    ///
    /// This function will panic if the configuration file does not exist or cannot be read.
    ///
    /// ## Returns
    ///
    /// A new `Config` instance containing data from `cargo.toml`
    pub fn new() -> Self {
        let base_dirs = BaseDirs::new().unwrap();
        let config_path = format!("{}/Rustle/config.toml", base_dirs.config_dir().display());
        let config: Self = toml::from_str(&fs::read_to_string(&config_path).expect(&format!(
            "No config file! Please create config at {}",
            config_path
        )))
        .unwrap();

        return config;
    }
}
