use crate::database::Database;
use chrono::{DateTime, Utc};
use log::info;

pub struct Domain {
    pub domain: String,
    pub crawl_time: DateTime<Utc>,
    pub robots: String,
}

impl Domain {
    /// Reads a `Domain` from the database based on the given domain.
    ///
    /// This function queries the database for a site with the specified domain.
    /// If a matching site is found, it constructs a `Domain` instance with the retrieved data.
    ///
    /// ## Arguments
    ///
    /// * `domain` - A string slice that holds the domain of the site to be read.
    /// * `database` - A reference to the `Database` from which the site will be read.
    ///
    /// ## Returns
    ///
    /// An `Option<Self>` which is `Some(Domain)` if a matching domain is found, or `None` if no match is found.
    pub fn read_into(domain: &str, database: &Database) -> Option<Self> {
        let query = format!(
            "SELECT crawl_time, robots FROM domains WHERE domain = '{}'",
            domain
        );

        let mut statement = database.prepare(&query).unwrap();

        while let sqlite::State::Row = statement.next().unwrap() {
            let crawl_time_str: String = statement.read::<String, usize>(0).unwrap();
            let robots: String = statement
                .read::<String, usize>(1)
                .unwrap()
                .replace("''", "'");

            let crawl_time = DateTime::parse_from_rfc3339(&crawl_time_str)
                .unwrap()
                .with_timezone(&Utc);

            return Some(Self {
                domain: domain.to_string(),
                crawl_time,
                robots,
            });
        }

        return None;
    }

    /// Writes the `Domain` instance into the database.
    ///
    /// This function formats the `crawl_time` field into an RFC 3339 string, and then inserts or
    /// replaces the domain record in the database with the current `Domain` instance's data.
    ///
    /// ## Arguments
    ///
    /// * `database` - A reference to the `Database` where the site will be written.
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
    pub fn summarize_domain_table(database: &Database) {
        let query = "SELECT COUNT(*) FROM domains";
        let mut statement = database.prepare(query).unwrap();
        let _ = statement.next();

        let count = statement.read::<i64, usize>(0).unwrap();

        info!("{} Entries in domain table", count);
    }
}
