use rusqlite::{params, Connection, OptionalExtension};

use crate::models::game::Game;

#[derive(Debug, Clone)]
pub struct GameStatsSnapshot {
    pub id: String,
    pub title: String,
    pub total_playtime_minutes: i64,
    pub last_played: Option<String>,
}

pub fn get_all_games(connection: &Connection) -> Result<Vec<Game>, String> {
    let mut statement = connection
        .prepare(
            "SELECT id, title, exe_path, cover_art, platform, total_playtime, last_played, status, genres, description
             FROM games
             WHERE installed = 1
             ORDER BY title COLLATE NOCASE ASC",
        )
        .map_err(|error| format!("Failed to prepare games query: {error}"))?;

    let rows = statement
        .query_map([], |row| {
            let genres_json: String = row.get(8)?;
            let genres = serde_json::from_str::<Vec<String>>(&genres_json).unwrap_or_default();
            let total_playtime = row
                .get::<_, i64>(5)?
                .try_into()
                .unwrap_or_default();

            Ok(Game {
                id: row.get(0)?,
                title: row.get(1)?,
                exe_path: row.get(2)?,
                cover_art: row.get(3)?,
                platform: row.get(4)?,
                total_playtime,
                last_played: row.get(6)?,
                status: row.get(7)?,
                genres,
                description: row.get(9)?,
            })
        })
        .map_err(|error| format!("Failed to query games: {error}"))?;

    let mut games = Vec::new();
    for row in rows {
        games.push(row.map_err(|error| format!("Failed to read game row: {error}"))?);
    }

    Ok(games)
}

fn rounded_minutes(duration_seconds: i64) -> i64 {
    if duration_seconds <= 0 {
        0
    } else {
        (duration_seconds + 30) / 60
    }
}

fn existing_identity(connection: &Connection, game: &Game) -> Result<Option<String>, String> {
    connection
        .query_row(
            "SELECT id FROM games WHERE id = ?1 OR exe_path = ?2 LIMIT 1",
            params![game.id, game.exe_path],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| format!("Failed to look up existing game identity: {error}"))
}

pub fn sync_installed_games(connection: &mut Connection, games: &[Game]) -> Result<(), String> {
    let transaction = connection
        .transaction()
        .map_err(|error| format!("Failed to start games transaction: {error}"))?;

    transaction
        .execute("UPDATE games SET installed = 0", [])
        .map_err(|error| format!("Failed to mark games as not installed: {error}"))?;

    for game in games {
        let genres_json = serde_json::to_string(&game.genres)
            .map_err(|error| format!("Failed to serialize game genres: {error}"))?;
        let identity = existing_identity(&transaction, game)?;

        if let Some(existing_id) = identity {
            transaction
                .execute(
                    "UPDATE games
                     SET id = ?1,
                         title = ?2,
                         exe_path = ?3,
                         cover_art = ?4,
                         platform = ?5,
                         status = ?6,
                         genres = ?7,
                         description = ?8,
                         installed = 1
                     WHERE id = ?9",
                    params![
                        game.id,
                        game.title,
                        game.exe_path,
                        game.cover_art,
                        game.platform,
                        game.status,
                        genres_json,
                        game.description,
                        existing_id
                    ],
                )
                .map_err(|error| format!("Failed to update game '{}': {error}", game.title))?;
        } else {
            transaction
                .execute(
                    "INSERT INTO games (
                        id, title, exe_path, installed, cover_art, platform, total_playtime,
                        last_played, status, genres, description, created_at
                     ) VALUES (?1, ?2, ?3, 1, ?4, ?5, ?6, ?7, ?8, ?9, ?10, CURRENT_TIMESTAMP)",
                    params![
                        game.id,
                        game.title,
                        game.exe_path,
                        game.cover_art,
                        game.platform,
                        game.total_playtime as i64,
                        game.last_played,
                        game.status,
                        genres_json,
                        game.description
                    ],
                )
                .map_err(|error| format!("Failed to insert game '{}': {error}", game.title))?;
        }
    }

    transaction
        .commit()
        .map_err(|error| format!("Failed to commit games transaction: {error}"))
}

pub fn apply_session_playtime(
    connection: &Connection,
    game_id: &str,
    ended_at: i64,
    duration_seconds: i64,
) -> Result<(), String> {
    let additional_minutes = rounded_minutes(duration_seconds);

    connection
        .execute(
            "UPDATE games
             SET total_playtime = total_playtime + ?2,
                 last_played = strftime('%Y-%m-%dT%H:%M:%SZ', ?3, 'unixepoch')
             WHERE id = ?1",
            params![game_id, additional_minutes, ended_at],
        )
        .map_err(|error| format!("Failed to update game play stats: {error}"))?;

    Ok(())
}

pub fn get_game_stats_snapshot(
    connection: &Connection,
    game_id: &str,
) -> Result<Option<GameStatsSnapshot>, String> {
    connection
        .query_row(
            "SELECT id, title, total_playtime, last_played
             FROM games
             WHERE id = ?1
             LIMIT 1",
            params![game_id],
            |row| {
                Ok(GameStatsSnapshot {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    total_playtime_minutes: row.get(2)?,
                    last_played: row.get(3)?,
                })
            },
        )
        .optional()
        .map_err(|error| format!("Failed to read game stats snapshot: {error}"))
}
