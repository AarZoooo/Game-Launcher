use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::db::{database, games as game_db};
use crate::media::resolver;
use crate::models::game::Game;
use serde::Serialize;
use tauri::AppHandle;

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

#[derive(Debug, Clone)]
struct CandidateMatch {
    candidate: ScannedGameCandidate,
    score: i32,
}

fn read_games_from_database(app: &AppHandle) -> Result<Vec<Game>, String> {
    let connection = database::open_database(app)?;
    let games = game_db::get_all_games(&connection)?;
    resolver::enrich_games(app, games)
}

fn refresh_games_media_from_database(app: &AppHandle) -> Result<Vec<Game>, String> {
    let connection = database::open_database(app)?;
    let games = game_db::get_all_games(&connection)?;
    resolver::force_refresh_games(app, games)
}

fn write_games_to_database(app: &AppHandle, games: &[Game]) -> Result<String, String> {
    let enriched_games = resolver::enrich_games(app, games.to_vec())?;
    let mut connection = database::open_database(app)?;
    game_db::sync_installed_games(&mut connection, &enriched_games)?;

    let database_path = database::database_path(app)?;
    Ok(format!(
        "Saved {} game(s) to {}",
        enriched_games.len(),
        database_path.display()
    ))
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
        roots.push(PathBuf::from(format!(r"{drive}:\Installed")));
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
    let directory_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_lowercase();

    let blocked_segments = [
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
        "\\engine\\extras\\",
        "\\engine\\binaries\\thirdparty\\",
        "\\_redist\\",
        "\\redist\\",
        "\\redistributable\\",
        "\\prereq\\",
        "\\prerequisites\\",
        "\\easyanticheat\\",
    ];

    if blocked_segments.iter().any(|needle| normalized.contains(needle)) {
        return true;
    }

    [
        "engine",
        "binaries",
        "win64",
        "win32",
        "shipping",
        "launcher",
        "redistributable",
        "redist",
        "prereq",
        "prerequisites",
        "easyanticheat",
        "thirdparty",
        "extras",
        "debug",
        "support",
        "tools",
        "installer",
        "installers",
    ]
    .iter()
    .any(|needle| directory_name == *needle)
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

    if [
        "\\steamapps\\",
        "\\epic games\\",
        "\\epicgameslauncher\\",
        "\\engine\\",
        "\\binaries\\",
        "\\win64\\",
        "\\win32\\",
        "\\_redist\\",
        "\\redist\\",
        "\\redistributable\\",
        "\\prereq\\",
        "\\prerequisites\\",
        "\\easyanticheat\\",
    ]
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
        "quicksfv",
        "eac",
        "anticheat",
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

fn normalized_name(value: &str) -> String {
    value
        .to_lowercase()
        .replace('-', "")
        .replace('_', "")
        .replace(' ', "")
}

fn is_generic_container_name(name: &str) -> bool {
    let normalized = normalized_name(name);

    if normalized.is_empty() {
        return true;
    }

    let months = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];

    let generic = [
        "games",
        "installed",
        "install",
        "library",
        "libraries",
        "collection",
        "collections",
        "downloads",
        "downloaded",
        "new",
        "old",
        "backup",
        "backups",
        "setup",
        "bin",
        "binaries",
        "win64",
        "win32",
        "shipping",
        "content",
        "engine",
        "rpg",
        "action",
        "adventure",
        "indie",
        "strategy",
        "simulation",
        "sandbox",
    ];

    normalized.chars().all(|char| char.is_ascii_digit())
        || normalized.len() == 4 && normalized.starts_with("20") && normalized.chars().all(|char| char.is_ascii_digit())
        || months.contains(&normalized.as_str())
        || generic.contains(&normalized.as_str())
}

fn path_components_below_allowed_root(path: &Path) -> Option<Vec<String>> {
    let scan_root = scan_root_for_path(path)?;
    let exe_dir = path.parent()?;
    let relative = exe_dir.strip_prefix(scan_root).ok()?;

    Some(
        relative
            .components()
            .map(|component| component.as_os_str().to_string_lossy().to_string())
            .collect(),
    )
}

fn install_root_for_path(path: &Path) -> Option<PathBuf> {
    let exe_dir = path.parent()?;
    let below_allowed_root = path_components_below_allowed_root(path)?;
    let file_stem = normalized_name(
        path.file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or_default(),
    );

    let mut best: Option<(PathBuf, i32)> = None;

    for ancestor in exe_dir.ancestors() {
        let name = ancestor
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default();

        if name.is_empty() {
            continue;
        }

        let matches_scope = below_allowed_root
            .iter()
            .any(|component| normalized_name(component) == normalized_name(name));

        if !matches_scope {
            continue;
        }

        let mut score = 0;
        let normalized_ancestor = normalized_name(name);

        if !is_generic_container_name(name) {
            score += 4;
        } else {
            score -= 4;
        }

        if !file_stem.is_empty()
            && (file_stem.contains(&normalized_ancestor) || normalized_ancestor.contains(&file_stem))
        {
            score += 5;
        }

        let distance = exe_dir
            .strip_prefix(ancestor)
            .ok()
            .map(|relative| relative.components().count())
            .unwrap_or(0);

        if distance == 0 {
            score += 4;
        } else if distance == 1 {
            score += 3;
        } else if distance == 2 {
            score += 1;
        } else {
            score -= distance as i32;
        }

        if directory_has_game_markers(ancestor, "") {
            score += 4;
        }

        match &best {
            Some((_, best_score)) if *best_score >= score => {}
            _ => best = Some((ancestor.to_path_buf(), score)),
        }
    }

    best.map(|(path, _)| path)
}

fn title_from_install_root(path: &Path, install_root: Option<&Path>) -> String {
    if let Some(root) = install_root {
        if let Some(name) = root.file_name().and_then(|value| value.to_str()) {
            if !name.trim().is_empty() {
                return name.replace('_', " ").replace('-', " ");
            }
        }
    }

    title_from_path(path)
}

fn build_candidate(path: &Path, install_root: Option<&Path>) -> ScannedGameCandidate {
    let title = title_from_install_root(path, install_root);

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

fn scan_root_for_path(path: &Path) -> Option<PathBuf> {
    let normalized = normalized_path(path);

    collect_scan_roots()
        .into_iter()
        .filter(|root| normalized.starts_with(&normalized_path(root)))
        .max_by_key(|root| normalized_path(root).len())
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
        .to_string();
    let dir_name = directory
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_string();

    let file_stem = normalized_name(&file_stem);
    let dir_name = normalized_name(&dir_name);

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
    let install_root = install_root_for_path(path);

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

    if let Some(root) = install_root.as_deref() {
        if stems_look_related(path, root) {
            score += 3;
        }

        if let Some(exe_dir) = path.parent() {
            let distance = exe_dir
                .strip_prefix(root)
                .ok()
                .map(|relative| relative.components().count())
                .unwrap_or(0);

            if distance == 0 {
                score += 4;
            } else if distance == 1 {
                score += 2;
            } else {
                score -= distance as i32;
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
    best_by_root: &mut HashMap<String, CandidateMatch>,
) {
    if depth > MAX_SCAN_DEPTH || best_by_root.len() >= MAX_DISCOVERED_GAMES || !root.exists() {
        return;
    }

    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        if best_by_root.len() >= MAX_DISCOVERED_GAMES {
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

            walk_for_games(&path, depth + 1, seen_paths, best_by_root);
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

        if !seen_paths.insert(normalized) {
            continue;
        }

        let install_root = install_root_for_path(&path);
        let root_key = install_root
            .as_deref()
            .map(normalized_path)
            .unwrap_or_else(|| normalized_path(&path));
        let candidate = build_candidate(&path, install_root.as_deref());

        match best_by_root.get(&root_key) {
            Some(existing) if existing.score >= score => {}
            _ => {
                best_by_root.insert(root_key, CandidateMatch { candidate, score });
            }
        }
    }
}

#[tauri::command]
pub fn read_games(app: AppHandle) -> Result<Vec<Game>, String> {
    read_games_from_database(&app)
}

#[tauri::command]
pub fn write_games(app: AppHandle, games: Vec<Game>) -> Result<String, String> {
    write_games_to_database(&app, &games)
}

#[tauri::command]
pub fn scan_local_games(app: AppHandle) -> Result<Vec<ScannedGameCandidate>, String> {
    let current_games = read_games_from_database(&app)?;
    let mut seen_paths = HashSet::new();
    let mut best_by_root = HashMap::new();

    for game in &current_games {
        seen_paths.insert(game.exe_path.to_lowercase());
    }

    for root in collect_scan_roots() {
        walk_for_games(&root, 0, &mut seen_paths, &mut best_by_root);

        if best_by_root.len() >= MAX_DISCOVERED_GAMES {
            break;
        }
    }

    let mut discovered = best_by_root
        .into_values()
        .map(|item| item.candidate)
        .collect::<Vec<_>>();

    discovered.sort_by(|left, right| left.title.cmp(&right.title));

    Ok(discovered)
}

#[tauri::command]
pub fn refresh_installed_game_media(app: AppHandle) -> Result<Vec<Game>, String> {
    refresh_games_media_from_database(&app)
}
