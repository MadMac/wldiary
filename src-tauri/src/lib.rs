extern crate diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
pub mod models;
pub mod schema;
use serde::{Deserialize, Serialize};

use schematic::Config;

#[derive(Config, Serialize, Deserialize, Debug)]
pub struct AppConfig {
    #[setting(default = "log_files.db")]
    #[serde(rename(serialize = "logPath"))]
    pub log_path: String,
}

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("log_files.db")
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}
