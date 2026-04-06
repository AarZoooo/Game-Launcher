use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameSession {
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub duration: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: String,
    pub title: String,
    pub exe_path: String,
    #[serde(default)]
    pub cover_art: String,
    #[serde(default)]
    pub cover_vertical: Option<String>,
    #[serde(default)]
    pub cover_horizontal: Option<String>,
    #[serde(default)]
    pub banner: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub accent_color: Option<String>,
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
    #[serde(default = "default_rating")]
    pub rating: String,
    #[serde(default = "default_coop")]
    pub coop: String,
    #[serde(default = "default_completion")]
    pub completion: String,
    #[serde(default)]
    pub sessions: Vec<GameSession>,
    #[serde(default)]
    pub media_query_signature: Option<String>,
}

fn default_status() -> String {
    "installed".to_string()
}

pub fn default_rating() -> String {
    "0.0".to_string()
}

pub fn default_coop() -> String {
    "Unknown".to_string()
}

pub fn default_completion() -> String {
    "Unknown".to_string()
}
