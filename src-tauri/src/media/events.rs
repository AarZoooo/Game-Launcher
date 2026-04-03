use tauri::{AppHandle, Emitter};

use crate::models::game::Game;

pub const GAME_MEDIA_UPDATED_EVENT: &str = "game-media-updated";
pub const GAME_MEDIA_RESOLUTION_EVENT: &str = "game-media-resolution";

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameMediaResolutionEvent {
    game_id: String,
    exe_path: String,
    state: &'static str,
}

pub fn emit_game_media_updated(app: &AppHandle, game: &Game) {
    println!(
        "[media] emitting event state=updated game_id={} title='{}'",
        game.id, game.title
    );

    let _ = app.emit(GAME_MEDIA_UPDATED_EVENT, game.clone());
}

pub fn emit_game_media_resolution_state(
    app: &AppHandle,
    game_id: &str,
    exe_path: &str,
    state: &'static str,
) {
    println!(
        "[media] emitting resolution state={} game_id={} exe_path={}",
        state, game_id, exe_path
    );

    let _ = app.emit(
        GAME_MEDIA_RESOLUTION_EVENT,
        GameMediaResolutionEvent {
            game_id: game_id.to_string(),
            exe_path: exe_path.to_string(),
            state,
        },
    );
}
