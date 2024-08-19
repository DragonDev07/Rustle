use log::trace;
use rusqlite::Connection;

pub struct Database {
    conn: Connection,
}
