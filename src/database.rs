use log::trace;
use sqlite::ConnectionThreadSafe;
use std::str::FromStr;
use uuid::Uuid;

pub struct Database {
    conn: ConnectionThreadSafe,
}

impl Database {
    pub fn new(db_name: &str) -> Self {
        let conn = sqlite::Connection::open_thread_safe(format!("{}.db", db_name)).unwrap();
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

    pub fn get_all_site_uuids(&self) -> Vec<Uuid> {
        let mut uuids = Vec::new();
        let query = "SELECT id FROM sites";
        let mut statement = self.prepare(query).unwrap();

        while let sqlite::State::Row = statement.next().unwrap() {
            let id_str = statement.read::<String, usize>(0).unwrap();
            if let Ok(uuid) = Uuid::from_str(&id_str) {
                uuids.push(uuid);
            }
        }

        return uuids;
    }
}
