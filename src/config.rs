use anyhow::{Context, Result};
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
    /// Creates a new `Config` instance by reading from the configuration file.
    ///
    /// This function reads the configuration file located at `config.toml` and parses its contents
    /// into a `Config` struct.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new `Config` instance with data from the configuration file,
    /// or an error if the file cannot be read or parsed.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to read or parse the configuration file.
    ///
    /// # Panics
    ///
    /// This function will panic if the base directories cannot be determined.    
    pub fn new() -> Result<Self> {
        let base_dirs = BaseDirs::new().context("Failed to get base directories")?;
        let config_path = format!("{}/Rustle/config.toml", base_dirs.config_dir().display());
        let config_str = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {}", config_path))?;
        let config: Self = toml::from_str(&config_str)
            .with_context(|| format!("Failed to parse config file at {}", config_path))?;

        return Ok(config);
    }
}
