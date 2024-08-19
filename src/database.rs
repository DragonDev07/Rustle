use log::{debug, trace};
use sqlite::Connection;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_name: &str) -> Self {
        let conn = sqlite::open(format!("{}.db", db_name)).unwrap();
        return Database { conn };
    }

    pub fn setup(&self) {
        trace!("Setting Up SQLite Table");

        self.conn
            .execute(
                r#"
              CREATE TABLE IF NOT EXISTS sites (
                id TEXT PRIMARY KEY,
                url TEXT NOT NULL,
                crawl_time TEXT NOT NULL,
                links_to TEXT
              );"#,
            )
            .unwrap();
    }

    pub fn prepare(&self, statement: &str) -> sqlite::Result<sqlite::Statement<'_>> {
        trace!("Preparing SQLite Statement: '{}'", statement);

        return self.conn.prepare(statement);
    }

    pub fn execute(&self, statement: &str) -> sqlite::Result<()> {
        trace!("Executing SQLite Statement: '{}'", statement);

        return self.conn.execute(statement);
    }

    // TODO: Function to retrieve all site Uuids
    // TODO: Function to read `Site` by id
    // TODO: Function to write `Site`
}
