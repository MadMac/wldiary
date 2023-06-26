// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;

use chrono::{DateTime, NaiveDate, Utc};
use diesel::dsl::today;
use diesel::prelude::*;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};
use log::{debug, info};
use uuid::Uuid;
use wldiary::establish_connection;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

mod models;
mod schema;

use self::models::DailyLog;

#[tauri::command]
fn get_log_dates() -> Vec<DailyLog> {
    use self::schema::daily_logs::dsl::*;

    let conn = &mut establish_connection();

    debug!("Getting log dates");
    let mut all_daily_logs: Vec<DailyLog> = daily_logs
        .select(DailyLog::as_select())
        .load(conn)
        .expect("Expected to get all daily logs");

    debug!("{:?}", all_daily_logs);

    // Sort logs by date
    all_daily_logs.sort_by(|a, b| b.log_date.cmp(&a.log_date));

    return all_daily_logs;
}

#[tauri::command]
fn update_daily_log(daily_log: DailyLog) {
    use self::schema::daily_logs::dsl::*;

    let conn = &mut establish_connection();
    debug!("Saving log: {:?}", daily_log);

    diesel::update(daily_logs.filter(id.eq(&daily_log.id)))
        .set((content.eq(daily_log.content.to_owned()),))
        .execute(conn)
        .unwrap();
}

#[tauri::command]
fn add_today_date() -> Option<DailyLog> {
    use self::schema::daily_logs::dsl::*;

    let now: NaiveDate = Utc::now().date_naive();

    let conn = &mut establish_connection();

    debug!("Check if today already exists");
    let today_log: Vec<DailyLog> = daily_logs
        .select(DailyLog::as_select())
        .filter(log_date.eq(now))
        .load(conn)
        .expect("Expected to get daily log");

    debug!("{:?}", today_log);

    match today_log.first() {
        Some(log) => {
            return Some(log.clone());
        }
        None => {
            let new_today_log = DailyLog {
                id: Uuid::new_v4().to_string(),
                content: String::new(),
                log_date: now,
            };

            diesel::insert_into(daily_logs)
                .values(&new_today_log)
                .execute(conn)
                .unwrap();

            return Some(new_today_log);
        }
    }
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    info!("Running migrations.");
    let mut conn = SqliteConnection::establish("log_files.db").unwrap();
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    info!("Starting Tauri backend.");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_log_dates,
            update_daily_log,
            add_today_date
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
