use std::fs;
use std::path::PathBuf;

use crate::models::game::Game;

fn games_file_path() -> Result<PathBuf, String> {
    let current_dir = std::env::current_dir().map_err(|error| format!("Could not read current directory: {error}"))?;
    Ok(current_dir.join("games.json"))
}

fn ensure_games_file_exists() -> Result<PathBuf, String> {
    let file_path = games_file_path()?;

    if !file_path.exists() {
        let sample_games = sample_games();
        let json = serde_json::to_string_pretty(&sample_games)
            .map_err(|error| format!("Could not serialize sample games: {error}"))?;

        fs::write(&file_path, json).map_err(|error| format!("Could not create games.json: {error}"))?;
    }

    Ok(file_path)
}

fn sample_games() -> Vec<Game> {
    vec![
        Game {
            id: "game-hades".into(),
            title: "Hades".into(),
            exe_path: r"C:\Games\Hades\Hades.exe".into(),
            cover_art: "https://images.igdb.com/igdb/image/upload/t_cover_big/co2mvt.png".into(),
            platform: "steam".into(),
            total_playtime: 1240,
            last_played: Some("2026-03-28T19:30:00Z".into()),
            status: "playing".into(),
            genres: vec!["Roguelike".into(), "Action".into()],
            description: "Fight your way out of the Underworld in fast, replayable runs.".into(),
        },
        Game {
            id: "game-celeste".into(),
            title: "Celeste".into(),
            exe_path: r"C:\Games\Celeste\Celeste.exe".into(),
            cover_art: "https://images.igdb.com/igdb/image/upload/t_cover_big/co5q3r.png".into(),
            platform: "gog".into(),
            total_playtime: 640,
            last_played: Some("2026-03-10T12:15:00Z".into()),
            status: "completed".into(),
            genres: vec!["Platformer".into(), "Indie".into()],
            description: "A precise climbing platformer with a great story and soundtrack.".into(),
        },
        Game {
            id: "game-witcher-3".into(),
            title: "The Witcher 3".into(),
            exe_path: r"D:\Games\TheWitcher3\bin\x64\witcher3.exe".into(),
            cover_art: "https://images.igdb.com/igdb/image/upload/t_cover_big/co1wyy.png".into(),
            platform: "epic".into(),
            total_playtime: 3120,
            last_played: Some("2026-02-21T21:05:00Z".into()),
            status: "installed".into(),
            genres: vec!["RPG".into(), "Open World".into()],
            description: "A large story-driven RPG with exploration, combat, and quests.".into(),
        },
        Game {
            id: "game-hollow-knight".into(),
            title: "Hollow Knight".into(),
            exe_path: r"C:\Games\HollowKnight\Hollow Knight.exe".into(),
            cover_art: "https://images.igdb.com/igdb/image/upload/t_cover_big/co1rgi.png".into(),
            platform: "pc".into(),
            total_playtime: 0,
            last_played: None,
            status: "backlog".into(),
            genres: vec!["Metroidvania".into(), "Action".into()],
            description: "A moody exploration game with challenging combat and beautiful art.".into(),
        },
    ]
}

#[tauri::command]
pub fn read_games() -> Result<Vec<Game>, String> {
    let file_path = ensure_games_file_exists()?;

    let file_contents =
        fs::read_to_string(&file_path).map_err(|error| format!("Could not read games.json: {error}"))?;

    serde_json::from_str(&file_contents).map_err(|error| format!("Could not parse games.json: {error}"))
}

#[tauri::command]
pub fn write_games(games: Vec<Game>) -> Result<String, String> {
    let file_path = games_file_path()?;

    let json = serde_json::to_string_pretty(&games)
        .map_err(|error| format!("Could not serialize game list: {error}"))?;

    fs::write(&file_path, json).map_err(|error| format!("Could not write games.json: {error}"))?;

    Ok(format!("Saved {} game(s) to {}", games.len(), file_path.display()))
}
