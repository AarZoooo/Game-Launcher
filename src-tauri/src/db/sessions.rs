use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};

use crate::commands::stats::TodayPlaytime;

fn now_epoch_seconds() -> Result<i64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .map_err(|error| format!("Failed to read system time: {error}"))
}

pub fn begin_session(connection: &Connection, session_id: &str, game_id: &str) -> Result<i64, String> {
    let started_at = now_epoch_seconds()?;

    connection
        .execute(
            "INSERT INTO play_sessions (id, game_id, started_at)
             VALUES (?1, ?2, strftime('%Y-%m-%dT%H:%M:%SZ', ?3, 'unixepoch'))",
            params![session_id, game_id, started_at],
        )
        .map_err(|error| format!("Failed to create play session: {error}"))?;

    Ok(started_at)
}

pub fn finish_session(
    connection: &Connection,
    session_id: &str,
    ended_at: i64,
) -> Result<Option<i64>, String> {
    let updated_rows = connection
        .execute(
            "UPDATE play_sessions
             SET ended_at = strftime('%Y-%m-%dT%H:%M:%SZ', ?2, 'unixepoch'),
                 duration_seconds = MAX(0, CAST(?2 AS INTEGER) - CAST(strftime('%s', started_at) AS INTEGER))
             WHERE id = ?1 AND ended_at IS NULL",
            params![session_id, ended_at],
        )
        .map_err(|error| format!("Failed to finish play session: {error}"))?;

    if updated_rows == 0 {
        return Ok(None);
    }

    connection
        .query_row(
            "SELECT duration_seconds FROM play_sessions WHERE id = ?1",
            params![session_id],
            |row| row.get::<_, Option<i64>>(0),
        )
        .map_err(|error| format!("Failed to read finished session duration: {error}"))
}

pub fn get_today_playtime(connection: &Connection) -> Result<Vec<TodayPlaytime>, String> {
    let mut statement = connection
        .prepare(
            "WITH bounds AS (
                SELECT
                    strftime('%s', 'now', 'localtime', 'start of day', 'utc') AS day_start_utc,
                    strftime('%s', 'now', 'localtime', 'start of day', '+1 day', 'utc') AS day_end_utc
             )
             SELECT
                 game_id,
                 SUM(
                     CASE
                         WHEN COALESCE(strftime('%s', ended_at), strftime('%s', 'now')) <= bounds.day_start_utc
                              OR strftime('%s', started_at) >= bounds.day_end_utc THEN 0
                         ELSE CAST(
                             (
                                 MIN(COALESCE(strftime('%s', ended_at), strftime('%s', 'now')), bounds.day_end_utc) -
                                 MAX(strftime('%s', started_at), bounds.day_start_utc)
                             ) / 60 AS INTEGER
                         )
                     END
                 ) AS minutes_played_today
             FROM play_sessions, bounds
             GROUP BY game_id
             HAVING minutes_played_today > 0
             ORDER BY minutes_played_today DESC, game_id ASC",
        )
        .map_err(|error| format!("Failed to prepare today playtime query: {error}"))?;

    let rows = statement
        .query_map([], |row| {
            Ok(TodayPlaytime {
                game_id: row.get(0)?,
                minutes_played_today: row.get(1)?,
            })
        })
        .map_err(|error| format!("Failed to query today playtime: {error}"))?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row.map_err(|error| format!("Failed to read today playtime row: {error}"))?);
    }

    Ok(results)
}

pub fn get_game_today_playtime_minutes(
    connection: &Connection,
    game_id: &str,
) -> Result<i64, String> {
    connection
        .query_row(
            "WITH bounds AS (
                SELECT
                    strftime('%s', 'now', 'localtime', 'start of day', 'utc') AS day_start_utc,
                    strftime('%s', 'now', 'localtime', 'start of day', '+1 day', 'utc') AS day_end_utc
             )
             SELECT COALESCE(SUM(
                 CASE
                     WHEN COALESCE(strftime('%s', ended_at), strftime('%s', 'now')) <= bounds.day_start_utc
                          OR strftime('%s', started_at) >= bounds.day_end_utc THEN 0
                     ELSE CAST(
                         (
                             MIN(COALESCE(strftime('%s', ended_at), strftime('%s', 'now')), bounds.day_end_utc) -
                             MAX(strftime('%s', started_at), bounds.day_start_utc)
                         ) / 60 AS INTEGER
                     )
                 END
             ), 0)
             FROM play_sessions, bounds
             WHERE game_id = ?1",
            params![game_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| format!("Failed to read today's playtime for game: {error}"))
}
