use rusqlite::{params, Connection, OptionalExtension};

use crate::db::sessions;
use crate::models::game::Game;
use crate::models::game::{default_completion, default_coop, default_rating};

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
            "SELECT games.id, games.title, games.exe_path, games.cover_art, games.cover_vertical, games.cover_horizontal,
                    games.banner, games.icon, games.accent_color, games.platform,
                    COALESCE(session_stats.total_playtime_minutes, games.total_playtime) AS synced_total_playtime,
                    COALESCE(session_stats.last_played, games.last_played) AS synced_last_played,
                    games.status, games.genres, games.description, games.rating, games.coop, games.completion,
                    games.media_query_signature
             FROM games
             LEFT JOIN (
                 SELECT
                     game_id,
                     SUM(
                         CASE
                             WHEN ended_at IS NULL THEN
                                 MAX(
                                     0,
                                     CAST(
                                         (
                                             strftime('%s', 'now') - CAST(strftime('%s', started_at) AS INTEGER) + 30
                                         ) / 60 AS INTEGER
                                     )
                                 )
                             ELSE MAX(0, COALESCE(duration_seconds, 0) + 30) / 60
                         END
                     ) AS total_playtime_minutes,
                     MAX(COALESCE(ended_at, started_at)) AS last_played
                 FROM play_sessions
                 GROUP BY game_id
             ) session_stats ON session_stats.game_id = games.id
             WHERE installed = 1
             ORDER BY title COLLATE NOCASE ASC",
        )
        .map_err(|error| format!("Failed to prepare games query: {error}"))?;

    let rows = statement
        .query_map([], |row| {
            let genres_json: String = row.get(13)?;
            let genres = serde_json::from_str::<Vec<String>>(&genres_json).unwrap_or_default();
            let total_playtime = row
                .get::<_, i64>(10)?
                .try_into()
                .unwrap_or_default();

            Ok(Game {
                id: row.get(0)?,
                title: row.get(1)?,
                exe_path: row.get(2)?,
                cover_art: row.get(3)?,
                cover_vertical: row.get(4)?,
                cover_horizontal: row.get(5)?,
                banner: row.get(6)?,
                icon: row.get(7)?,
                accent_color: row.get(8)?,
                platform: row.get(9)?,
                total_playtime,
                last_played: row.get(11)?,
                status: row.get(12)?,
                genres,
                description: row.get(14)?,
                rating: row.get(15)?,
                coop: row.get(16)?,
                completion: row.get(17)?,
                sessions: Vec::new(),
                media_query_signature: row.get(18)?,
            })
        })
        .map_err(|error| format!("Failed to query games: {error}"))?;

    let mut games = Vec::new();
    for row in rows {
        games.push(row.map_err(|error| format!("Failed to read game row: {error}"))?);
    }

    let game_ids = games.iter().map(|game| game.id.clone()).collect::<Vec<_>>();
    let sessions_by_game = sessions::get_sessions_by_game_ids(connection, &game_ids)?;

    for game in &mut games {
        game.sessions = sessions_by_game
            .get(&game.id)
            .cloned()
            .unwrap_or_default();
    }

    Ok(games)
}

pub fn get_game_by_id(connection: &Connection, game_id: &str) -> Result<Option<Game>, String> {
    Ok(get_all_games(connection)?
        .into_iter()
        .find(|game| game.id.eq_ignore_ascii_case(game_id)))
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

fn sanitized_rating(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        default_rating()
    } else {
        trimmed.to_string()
    }
}

fn sanitized_coop(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        default_coop()
    } else {
        trimmed.to_string()
    }
}

fn sanitized_completion(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        default_completion()
    } else {
        trimmed.to_string()
    }
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
                         cover_vertical = ?5,
                         cover_horizontal = ?6,
                         banner = ?7,
                         icon = ?8,
                         accent_color = ?9,
                         platform = ?10,
                         status = ?11,
                         genres = ?12,
                         description = ?13,
                         rating = CASE
                             WHEN trim(?14) = '' OR ?14 = '0.0' THEN COALESCE(NULLIF(rating, ''), '0.0')
                             ELSE ?14
                         END,
                         coop = CASE
                             WHEN trim(?15) = '' OR lower(?15) = 'unknown' THEN COALESCE(NULLIF(coop, ''), 'Unknown')
                             ELSE ?15
                         END,
                         completion = CASE
                             WHEN trim(?16) = '' OR lower(?16) = 'unknown' THEN COALESCE(NULLIF(completion, ''), 'Unknown')
                             ELSE ?16
                         END,
                         media_query_signature = COALESCE(?17, media_query_signature),
                         installed = 1
                     WHERE id = ?18",
                    params![
                        game.id,
                        game.title,
                        game.exe_path,
                        game.cover_art,
                        game.cover_vertical,
                        game.cover_horizontal,
                        game.banner,
                        game.icon,
                        game.accent_color,
                        game.platform,
                        game.status,
                        genres_json,
                        game.description,
                        sanitized_rating(&game.rating),
                        sanitized_coop(&game.coop),
                        sanitized_completion(&game.completion),
                        game.media_query_signature,
                        existing_id
                    ],
                )
                .map_err(|error| format!("Failed to update game '{}': {error}", game.title))?;
        } else {
            transaction
                .execute(
                    "INSERT INTO games (
                        id, title, exe_path, installed, cover_art, cover_vertical, cover_horizontal, banner, icon,
                        accent_color, platform, total_playtime, last_played, status, genres, description, rating,
                        coop, completion, media_query_signature, created_at
                     ) VALUES (?1, ?2, ?3, 1, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, CURRENT_TIMESTAMP)",
                    params![
                        game.id,
                        game.title,
                        game.exe_path,
                        game.cover_art,
                        game.cover_vertical,
                        game.cover_horizontal,
                        game.banner,
                        game.icon,
                        game.accent_color,
                        game.platform,
                        game.total_playtime as i64,
                        game.last_played,
                        game.status,
                        genres_json,
                        game.description,
                        sanitized_rating(&game.rating),
                        sanitized_coop(&game.coop),
                        sanitized_completion(&game.completion),
                        game.media_query_signature
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

pub fn update_game_media(connection: &Connection, game: &Game) -> Result<(), String> {
    connection
        .execute(
            "UPDATE games
             SET cover_art = ?2,
                 cover_vertical = ?3,
                 cover_horizontal = ?4,
                 banner = ?5,
                 icon = ?6,
                 accent_color = ?7,
                 genres = ?8,
                 description = ?9,
                 rating = ?10,
                 coop = ?11,
                 completion = ?12,
                 media_query_signature = ?13
             WHERE id = ?1",
            params![
                game.id,
                game.cover_art,
                game.cover_vertical,
                game.cover_horizontal,
                game.banner,
                game.icon,
                game.accent_color,
                serde_json::to_string(&game.genres)
                    .map_err(|error| format!("Failed to serialize updated game genres: {error}"))?,
                game.description,
                sanitized_rating(&game.rating),
                sanitized_coop(&game.coop),
                sanitized_completion(&game.completion),
                game.media_query_signature,
            ],
        )
        .map_err(|error| format!("Failed to update media for game '{}': {error}", game.title))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use crate::db::schema;

    use super::get_all_games;

    #[test]
    fn returns_games_with_session_history() {
        let connection = Connection::open_in_memory().expect("in-memory db");
        connection
            .execute_batch(schema::INITIAL_SCHEMA)
            .expect("schema");

        connection
            .execute(
                "INSERT INTO games (
                    id, title, exe_path, installed, cover_art, platform, total_playtime, last_played, status, genres, description, rating, coop, completion
                 ) VALUES (?1, ?2, ?3, 1, '', 'local', 0, NULL, 'installed', '[]', '', '0.0', 'Unknown', 'Unknown')",
                rusqlite::params!["game-1", "Game One", "D:\\Games\\GameOne\\game.exe"],
            )
            .expect("game insert");

        connection
            .execute(
                "INSERT INTO play_sessions (id, game_id, started_at, ended_at, duration_seconds)
                 VALUES (?1, ?2, '2026-04-05T10:00:00Z', '2026-04-05T11:00:00Z', 3600)",
                rusqlite::params!["session-1", "game-1"],
            )
            .expect("session insert");

        let games = get_all_games(&connection).expect("games query");
        assert_eq!(games.len(), 1);
        assert_eq!(games[0].sessions.len(), 1);
        assert_eq!(games[0].total_playtime, 60);
        assert_eq!(games[0].sessions[0].duration, 3_600_000);
        assert_eq!(
            games[0].last_played.as_deref(),
            Some("2026-04-05T11:00:00Z")
        );
    }
}
