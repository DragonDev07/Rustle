use log::info;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::time::Instant;
use url::Url;
extern crate pretty_env_logger;

mod spider;

/// The base URL to start web crawling from.
///
/// This constant defines the origin URL where the web crawler begins its operation.
/// All links extracted and processed will be relative to this origin URL.
///
/// ## Example
///
/// ```
/// pub const ORIGIN_URL: &str = "https://wikipedia.org";
/// ```
pub const ORIGIN_URL: &str = "https://wikipedia.org";

/// The main function that initializes the web crawler.
///
/// This function performs the following steps:
/// 1. Declares a reqwest blocking client.
/// 2. Fetches the HTML content of the origin URL.
/// 3. Writes the HTML content of the origin URL to a file.
/// 4. Extracts all links from the origin URL's HTML content.
/// 5. Iterates over all extracted links, fetching and processing each link to discover new links.
///
/// The HTML content of each URL is written to a file organized by the URL's path.
fn main() {
    // Start Runtime
    let time = Instant::now();

    // Start Logger
    pretty_env_logger::init();

    // Declare reqwest blocking client
    let reqwest_client = reqwest::blocking::Client::new();

    // Get HTML of origin url
    let html = get_html(&reqwest_client, ORIGIN_URL);
    write_html("", &html);

    // Get all links from the origin url
    let urls = get_links(&html);

    // Iterate over all links until none are left
    iterate_links(&urls, &reqwest_client);

    info!("Runtime: {}", time.elapsed().as_secs());
}
