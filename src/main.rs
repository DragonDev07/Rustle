use log::info;
use std::time::Instant;
extern crate pretty_env_logger;

mod config;
mod database;
mod domain;
mod error;
mod site;
mod spider;

/// The main entry point of the Rustle application.
///
/// This function initializes the runtime timer, sets up the logger,
/// creates a new instance of the `Crawler` struct, and starts the crawling process.
fn main() {
    // Get Config Values
    info!("Getting config values");
    let config = config::Config::new();

    // Start Runtime & Init Logger
    info!("Initializing rustle webcrawler");
    let runtime = Instant::now();
    pretty_env_logger::init();

    // Declare Crawler
    let crawler = spider::Crawler::new(config.origin_url, config.depth, &config.database_name);

    // Run Crawler
    crawler.crawl();

    // Print Runtime
    info!("Runtime: {}s", runtime.elapsed().as_secs());
}
