use rusqlite::Connection;
use tauri::AppHandle;

use super::database::open_database;

const INITIAL_SCHEMA: &str = r#"
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS games (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  exe_path TEXT NOT NULL UNIQUE,
  installed INTEGER NOT NULL DEFAULT 1,
  cover_art TEXT,
  cover_vertical TEXT,
  cover_horizontal TEXT,
  banner TEXT,
  icon TEXT,
  accent_color TEXT,
  platform TEXT NOT NULL,
  total_playtime INTEGER NOT NULL DEFAULT 0,
  last_played TEXT,
  status TEXT,
  genres TEXT NOT NULL DEFAULT '[]',
  description TEXT,
  rating TEXT NOT NULL DEFAULT '0.0',
  coop TEXT NOT NULL DEFAULT 'Unknown',
  completion TEXT NOT NULL DEFAULT 'Unknown',
  media_query_signature TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS play_sessions (
  id TEXT PRIMARY KEY,
  game_id TEXT NOT NULL,
  started_at TEXT NOT NULL,
  ended_at TEXT,
  duration_seconds INTEGER,
  FOREIGN KEY (game_id) REFERENCES games(id)
);

CREATE TABLE IF NOT EXISTS igdb_search_cache (
  title_key TEXT PRIMARY KEY,
  title_query TEXT NOT NULL,
  results_json TEXT NOT NULL,
  cached_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
"#;

fn ensure_games_columns(connection: &Connection) -> Result<(), String> {
    let mut statement = connection
        .prepare("PRAGMA table_info(games)")
        .map_err(|error| format!("Failed to inspect games schema: {error}"))?;

    let columns = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|error| format!("Failed to read games schema: {error}"))?;

    let mut existing = Vec::new();
    for column in columns {
        existing.push(column.map_err(|error| format!("Failed to inspect games column: {error}"))?);
    }

    if !existing.iter().any(|column| column == "genres") {
        connection
            .execute(
                "ALTER TABLE games ADD COLUMN genres TEXT NOT NULL DEFAULT '[]'",
                [],
            )
            .map_err(|error| format!("Failed to add genres column to games table: {error}"))?;
    }

    if !existing.iter().any(|column| column == "installed") {
        connection
            .execute(
                "ALTER TABLE games ADD COLUMN installed INTEGER NOT NULL DEFAULT 1",
                [],
            )
            .map_err(|error| format!("Failed to add installed column to games table: {error}"))?;
    }

    for (column, definition) in [
        ("cover_vertical", "TEXT"),
        ("cover_horizontal", "TEXT"),
        ("banner", "TEXT"),
        ("icon", "TEXT"),
        ("accent_color", "TEXT"),
        ("rating", "TEXT NOT NULL DEFAULT '0.0'"),
        ("coop", "TEXT NOT NULL DEFAULT 'Unknown'"),
        ("completion", "TEXT NOT NULL DEFAULT 'Unknown'"),
        ("media_query_signature", "TEXT"),
    ] {
        if !existing.iter().any(|existing_column| existing_column == column) {
            connection
                .execute(
                    &format!("ALTER TABLE games ADD COLUMN {column} {definition}"),
                    [],
                )
                .map_err(|error| format!("Failed to add {column} column to games table: {error}"))?;
        }
    }

    Ok(())
}

fn apply_schema(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(INITIAL_SCHEMA)
        .map_err(|error| format!("Failed to initialize database schema: {error}"))?;

    ensure_games_columns(connection)
}

pub fn init_database(app: &AppHandle) -> Result<(), String> {
    let connection = open_database(app)?;
    apply_schema(&connection)
}
