use log::info;
use std::time::Instant;
extern crate pretty_env_logger;

mod spider;

/// The main entry point of the Rustle application.
///
/// This function initializes the runtime timer, sets up the logger,
/// creates a new instance of the `Crawler` struct, and starts the crawling process.
fn main() {
    // Start Runtime
    let time = Instant::now();

    // Start Logger
    pretty_env_logger::init();

    // Declare Crawler
    let crawler = spider::Crawler::new("https://rolisz.ro".to_string(), 5);

    // Run Crawler
    crawler.crawl();

    info!("Runtime: {}", time.elapsed().as_secs());
}
