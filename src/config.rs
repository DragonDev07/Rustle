use serde::Deserialize;

#[derive(Deserialize)]
/// Configuration structure for the application.
///
/// This structure holds the configuration parameters needed for the application to run.
/// It is derived from the `Deserialize` trait to allow easy loading from configuration files.
///
/// ## Fields
/// - `origin_url`: The URL from which the application will start crawling.
/// - `depth`: The depth to which the application will crawl.
/// - `database_name`: The name of the database to be used by the crawler to store sites.
pub struct Config {
    pub origin_url: String,
    pub depth: u64,
    pub database_name: String,
}
