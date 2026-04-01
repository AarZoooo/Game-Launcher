use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: String,
    pub title: String,
    pub exe_path: String,
    #[serde(default)]
    pub cover_art: String,
    pub platform: String,
    #[serde(default)]
    pub total_playtime: u64,
    #[serde(default)]
    pub last_played: Option<String>,
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(default)]
    pub description: String,
}

fn default_status() -> String {
    "installed".to_string()
}
