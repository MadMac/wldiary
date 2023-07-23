// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;

use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};
use log::{debug, info};
use schematic::ConfigLoader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Mutex;
use uuid::Uuid;
use wldiary::establish_connection;
use wldiary::AppConfig;
use wldiary::AppState;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

mod models;
mod schema;

use self::models::DailyLog;

#[tauri::command]
fn get_log_dates(config: tauri::State<ConfigState>) -> Vec<DailyLog> {
    use self::schema::daily_logs::dsl::*;

    debug!("{:?}", &config.0.config.lock().unwrap());

    let conn = &mut establish_connection(&config.0.config.lock().unwrap());

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
fn update_daily_log(daily_log: DailyLog, config: tauri::State<ConfigState>) {
    use self::schema::daily_logs::dsl::*;

    debug!("{:?}", &config.0.config.lock().unwrap());

    let conn = &mut establish_connection(&config.0.config.lock().unwrap());
    debug!("Saving log: {:?}", daily_log);

    diesel::update(daily_logs.filter(id.eq(&daily_log.id)))
        .set((content.eq(daily_log.content.to_owned()),))
        .execute(conn)
        .unwrap();
}

#[tauri::command]
fn delete_daily_log(daily_log: DailyLog, config: tauri::State<ConfigState>) {
    use self::schema::daily_logs::dsl::*;

    let conn = &mut establish_connection(&config.0.config.lock().unwrap());
    debug!("Deleting log: {:?}", daily_log);

    diesel::delete(daily_logs.filter(id.eq(&daily_log.id)))
        .execute(conn)
        .unwrap();
}

#[tauri::command]
fn add_today_date(config: tauri::State<ConfigState>) -> Option<DailyLog> {
    use self::schema::daily_logs::dsl::*;

    debug!("{:?}", &config.0.config.lock().unwrap());

    let now: NaiveDate = Utc::now().date_naive();

    let conn = &mut establish_connection(&config.0.config.lock().unwrap());

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

#[tauri::command]
fn load_db_file(file_path: String, config: tauri::State<ConfigState>) -> () {
    debug!("Updating db path to: {}", file_path);
    let mut state = config.0.config.lock().expect("Could not lock mutex");
    state.log_path = file_path;
}

struct ConfigState(AppState);

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    // Check or create config file
    if !Path::new("config.json").exists() {
        info!("Config doesn't exist. Creating new config file.");

        let new_app_config = AppConfig {
            log_path: "log_files.db".to_string(),
        };

        let path = Path::new("config.json");

        let mut file = match File::create(&path) {
            Err(err) => panic!("Couldn't create config file {}", err),
            Ok(file) => file,
        };

        match file.write_all(serde_json::to_string(&new_app_config).unwrap().as_bytes()) {
            Err(err) => panic!("Couldn't write to config file {}", err),
            Ok(_) => info!("Successfully created config file"),
        }
    } else {
        info!("Config file found.")
    }

    let app_config = match ConfigLoader::<AppConfig>::new()
        .file("config.json")
        .unwrap()
        .load()
    {
        Err(err) => panic!("Couldn't open config file {}", err.to_full_string()),
        Ok(config) => config,
    };

    debug!("Config: {:?}", app_config.config);

    info!("Running migrations.");
    let mut conn = SqliteConnection::establish(&app_config.config.log_path).unwrap();
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    info!("Starting Tauri backend.");
    tauri::Builder::default()
        .manage(ConfigState(AppState {
            config: Mutex::new(app_config.config),
        }))
        .invoke_handler(tauri::generate_handler![
            get_log_dates,
            update_daily_log,
            add_today_date,
            delete_daily_log,
            load_db_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
