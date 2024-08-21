use crate::database::Database;
use chrono::{DateTime, Utc};
use log::info;

pub struct Domain {
    pub domain: String,
    pub crawl_time: DateTime<Utc>,
    pub robots: String,
}

impl Domain {
    pub fn read_into(domain: &str, database: &Database) -> Option<Self> {
        let query = format!(
            "SELECT crawl_time, robots FROM domains WHERE domain = '{}'",
            domain
        );

        let mut statement = database.prepare(&query).unwrap();

        while let sqlite::State::Row = statement.next().unwrap() {
            let crawl_time_str: String = statement.read::<String, usize>(0).unwrap();
            let robots: String = statement.read::<String, usize>(1).unwrap();

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

    pub fn write_into(&self, database: &Database) {
        let crawl_time_str = self.crawl_time.to_rfc3339();

        let query = format!(
            "INSERT OR REPLACE INTO domains (domain, crawl_time, robots) VALUES ('{}', '{}', '{}')",
            self.domain, crawl_time_str, self.robots
        );

        database.execute(&query).unwrap();
    }

    pub fn summarize_domain_table(database: &Database) {
        let query = "SELECT COUNT(*) FROM domains";
        let mut statement = database.prepare(query).unwrap();
        let _ = statement.next();

        let count = statement.read::<i64, usize>(0).unwrap();

        info!("{} Entries in domain table", count);
    }
}
