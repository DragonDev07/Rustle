use log::trace;
use sqlite::ConnectionThreadSafe;

/// Represents a database connection.
///
/// This struct encapsulates a thread-safe connection to the database,
/// allowing for safe concurrent access to the database.
///
/// ## Fields
///
/// * `conn` - A thread-safe connection to the database.
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
                url TEXT PRIMARY KEY,
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
}
