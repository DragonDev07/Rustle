use anyhow::{Context, Result};
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
    /// Creates a new `Database` instance with a thread-safe connection.
    ///
    /// This function opens a thread-safe connection to the specified database file.
    /// If the database file does not exist, it will be created.
    ///
    /// # Arguments
    ///
    /// * `db_name` - A string slice that holds the name of the database file (without the `.db` extension).
    ///
    /// # Returns
    ///
    /// A `Result` containing a new `Database` instance with an open connection to the specified database,
    /// or an error if the connection fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to open a thread-safe connection to the database.
    pub fn new(db_name: &str) -> Result<Self> {
        let conn =
            sqlite::Connection::open_thread_safe(format!("{}.db", db_name)).context(format!(
                "Failed to open thread-safe connection to the database: {}.db",
                db_name
            ))?;
        info!("Opened database connection to '{}'.db'", db_name);
        return Ok(Database { conn });
    }

    /// Initializes the SQLite tables for storing site and domain data.
    ///
    /// This function creates two tables in the database if they do not already exist:
    /// - `sites`: Stores site data with columns:
    ///   - `url`: The primary key, a text field that stores the URL of the site.
    ///   - `crawl_time`: A text field that stores the crawl time of the site.
    ///   - `links_to`: A text field that stores the URLs that the site links to, as a comma-separated string.
    /// - `domains`: Stores domain data with columns:
    ///   - `domain`: The primary key, a text field that stores the domain name.
    ///   - `crawl_time`: A text field that stores the crawl time of the domain.
    ///   - `robots`: A text field that stores the robots.txt content of the domain.
    ///
    /// This function logs trace messages indicating the progress of the table setup.    
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
            .context("Failed to setup SQLite table 'sites'");

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
            .context("Failed to setup SQLite table 'domains'");
    }

    /// Prepares an SQLite statement for execution.
    ///
    /// This function takes a raw SQL statement as input and prepares it for execution
    /// against the database. It logs the statement being prepared for tracing purposes.
    ///
    /// # Arguments
    ///
    /// * `statement` - A string slice that holds the raw SQL statement to be prepared.
    ///
    /// # Returns
    ///
    /// A `Result` containing a prepared `sqlite::Statement` if successful, or an `sqlite::Error` if an error occurs.
    ///
    /// # Errors
    ///
    /// This function will return an error if the SQL statement fails to prepare.
    ///
    /// # Examples
    ///
    /// ```
    /// let db = Database::new("example").unwrap();
    /// let stmt = db.prepare("SELECT * FROM test").unwrap();
    /// ```   
    pub fn prepare(&self, statement: &str) -> Result<sqlite::Statement<'_>> {
        trace!("Preparing SQLite Statement: '{}'", statement);

        return self
            .conn
            .prepare(statement)
            .context("Failed to prepare SQLite statement");
    }

    /// Executes a raw SQL statement against the database.
    ///
    /// This function takes a raw SQL statement as input and executes it against the database.
    /// It logs the statement being executed for tracing purposes.
    ///
    /// # Arguments
    ///
    /// * `statement` - A string slice that holds the raw SQL statement to be executed.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the execution. Returns `Ok(())` if successful,
    /// or an `sqlite::Error` if an error occurs.
    ///
    /// # Errors
    ///
    /// This function will return an error if the SQL statement fails to execute.
    ///
    /// # Examples
    ///
    /// ```
    /// let db = Database::new("example").unwrap();
    /// db.execute("CREATE TABLE test (id INTEGER PRIMARY KEY)").unwrap();
    /// ```
    pub fn execute(&self, statement: &str) -> Result<()> {
        trace!("Executing SQLite Statement: '{}'", statement);

        return self
            .conn
            .execute(statement)
            .context("Failed to execute SQLite statement");
    }
}
