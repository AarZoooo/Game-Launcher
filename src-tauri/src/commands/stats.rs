use crate::db::{database, sessions};
use serde::Serialize;
use tauri::AppHandle;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TodayPlaytime {
    pub game_id: String,
    pub minutes_played_today: i64,
}

#[tauri::command]
pub fn get_today_playtime(app: AppHandle) -> Result<Vec<TodayPlaytime>, String> {
    let connection = database::open_database(&app)?;
    sessions::get_today_playtime(&connection)
}
