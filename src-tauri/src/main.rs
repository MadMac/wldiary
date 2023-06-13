// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;

use diesel::prelude::*;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};
use log::{debug, info};
use wldiary::establish_connection;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

mod models;
mod schema;

use self::models::DailyLog;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_log_dates() -> Vec<DailyLog> {
    use self::schema::daily_logs::dsl::*;

    let conn = &mut establish_connection();

    debug!("Getting log dates");
    let all_daily_logs: Vec<DailyLog> = daily_logs
        .select(DailyLog::as_select())
        .load(conn)
        .expect("Expected to get all daily logs");

    debug!("{:?}", all_daily_logs);

    return all_daily_logs;
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    info!("Running migrations.");
    let mut conn = SqliteConnection::establish("log_files.db").unwrap();
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    info!("Starting Tauri backend.");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_log_dates])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
