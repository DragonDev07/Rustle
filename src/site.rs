use crate::database::Database;
use chrono::prelude::*;
use std::str::FromStr;
use uuid::Uuid;

pub struct Site {
    pub id: Uuid,
    pub url: String,
    pub crawl_time: DateTime<Utc>,
    pub links_to: Vec<Uuid>,
}

impl std::fmt::Display for Site {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{} ({})", self.url, self.id)
    }
}

impl Site {
    pub fn read_into(id: Uuid, database: &Database) -> Option<Self> {
        let query = format!(
            "SELECT url, crawl_time, links_to FROM sites WHERE id = '{}'",
            id
        );

        let mut statement = database.prepare(&query).unwrap();

        while let sqlite::State::Row = statement.next().unwrap() {
            let url: String = statement.read::<String, usize>(0).unwrap();
            let crawl_time_str: String = statement.read::<String, usize>(1).unwrap();
            let links_to_str: String = statement.read::<String, usize>(2).unwrap();

            let crawl_time = DateTime::parse_from_rfc3339(&crawl_time_str)
                .unwrap()
                .with_timezone(&Utc);

            let links_to = match links_to_str.is_empty() {
                true => vec![],
                false => links_to_str
                    .split(',')
                    .map(|s| Uuid::from_str(s).unwrap())
                    .collect(),
            };

            return Some(Self {
                id: id.clone(),
                url,
                crawl_time,
                links_to,
            });
        }

        return None;
    }

    pub fn write_into(&self, database: &Database) {
        let links_to_str = self
            .links_to
            .iter()
            .map(|uuid| uuid.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let crawl_time_str = self.crawl_time.to_rfc3339();

        let query = format!(
            "INSERT OR REPLACE INTO sites (id, url, crawl_time, links_to) VALUES ('{}', '{}', '{}', '{}')",
            self.id, self.url, crawl_time_str, links_to_str
        );

        database.execute(&query).unwrap();
    }
}
