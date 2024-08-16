use crate::database::Database;
use chrono::prelude::*;
use std::str::FromStr;
use uuid::Uuid;

pub struct Site {
    id: Uuid,
    url: String,
    crawl_time: DateTime<Utc>,
    links_to: Vec<Uuid>,
}

impl std::fmt::Display for Site {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{} ({})", self.url, self.id)
    }
}

impl Site {
    fn read_into(id: Uuid, database: &Database) -> Option<Self> {
        let query = format!(
            "SELECT url, crawl_time, links_to FROM sites WHERE id = '{}'",
            id
        );

        let mut statement = database.prepare(&query).ok()?;
        let mut rows = statement.query([]).ok()?;

        if let Some(row) = rows.next().ok()? {
            let url: String = row.get(0).ok()?;
            let crawl_time: String = row.get(1).ok()?;
            let links_to: String = row.get(2).ok()?;

            let crawl_time = DateTime::from_str(&crawl_time).ok()?;
            let links_to: Vec<Uuid> = links_to
                .split(',')
                .filter_map(|s| Uuid::parse_str(s).ok())
                .collect();

            return Some(Site {
                id,
                url,
                crawl_time,
                links_to,
            });
        }

        return None;
    }

    fn write_into(&self, database: &Database) -> rusqlite::Result<()> {
        let links_to = self.links_to
            .iter()
            .map(|uuid| uuid.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let query = format!(
            "INSERT INTO sites (id, url, crawl_time, links_to) VALUES ('{}', '{}', '{}', '{}') \
             ON CONFLICT(id) DO UPDATE SET url = excluded.url, crawl_time = excluded.crawl_time, links_to = excluded.links_to",
            self.id, self.url, self.crawl_time.to_rfc3339(), links_to
        );

        let mut statement = database.prepare(&query)?;
        statement.execute([])?;

        return Ok(());
    }
}
