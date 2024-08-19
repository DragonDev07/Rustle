use crate::database::Database;
use chrono::prelude::*;
use std::collections::HashSet;

/// Represents a website with its URL, crawl time, and links to other sites.
///
/// This struct is used to store information about a website, including its URL,
/// the time it was crawled, and the URLs it links to.
///
/// ## Fields
///
/// * `url` - A string that holds the URL of the site.
/// * `crawl_time` - A `DateTime<Utc>` that represents the time the site was crawled.
/// * `links_to` - A `HashSet<String>` containing the URLs that the site links to.
pub struct Site {
    pub url: String,
    pub crawl_time: DateTime<Utc>,
    pub links_to: HashSet<String>,
}

/// Implements the `Display` trait for the `Site` struct.
///
/// This allows a `Site` instance to be formatted as a string using the `{}` marker.
/// The formatted string will display the URL of the site and the number of links it contains.
impl std::fmt::Display for Site {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{} ({})", self.url, self.links_to.len())
    }
}

impl Site {
    /// Reads a `Site` from the database based on the given URL.
    ///
    /// This function queries the database for a site with the specified URL.
    /// If a matching site is found, it constructs a `Site` instance with the retrieved data.
    ///
    /// ## Arguments
    ///
    /// * `url` - A string slice that holds the URL of the site to be read.
    /// * `database` - A reference to the `Database` from which the site will be read.
    ///
    /// # Returns
    ///
    /// An `Option<Self>` which is `Some(Site)` if a matching site is found, or `None` if no match is found.
    pub fn read_into(url: &str, database: &Database) -> Option<Self> {
        let query = format!(
            "SELECT crawl_time, links_to FROM sites WHERE url = '{}'",
            url
        );

        let mut statement = database.prepare(&query).unwrap();

        while let sqlite::State::Row = statement.next().unwrap() {
            let url: String = statement.read::<String, usize>(0).unwrap();
            let crawl_time_str: String = statement.read::<String, usize>(1).unwrap();
            let links_to_str: String = statement.read::<String, usize>(2).unwrap();

            let crawl_time = DateTime::parse_from_rfc3339(&crawl_time_str)
                .unwrap()
                .with_timezone(&Utc);

            let links_to = if links_to_str.is_empty() {
                HashSet::new()
            } else {
                links_to_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            };

            return Some(Self {
                url: url.to_string(),
                crawl_time,
                links_to,
            });
        }

        return None;
    }

    /// Writes the `Site` instance into the database.
    ///
    /// This function converts the `links_to` field into a comma-separated string,
    /// formats the `crawl_time` field into an RFC 3339 string, and then inserts or
    /// replaces the site record in the database with the current `Site` instance's data.
    ///
    /// ## Arguments
    ///
    /// * `database` - A reference to the `Database` where the site will be written.
    pub fn write_into(&self, database: &Database) {
        let links_to_str = self
            .links_to
            .iter()
            .cloned()
            .collect::<Vec<String>>()
            .join(",");

        let crawl_time_str = self.crawl_time.to_rfc3339();

        let query = format!(
            "INSERT OR REPLACE INTO sites (url, crawl_time, links_to) VALUES ('{}', '{}', '{}')",
            self.url, crawl_time_str, links_to_str
        );

        database.execute(&query).unwrap();
    }
}
