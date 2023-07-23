extern crate diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
pub mod models;
pub mod schema;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use schematic::Config;

pub struct AppState {
    pub config: Mutex<AppConfig>,
}

#[derive(Config, Serialize, Deserialize, Debug)]
pub struct AppConfig {
    #[setting(default = "log_files.db")]
    #[serde(rename(serialize = "logPath"))]
    pub log_path: String,
}

pub fn establish_connection(config: &AppConfig) -> SqliteConnection {
    SqliteConnection::establish(&config.log_path)
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}
