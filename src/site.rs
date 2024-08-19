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

impl Site {}
