use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;
use tauri::{AppHandle, Manager};

const DATABASE_FILE_NAME: &str = "launcher.db";

pub fn database_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Failed to resolve app data directory: {error}"))?;

    fs::create_dir_all(&app_data_dir)
        .map_err(|error| format!("Failed to create app data directory: {error}"))?;

    Ok(app_data_dir.join(DATABASE_FILE_NAME))
}

pub fn open_database(app: &AppHandle) -> Result<Connection, String> {
    let database_path = database_path(app)?;

    Connection::open(&database_path)
        .map_err(|error| format!("Failed to open database at {}: {error}", database_path.display()))
}
