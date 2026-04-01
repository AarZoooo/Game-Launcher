use rusqlite::{params, Connection};

use crate::models::game::Game;

pub fn get_all_games(connection: &Connection) -> Result<Vec<Game>, String> {
    let mut statement = connection
        .prepare(
            "SELECT id, title, exe_path, cover_art, platform, total_playtime, last_played, status, genres, description
             FROM games
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

pub fn replace_all_games(connection: &mut Connection, games: &[Game]) -> Result<(), String> {
    let transaction = connection
        .transaction()
        .map_err(|error| format!("Failed to start games transaction: {error}"))?;

    transaction
        .execute("DELETE FROM games", [])
        .map_err(|error| format!("Failed to clear games table: {error}"))?;

    for game in games {
        let genres_json = serde_json::to_string(&game.genres)
            .map_err(|error| format!("Failed to serialize game genres: {error}"))?;

        transaction
            .execute(
                "INSERT INTO games (
                    id, title, exe_path, cover_art, platform, total_playtime,
                    last_played, status, genres, description, created_at
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, CURRENT_TIMESTAMP)",
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

    transaction
        .commit()
        .map_err(|error| format!("Failed to commit games transaction: {error}"))
}
