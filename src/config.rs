use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub origin_url: String,
    pub depth: u64,
    pub database_name: String,
}
