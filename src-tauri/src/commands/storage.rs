use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::models::game::Game;
use serde::Serialize;
use tauri::{AppHandle, Manager};

const MAX_SCAN_DEPTH: usize = 4;
const MAX_DISCOVERED_GAMES: usize = 32;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScannedGameCandidate {
    pub id: String,
    pub title: String,
    pub path: String,
    pub platform: String,
}

fn games_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Could not resolve app data directory: {error}"))?;

    fs::create_dir_all(&app_data_dir)
        .map_err(|error| format!("Could not create app data directory: {error}"))?;

    Ok(app_data_dir.join("games.json"))
}

fn legacy_games_file_path() -> Result<PathBuf, String> {
    let current_dir =
        std::env::current_dir().map_err(|error| format!("Could not read current directory: {error}"))?;
    Ok(current_dir.join("games.json"))
}

fn ensure_games_file_exists(app: &AppHandle) -> Result<PathBuf, String> {
    let file_path = games_file_path(app)?;

    if !file_path.exists() {
        let legacy_path = legacy_games_file_path()?;

        if legacy_path.exists() {
            fs::copy(&legacy_path, &file_path)
                .map_err(|error| format!("Could not migrate existing games.json: {error}"))?;
        } else {
            let sample_games = sample_games();
            let json = serde_json::to_string_pretty(&sample_games)
                .map_err(|error| format!("Could not serialize sample games: {error}"))?;

            fs::write(&file_path, json).map_err(|error| format!("Could not create games.json: {error}"))?;
        }
    }

    Ok(file_path)
}

fn read_games_from_disk(app: &AppHandle) -> Result<Vec<Game>, String> {
    let file_path = ensure_games_file_exists(app)?;
    let file_contents =
        fs::read_to_string(&file_path).map_err(|error| format!("Could not read games.json: {error}"))?;

    serde_json::from_str(&file_contents).map_err(|error| format!("Could not parse games.json: {error}"))
}

fn write_games_to_disk(app: &AppHandle, games: &[Game]) -> Result<String, String> {
    let file_path = games_file_path(app)?;

    let json = serde_json::to_string_pretty(games)
        .map_err(|error| format!("Could not serialize game list: {error}"))?;

    fs::write(&file_path, json).map_err(|error| format!("Could not write games.json: {error}"))?;

    Ok(format!("Saved {} game(s) to {}", games.len(), file_path.display()))
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

fn env_path(name: &str) -> Option<PathBuf> {
    std::env::var_os(name).map(PathBuf::from)
}

fn collect_scan_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();

    if let Some(user_profile) = env_path("USERPROFILE") {
        roots.push(user_profile.join("Games"));
    }

    if let Some(public_path) = env_path("PUBLIC") {
        roots.push(public_path.join("Games"));
    }

    for drive in ["C", "D", "E", "F", "G"] {
        roots.push(PathBuf::from(format!(r"{drive}:\Games")));
        roots.push(PathBuf::from(format!(r"{drive}:\XboxGames")));
        roots.push(PathBuf::from(format!(r"{drive}:\GOG Games")));
        roots.push(PathBuf::from(format!(r"{drive}:\Ubisoft")));
    }

    let mut unique = Vec::new();
    let mut seen = HashSet::new();

    for root in roots {
        let normalized = root.to_string_lossy().to_lowercase();
        if seen.insert(normalized) {
            unique.push(root);
        }
    }

    unique
}

fn normalized_path(path: &Path) -> String {
    path.to_string_lossy().replace('/', "\\").to_lowercase()
}

fn should_skip_directory(path: &Path) -> bool {
    let normalized = normalized_path(path);

    [
        "\\steamapps\\",
        "\\epic games\\",
        "\\epicgameslauncher\\",
        "\\commonredist\\",
        "\\redistributables\\",
        "\\installer\\",
        "\\installers\\",
        "\\_commonredist\\",
        "\\cache\\",
        "\\temp\\",
        "\\appdata\\",
        "\\microsoft\\",
        "\\windowsapps\\",
    ]
    .iter()
    .any(|needle| normalized.contains(needle))
}

fn should_skip_executable(path: &Path) -> bool {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_lowercase();

    if file_name.is_empty() {
        return true;
    }

    let normalized = normalized_path(path);

    if ["\\steamapps\\", "\\epic games\\", "\\epicgameslauncher\\"]
        .iter()
        .any(|needle| normalized.contains(needle))
    {
        return true;
    }

    let blocked_names = [
        "unins",
        "uninstall",
        "setup",
        "launcher",
        "crash",
        "report",
        "updater",
        "update",
        "vc_redist",
        "dxsetup",
        "benchmark",
        "config",
        "helper",
        "support",
        "service",
        "cef",
        "eadesktop",
        "origin",
        "acrobat",
        "acrocef",
        "acrobroker",
        "acrobatinfo",
    ];

    blocked_names.iter().any(|needle| file_name.contains(needle))
}

fn title_from_path(path: &Path) -> String {
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("Unknown Game");

    stem.replace('_', " ").replace('-', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    let mut title = first.to_uppercase().to_string();
                    title.push_str(&chars.as_str().to_lowercase());
                    title
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut last_dash = false;

    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash {
            slug.push('-');
            last_dash = true;
        }
    }

    slug.trim_matches('-').to_string()
}

fn build_candidate(path: &Path) -> ScannedGameCandidate {
    let title = title_from_path(path);

    ScannedGameCandidate {
        id: format!("local-{}", slugify(&title)),
        title,
        path: path.to_string_lossy().replace('/', "\\"),
        platform: "local".into(),
    }
}

fn path_has_allowed_root(path: &Path) -> bool {
    let normalized = normalized_path(path);

    ["\\games\\", "\\xboxgames\\", "\\gog games\\", "\\ubisoft\\"]
        .iter()
        .any(|needle| normalized.contains(needle))
}

fn file_name_lowercase(path: &Path) -> String {
    path.file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_lowercase()
}

fn stems_look_related(path: &Path, directory: &Path) -> bool {
    let file_stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_lowercase()
        .replace('-', "")
        .replace('_', "")
        .replace(' ', "");
    let dir_name = directory
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_lowercase()
        .replace('-', "")
        .replace('_', "")
        .replace(' ', "");

    !file_stem.is_empty()
        && !dir_name.is_empty()
        && (file_stem.contains(&dir_name) || dir_name.contains(&file_stem))
}

fn directory_has_game_markers(directory: &Path, exe_name: &str) -> bool {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(_) => return false,
    };

    let exe_stem = Path::new(exe_name)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_lowercase();

    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = file_name_lowercase(&path);

        if path.is_file() {
            if file_name.ends_with(".pak")
                || file_name == "unityplayer.dll"
                || file_name == "gameassembly.dll"
                || file_name.ends_with(".uproject")
            {
                return true;
            }
        }

        if path.is_dir() {
            if file_name == "content"
                || file_name == "engine"
                || file_name == "paks"
                || file_name == "mono"
                || file_name == "monobleedingedge"
                || file_name == "binaries"
                || file_name == "win64"
                || (!exe_stem.is_empty() && file_name == format!("{exe_stem}_data"))
            {
                return true;
            }
        }
    }

    false
}

fn has_game_signature(path: &Path) -> bool {
    let exe_name = file_name_lowercase(path);
    let Some(parent) = path.parent() else {
        return false;
    };

    if directory_has_game_markers(parent, &exe_name) {
        return true;
    }

    if let Some(grandparent) = parent.parent() {
        if directory_has_game_markers(grandparent, &exe_name) {
            return true;
        }
    }

    false
}

fn executable_score(path: &Path) -> i32 {
    let mut score = 0;

    if path_has_allowed_root(path) {
        score += 3;
    }

    if has_game_signature(path) {
        score += 4;
    }

    if let Some(parent) = path.parent() {
        if stems_look_related(path, parent) {
            score += 2;
        }

        let parent_name = file_name_lowercase(parent);
        if ["binaries", "win64", "shipping"].iter().any(|needle| parent_name.contains(needle)) {
            score += 1;
        }

        if let Some(grandparent) = parent.parent() {
            if stems_look_related(path, grandparent) {
                score += 1;
            }
        }
    }

    let file_name = file_name_lowercase(path);
    if file_name.contains("shipping") || file_name.contains("game") {
        score += 1;
    }

    score
}

fn walk_for_games(
    root: &Path,
    depth: usize,
    seen_paths: &mut HashSet<String>,
    seen_ids: &mut HashSet<String>,
    discovered: &mut Vec<ScannedGameCandidate>,
) {
    if depth > MAX_SCAN_DEPTH || discovered.len() >= MAX_DISCOVERED_GAMES || !root.exists() {
        return;
    }

    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        if discovered.len() >= MAX_DISCOVERED_GAMES {
            return;
        }

        let path = entry.path();
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(_) => continue,
        };

        if file_type.is_dir() {
            if should_skip_directory(&path) {
                continue;
            }

            walk_for_games(&path, depth + 1, seen_paths, seen_ids, discovered);
            continue;
        }

        if !file_type.is_file() {
            continue;
        }

        let extension = path.extension().and_then(|value| value.to_str()).unwrap_or_default();
        if !extension.eq_ignore_ascii_case("exe") || should_skip_executable(&path) {
            continue;
        }

        let normalized = normalized_path(&path);
        let score = executable_score(&path);

        if score < 5 {
            continue;
        }

        let candidate = build_candidate(&path);
        if seen_paths.insert(normalized) && seen_ids.insert(candidate.id.to_lowercase()) {
            discovered.push(candidate);
        }
    }
}

#[tauri::command]
pub fn read_games(app: AppHandle) -> Result<Vec<Game>, String> {
    read_games_from_disk(&app)
}

#[tauri::command]
pub fn write_games(app: AppHandle, games: Vec<Game>) -> Result<String, String> {
    write_games_to_disk(&app, &games)
}

#[tauri::command]
pub fn scan_local_games(app: AppHandle) -> Result<Vec<ScannedGameCandidate>, String> {
    let current_games = read_games_from_disk(&app)?;
    let mut discovered = Vec::new();
    let mut seen_paths = HashSet::new();
    let mut seen_ids = HashSet::new();

    for game in &current_games {
        seen_paths.insert(game.exe_path.to_lowercase());
        seen_ids.insert(game.id.to_lowercase());
    }

    for root in collect_scan_roots() {
        walk_for_games(&root, 0, &mut seen_paths, &mut seen_ids, &mut discovered);

        if discovered.len() >= MAX_DISCOVERED_GAMES {
            break;
        }
    }

    Ok(discovered)
}
