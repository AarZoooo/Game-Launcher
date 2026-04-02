use serde::Serialize;
use tauri::{AppHandle, Emitter};

pub const GAME_PROCESS_EVENT: &str = "game-process-state";

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameProcessEvent {
    game_id: Option<String>,
    exe_path: String,
    state: &'static str,
    message: Option<String>,
}

pub fn emit_game_process_state(
    app: &AppHandle,
    game_id: &Option<String>,
    exe_path: &str,
    state: &'static str,
    message: Option<String>,
) {
    println!(
        "[launch_game] emitting event state={state} game_id={:?} exe_path={exe_path} message={message:?}",
        game_id
    );

    let _ = app.emit(
        GAME_PROCESS_EVENT,
        GameProcessEvent {
            game_id: game_id.clone(),
            exe_path: exe_path.to_string(),
            state,
            message,
        },
    );
}
