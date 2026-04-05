use rusqlite::{Connection, OptionalExtension, params};

use crate::media::matching::title::normalize_title;
use crate::media::providers::igdb::IgdbSearchResult;

const CACHE_MAX_AGE_HOURS: i64 = 24;

fn cache_key(title: &str) -> String {
    normalize_title(title)
}

pub fn get_search_results(
    connection: &Connection,
    title: &str,
) -> Result<Option<Vec<IgdbSearchResult>>, String> {
    let key = cache_key(title);
    let cached = connection
        .query_row(
            "SELECT results_json
             FROM igdb_search_cache
             WHERE title_key = ?1
               AND cached_at >= datetime('now', ?2)
             LIMIT 1",
            params![key, format!("-{} hours", CACHE_MAX_AGE_HOURS)],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| format!("Failed to query IGDB search cache: {error}"))?;

    cached
        .map(|json| {
            serde_json::from_str::<Vec<IgdbSearchResult>>(&json)
                .map_err(|error| format!("Failed to parse IGDB cached search results: {error}"))
        })
        .transpose()
}

pub fn store_search_results(
    connection: &Connection,
    title: &str,
    results: &[IgdbSearchResult],
) -> Result<(), String> {
    let key = cache_key(title);
    let results_json = serde_json::to_string(results)
        .map_err(|error| format!("Failed to serialize IGDB search cache results: {error}"))?;

    connection
        .execute(
            "INSERT INTO igdb_search_cache (title_key, title_query, results_json, cached_at)
             VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP)
             ON CONFLICT(title_key) DO UPDATE SET
                title_query = excluded.title_query,
                results_json = excluded.results_json,
                cached_at = CURRENT_TIMESTAMP",
            params![key, title.trim(), results_json],
        )
        .map_err(|error| format!("Failed to store IGDB search cache: {error}"))?;

    Ok(())
}
