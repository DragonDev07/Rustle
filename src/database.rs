use log::trace;
use rusqlite::Connection;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_name: &str) -> Self {
        let conn = Connection::open(format!("{}.db", db_name)).unwrap();
        return Database { conn };
    }

    pub fn setup(&self) {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS sites (
                    id TEXT PRIMARY KEY,
                    url TEXT NOT NULL,
                    crawl_time TEXT NOT NULL,
                    links_to TEXT
                )",
                [],
            )
            .unwrap();

        // TODO: Check all columns exist (eg. useful for if a different database is imported)
    }

    pub fn prepare(&self, statement: &str) -> rusqlite::Result<rusqlite::Statement<'_>> {
        trace!("Preparing SQL statement '{statement}'.");

        self.conn.prepare(statement)
    }

    // TODO: Function to get all site Uuids
    // TODO: Function to read site by id
    // TODO: Function to write site
}

// TODO: Function to summarize whole database
