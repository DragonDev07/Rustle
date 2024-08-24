use crate::database::Database;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use log::info;

/// Represents a domain that has been crawled.
///
/// This struct holds information about a domain, including the domain name,
/// the time it was crawled, and the contents of its robots.txt file.
pub struct Domain {
    ///  A `String` that holds the domain name.
    pub domain: String,
    /// A `DateTime<Utc>` that represents the time the domain was crawled.
    pub crawl_time: DateTime<Utc>,
    /// A `String` that contains the contents of the domain's robots.txt file.
    pub robots: String,
}

impl Domain {
    /// Reads a `Domain` from the database based on the given domain.
    ///
    /// This function queries the database for a domain with the specified domain name.
    /// If a matching domain is found, it constructs a `Domain` instance with the retrieved data.
    ///
    /// # Arguments
    ///
    /// * `domain` - A string slice that holds the domain of the site to be read.
    /// * `database` - A reference to the `Database` from which the domain will be read.
    ///
    /// # Returns
    ///
    /// A `Result<Option<Self>>` which is `Ok(Some(Domain))` if a matching domain is found,
    /// or `Ok(None)` if no match is found. If an error occurs during the query or data retrieval,
    /// it returns an `Err`.
    pub fn read_into(domain: &str, database: &Database) -> Result<Option<Self>> {
        let query = format!(
            "SELECT crawl_time, robots FROM domains WHERE domain = '{}'",
            domain
        );

        let mut statement = database.prepare(&query)?;

        while let sqlite::State::Row = statement
            .next()
            .context("Failed to execute the SQL query")?
        {
            let crawl_time_str: String = statement
                .read::<String, usize>(0)
                .context("Failed to read crawl_time from the database")?;
            let robots: String = statement
                .read::<String, usize>(1)
                .context("Failed to read robots from the database")?
                .replace("''", "'");

            let crawl_time = DateTime::parse_from_rfc3339(&crawl_time_str)
                .context("Failed to parse crawl_time as RFC 3339")?
                .with_timezone(&Utc);

            return Ok(Some(Self {
                domain: domain.to_string(),
                crawl_time,
                robots,
            }));
        }

        return Ok(None);
    }

    /// Writes the `Domain` instance into the database.
    ///
    /// This function formats the `crawl_time` field into an RFC 3339 string, and then inserts or
    /// replaces the domain record in the database with the current `Domain` instance's data.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the `Database` where the domain will be written.
    pub fn write_into(&self, database: &Database) {
        let crawl_time_str = self.crawl_time.to_rfc3339();

        let query =
            format!(
            "INSERT OR REPLACE INTO domains (domain, crawl_time, robots) VALUES ('{}', '{}', '{}')",
            self.domain, crawl_time_str, self.robots.replace("'", "''")
        );

        database.execute(&query).unwrap();
    }

    /// Summarizes the database by counting the number of entries in the `domains` table.
    ///
    /// This function prepares and executes a SQL query to count the number of entries
    /// in the `domains` table and logs the result using the `info` log level.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the `Database` where the domain will be summarized.
    ///
    /// # Returns
    ///
    /// A `Result<()>` which is `Ok(())` if the operation is successful, or an `Err` if an error occurs.
    pub fn summarize_domain_table(database: &Database) -> Result<()> {
        let query = "SELECT COUNT(*) FROM domains";
        let mut statement = database.prepare(query)?;
        let _ = statement
            .next()
            .context("Failed to execute the SQL query")?;

        let count = statement
            .read::<i64, usize>(0)
            .context("Failed to read the count from the database")?;

        info!("{} Entries in domain table", count);
        return Ok(());
    }
}
