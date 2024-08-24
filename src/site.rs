use crate::database::Database;
use anyhow::{Context, Result};
use chrono::prelude::*;
use log::info;
use std::collections::HashSet;

/// Represents a website with its URL, crawl time, and links to other sites.
///
/// This struct is used to store information about a website, including its URL,
/// the time it was crawled, and the URLs it links to.
pub struct Site {
    /// A string that holds the URL of a given site.
    pub url: String,
    /// A `DateTime<Utc>` that represents the time the site was crawled.
    pub crawl_time: DateTime<Utc>,
    /// A `HashSet<String>` containing the urls that the site links to.
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
    /// Reads a `Site` from the database based on the given domain.
    ///
    /// This function queries the database for a site with the specified URL.
    /// If a matching domain is found, it constructs a `Site` instance with the retrieved data.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice that holds the URL of the site to be read.
    /// * `database` - A reference to the `Database` from which the site will be read.
    ///
    /// # Returns
    ///
    /// A `Result<Option<Self>>` which is `Ok(Some(Site))` if a matching site is found,
    /// or `Ok(None)` if no match is found. If an error occurs during the query or data retrieval,
    /// it returns an `Err`.
    pub fn read_into(url: &str, database: &Database) -> Result<Option<Self>> {
        // Declare SQLite Query to get all entries where the URL value is equal to the given URL
        let query = format!(
            "SELECT crawl_time, links_to FROM sites WHERE url = '{}'",
            url.replace("'", "''")
        );

        // Prepare Query
        let mut statement = database.prepare(&query)?;

        // Iterate over the rows returned by the query (should only be one, but need to return none
        // if no rows are returned)
        while let sqlite::State::Row = statement
            .next()
            .context("Failed to execute the SQL query")?
        {
            // Read the crawl time from the first column of the current row
            let crawl_time_str: String = statement
                .read::<String, usize>(0)
                .context("Failed to read crawl_time from the database")?;

            // Read the links to other sites from the second column of the current row
            let links_to_str: String = statement
                .read::<String, usize>(1)
                .context("Failed to read links_to from the database")?;

            // Parse the crawl time string into a DateTime<Utc> object
            let crawl_time = DateTime::parse_from_rfc3339(&crawl_time_str)
                .context("Failed to parse crawl_time as RFC 3339")?
                .with_timezone(&Utc);

            // Split the links_to string by commas and collect them into a HashSet
            let links_to = if links_to_str.is_empty() {
                HashSet::new()
            } else {
                links_to_str
                    .replace("''", "'")
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            };

            // Return a `Site` instance with the retrieved data
            return Ok(Some(Self {
                url: url.to_string().replace("''", "'"),
                crawl_time,
                links_to,
            }));
        }

        // If no rows are retrieved by the query, return None
        return Ok(None);
    }

    /// Writes the `Site` instance into the database.
    ///
    /// This function converts the `links_to` field into a comma-separated string,
    /// formats the `crawl_time` field into an RFC 3339 string, and then inserts or
    /// replaces the site record in the database with the current `Site` instance's data.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the `Database` where the site will be written.
    pub fn write_into(&self, database: &Database) {
        // Convert links_to HashSet into comma-separated string
        let links_to_str = self
            .links_to
            .iter()
            .cloned()
            .collect::<Vec<String>>()
            .join(",");

        // Convert crawl_time to RFC 3339 string
        let crawl_time_str = self.crawl_time.to_rfc3339();

        // Declare SQLite query
        let query =
            format!(
            "INSERT OR REPLACE INTO sites (url, crawl_time, links_to) VALUES ('{}', '{}', '{}')",
            self.url.replace("'", "''"), crawl_time_str, links_to_str.replace("'", "''")
        );

        // Execute query
        database.execute(&query).unwrap();
    }

    /// Summarizes the database by counting the number of entries in the `sites` table.
    ///
    /// This function prepares and executes a SQL query to count the number of entries
    /// in the `sites` table and logs the result using the `info` log level.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the `Database` where the domain will be summarized.
    ///
    /// # Returns
    ///
    /// A `Result<()>` which is `Ok(())` if the operation is successful, or an `Err` if an error occurs.
    pub fn summarize_site_table(database: &Database) -> Result<()> {
        let query = "SELECT COUNT(*) FROM sites";
        let mut statement = database.prepare(query).unwrap();
        let _ = statement
            .next()
            .context("Failed to execute the SQL query")?;

        let count = statement
            .read::<i64, usize>(0)
            .context("Failed to read the count from the database")?;

        info!("{} Entries in site table", count);
        return Ok(());
    }
}
