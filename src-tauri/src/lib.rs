mod commands;
mod db;
mod launch;
mod media;
mod models;
mod perf;
mod tracking;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = dotenvy::from_filename("src-tauri/.env")
        .or_else(|_| dotenvy::from_filename(".env"));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            db::schema::init_database(app.handle())?;
            #[cfg(debug_assertions)]
            perf::monitor::start_dev_monitor();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::storage::read_games,
            commands::storage::write_games,
            commands::storage::scan_local_games,
            commands::storage::refresh_installed_game_media,
            commands::manual_add::pick_game_executable,
            commands::launch_game::launch_game,
            commands::stats::get_today_playtime,
            commands::igdb::search_igdb_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
