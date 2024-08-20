use directories::BaseDirs;
use log::info;
use std::{fs, time::Instant};
extern crate pretty_env_logger;

mod config;
mod database;
mod site;
mod spider;

/// The main entry point of the Rustle application.
///
/// This function initializes the runtime timer, sets up the logger,
/// creates a new instance of the `Crawler` struct, and starts the crawling process.
fn main() {
    // Get Config Values
    info!("Getting Config Values");
    let base_dirs = BaseDirs::new().unwrap();
    let config_path = format!("{}/Rustle/config.toml", base_dirs.config_dir().display());
    let config: config::Config = toml::from_str(&fs::read_to_string(&config_path).expect(
        &format!("No config file! Please create config at {}", config_path),
    ))
    .unwrap();

    // Start Runtime & Init Logger
    info!("Initializing Rustle Webcrawler");
    let runtime = Instant::now();
    pretty_env_logger::init();

    // Declare Crawler
    let crawler = spider::Crawler::new(config.origin_url, config.depth, &config.database_name);

    // Run Crawler
    crawler.crawl();

    // Print Runtime
    info!("Runtime: {}s", runtime.elapsed().as_secs());
}
