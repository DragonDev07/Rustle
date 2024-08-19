use log::info;
use std::time::Instant;
extern crate pretty_env_logger;

mod database;
mod site;
mod spider;

/// The main entry point of the Rustle application.
///
/// This function initializes the runtime timer, sets up the logger,
/// creates a new instance of the `Crawler` struct, and starts the crawling process.
fn main() {
    // Start Runtime & Init Logger
    info!("Initializing Rustle Webcrawler");
    let runtime = Instant::now();
    pretty_env_logger::init();

    // Declare Crawler
    let crawler = spider::Crawler::new("https://rolisz.ro".to_string(), 6, "crawler");

    // Run Crawler
    crawler.crawl();

    // Print Runtime
    info!("Runtime: {}s", runtime.elapsed().as_secs());
}
