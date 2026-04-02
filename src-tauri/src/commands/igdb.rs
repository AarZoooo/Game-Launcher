use crate::media::providers::igdb::{self, IgdbSearchResult};

#[tauri::command]
pub fn search_igdb_game(title: String) -> Result<Vec<IgdbSearchResult>, String> {
    igdb::search_games(&title)
}
