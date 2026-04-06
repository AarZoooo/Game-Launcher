use std::time::{SystemTime, UNIX_EPOCH};

use tauri::AppHandle;

use crate::db::{database, games, sessions};
use crate::models::game::Game;

#[derive(Debug, Clone)]
pub struct SessionSummary {
    pub game_id: String,
    pub title: String,
    pub total_playtime_minutes: i64,
    pub minutes_played_today: i64,
    pub last_played: Option<String>,
    pub game: Game,
}

fn build_session_id(game_id: &str) -> Result<String, String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .map_err(|error| format!("Failed to read system time: {error}"))?;

    Ok(format!("session-{game_id}-{timestamp}"))
}

pub fn start_session(app: &AppHandle, game_id: &str) -> Result<(String, Option<Game>), String> {
    let connection = database::open_database(app)?;
    let session_id = build_session_id(game_id)?;
    sessions::begin_session(&connection, &session_id, game_id)?;
    let game = games::get_game_by_id(&connection, game_id)?;
    Ok((session_id, game))
}

pub fn finish_session(
    app: &AppHandle,
    game_id: &str,
    session_id: &str,
    ended_at: i64,
) -> Result<Option<SessionSummary>, String> {
    let connection = database::open_database(app)?;
    if let Some(duration_seconds) = sessions::finish_session(&connection, session_id, ended_at)? {
        games::apply_session_playtime(&connection, game_id, ended_at, duration_seconds)?;
        if let Some(snapshot) = games::get_game_stats_snapshot(&connection, game_id)? {
            let minutes_played_today =
                sessions::get_game_today_playtime_minutes(&connection, game_id)?;
            let game = games::get_game_by_id(&connection, game_id)?
                .ok_or_else(|| format!("Failed to load updated game after finishing session: {game_id}"))?;
            return Ok(Some(SessionSummary {
                game_id: snapshot.id,
                title: snapshot.title,
                total_playtime_minutes: snapshot.total_playtime_minutes,
                minutes_played_today,
                last_played: snapshot.last_played,
                game,
            }));
        }
    }
    Ok(None)
}
