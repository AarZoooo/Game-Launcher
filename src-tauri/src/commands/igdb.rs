use tauri::AppHandle;

use crate::db::{database, igdb_cache};
use crate::media::providers::igdb::{self, IgdbSearchResult};

#[tauri::command]
pub async fn search_igdb_game(
    app: AppHandle,
    title: String,
) -> Result<Vec<IgdbSearchResult>, String> {
    let connection = database::open_database(&app)?;
    if let Some(cached) = igdb_cache::get_search_results(&connection, &title)? {
        println!("[media] igdb cache hit: title='{}' count={}", title.trim(), cached.len());
        return Ok(cached);
    }

    let search_title = title.clone();
    let results = tauri::async_runtime::spawn_blocking(move || igdb::search_games(&search_title))
        .await
        .map_err(|error| format!("Failed to join IGDB search task: {error}"))??;

    igdb_cache::store_search_results(&connection, &title, &results)?;
    Ok(results)
}
