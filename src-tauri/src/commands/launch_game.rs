use std::path::Path;
use tauri::AppHandle;

use crate::launch::process_launcher::spawn_game_process;
use crate::tracking::events::emit_game_process_state;
use crate::tracking::process_tracker::start_tracking;

#[tauri::command]
pub fn launch_game(
    app: AppHandle,
    exe_path: String,
    game_id: Option<String>,
) -> Result<String, String> {
    let trimmed_path = exe_path.trim();

    if trimmed_path.is_empty() {
        return Err("Executable path cannot be empty.".into());
    }

    if !Path::new(trimmed_path).exists() {
        return Err(format!("Executable not found: {trimmed_path}"));
    }

    println!(
        "[launch_game] launching exe_path={} game_id={:?}",
        trimmed_path, game_id
    );

    let pid = match spawn_game_process(trimmed_path) {
        Ok(pid) => pid,
        Err(error) => {
            emit_game_process_state(
                &app,
                &game_id,
                trimmed_path,
                "error",
                Some(error.clone()),
            );
            emit_game_process_state(&app, &game_id, trimmed_path, "exited", None);
            return Err(error);
        }
    };
    println!("[launch_game] initial pid={pid} exe_path={trimmed_path}");

    emit_game_process_state(&app, &game_id, trimmed_path, "started", None);

    let tracked_app = app.clone();
    let tracked_path = trimmed_path.to_string();
    let tracked_game_id = game_id.clone();
    start_tracking(tracked_app, tracked_path, tracked_game_id);

    Ok(format!("Launched game: {trimmed_path}"))
}
