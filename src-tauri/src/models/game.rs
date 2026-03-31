use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: String,
    pub title: String,
    pub exe_path: String,
    pub cover_art: String,
    pub platform: String,
    pub total_playtime: u64,
    pub last_played: Option<String>,
    pub status: String,
    pub genres: Vec<String>,
    pub description: String,
}
