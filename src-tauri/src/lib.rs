extern crate diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("log_files.db")
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}
