use crate::error::Errors;
use log::{info, trace};
use sqlite::ConnectionThreadSafe;

/// Represents a database connection.
///
/// This struct encapsulates a thread-safe connection to the database,
/// allowing for safe concurrent access to the database.
pub struct Database {
    /// A thread-safe connection to the database.
    conn: ConnectionThreadSafe,
}

impl Database {
    /// Creates a new `Database` instance.
    ///
    /// This function opens a thread-safe connection to the specified database file.
    /// If the database file does not exist, it will be created.
    ///
    /// ## Arguments
    ///
    /// * `db_name` - A string slice that holds the name of the database file (without the `.db` extension).
    ///
    /// ## Returns
    ///
    /// A new `Database` instance with an open connection to the specified database.
    pub fn new(db_name: &str) -> Result<Self, Errors> {
        let conn =
            sqlite::Connection::open_thread_safe(format!("{}.db", db_name)).map_err(|_| {
                Errors::DatabaseError(format!(
                    "Failed to open SQLite thread safe connection to db {}",
                    db_name
                ))
            })?;
        info!("Opened database connection to '{}'.db'", db_name);
        return Ok(Database { conn });
    }

    /// Sets up the SQLite table for storing site data.
    ///
    /// This function creates a table named `sites` in the database if it does not already exist.
    /// The table has the following columns:
    /// - `url`: The primary key, a text field that stores the URL of the site.
    /// - `crawl_time`: A text field that stores the crawl time of the site.
    /// - `links_to`: A text field that stores the URLs that the site links to, as a comma-separated string.
    ///
    /// This function will log a trace message indicating that the table setup is in progress.
    pub fn setup(&self) {
        trace!("Setting up SQLite table 'sites'");
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

        trace!("Setting up SQLite table 'domains'");
        self.conn
            .execute(
                r#"
                CREATE TABLE IF NOT EXISTS domains (
                    domain TEXT PRIMARY KEY,
                    crawl_time TEXT NOT NULL,
                    robots TEXT
                );"#,
            )
            .unwrap();
    }

    /// Prepares an SQLite statement for execution.
    ///
    /// This function takes a raw SQL statement as input and prepares it for execution
    /// against the database. It logs the statement being prepared for tracing purposes.
    ///
    /// ## Arguments
    ///
    /// * `statement` - A string slice that holds the raw SQL statement to be prepared.
    ///
    /// ## Returns
    ///
    /// A `Result` containing a prepared `sqlite::Statement` if successful, or an `sqlite::Error` if an error occurs.
    pub fn prepare(&self, statement: &str) -> sqlite::Result<sqlite::Statement<'_>> {
        trace!("Preparing SQLite Statement: '{}'", statement);

        return self.conn.prepare(statement);
    }

    /// Executes a raw SQL statement against the database.
    ///
    /// This function takes a raw SQL statement as input and executes it against the database.
    /// It logs the statement being executed for tracing purposes.
    ///
    /// ## Arguments
    ///
    /// * `statement` - A string slice that holds the raw SQL statement to be executed.
    ///
    /// ## Returns
    ///
    /// A `Result` indicating the success or failure of the execution. Returns `Ok(())` if successful,
    /// or an `sqlite::Error` if an error occurs.
    pub fn execute(&self, statement: &str) -> sqlite::Result<()> {
        trace!("Executing SQLite Statement: '{}'", statement);

        return self.conn.execute(statement);
    }
}
