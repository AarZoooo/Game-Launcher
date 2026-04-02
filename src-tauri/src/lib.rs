mod commands;
mod db;
mod launch;
mod models;
mod tracking;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            db::schema::init_database(app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::storage::read_games,
            commands::storage::write_games,
            commands::storage::scan_local_games,
            commands::manual_add::pick_game_executable,
            commands::launch_game::launch_game,
            commands::stats::get_today_playtime
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
