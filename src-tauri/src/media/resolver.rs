use tauri::AppHandle;

use crate::db::{database, games as game_db};
use crate::models::game::Game;

use super::fallbacks::local_files;
use super::matching::title::normalize_title;
use super::placeholders::{PlaceholderKind, placeholder_data_url};
use super::providers::igdb;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct ResolvedMedia {
    cover_vertical: Option<String>,
    cover_horizontal: Option<String>,
    banner: Option<String>,
    icon: Option<String>,
    accent_color: Option<String>,
    genres: Vec<String>,
    description: Option<String>,
}

fn accent_for_platform(platform: &str) -> Option<String> {
    match platform.to_ascii_lowercase().as_str() {
        "steam" => Some("#c9a35d".to_string()),
        "epic" => Some("#b7bcc7".to_string()),
        "local" | "pc" => Some("#889c58".to_string()),
        _ => None,
    }
}

fn is_missing(value: Option<&String>) -> bool {
    value.map(|item| item.trim().is_empty()).unwrap_or(true)
}

fn has_resolved_media(game: &Game) -> bool {
    !is_missing(game.cover_vertical.as_ref())
        && !is_missing(game.cover_horizontal.as_ref())
}

fn apply_provider_media(game: &Game, media: &mut ResolvedMedia) {
    if let Ok(Some(best_match)) = igdb::search_best_match(&game.title) {
        println!(
            "[media] igdb match title='{}' matched='{}' score={}",
            game.title, best_match.name, best_match.score
        );

        media.cover_vertical = best_match.cover_url.clone();
        media.cover_horizontal = best_match
            .screenshot_urls
            .first()
            .cloned()
            .or_else(|| best_match.cover_url.clone());
        media.genres = best_match.genres.clone();
        media.description = best_match.summary.clone();
    }
}

fn merge_media(mut game: Game, media: ResolvedMedia) -> Game {
    if game.cover_vertical.as_deref().map(str::is_empty).unwrap_or(true) {
        game.cover_vertical = media.cover_vertical;
    }
    if game.cover_horizontal.as_deref().map(str::is_empty).unwrap_or(true) {
        game.cover_horizontal = media.cover_horizontal;
    }
    if game.icon.as_deref().map(str::is_empty).unwrap_or(true) {
        game.icon = media.icon;
    }
    if game.accent_color.as_deref().map(str::is_empty).unwrap_or(true) {
        game.accent_color = media.accent_color.or_else(|| accent_for_platform(&game.platform));
    }
    if game.cover_art.trim().is_empty() {
        game.cover_art = game.cover_vertical.clone().unwrap_or_default();
    }
    if game.genres.is_empty() && !media.genres.is_empty() {
        game.genres = media.genres;
    }
    if game.description.trim().is_empty() {
        game.description = media.description.unwrap_or_default();
    }

    game.cover_vertical
        .get_or_insert_with(|| placeholder_data_url(&game.title, PlaceholderKind::Vertical));
    game.cover_horizontal
        .get_or_insert_with(|| placeholder_data_url(&game.title, PlaceholderKind::Horizontal));
    if game.banner.as_deref().map(str::is_empty).unwrap_or(true) {
        game.banner = game.cover_horizontal.clone();
    }

    if game.cover_art.trim().is_empty() {
        game.cover_art = game.cover_vertical.clone().unwrap_or_default();
    }

    game
}

fn resolve_missing_media(game: &Game) -> Game {
    let mut media = ResolvedMedia::default();

    if !game.cover_art.trim().is_empty() && is_missing(game.cover_vertical.as_ref()) {
        media.cover_vertical = Some(game.cover_art.clone());
    }

    apply_provider_media(game, &mut media);

    let local_media = local_files::find_local_media(&game.exe_path, &game.title);
    if media.cover_vertical.is_none() {
        media.cover_vertical = local_media.cover_vertical;
    }
    if media.cover_horizontal.is_none() {
        media.cover_horizontal = local_media.cover_horizontal;
    }
    if media.banner.is_none() {
        media.banner = local_media.banner;
    }
    if media.icon.is_none() {
        media.icon = local_media.icon;
    }

    media.accent_color = accent_for_platform(&game.platform);

    merge_media(game.clone(), media)
}

fn enrich_games_inner(app: &AppHandle, games: Vec<Game>, force_refresh: bool) -> Result<Vec<Game>, String> {
    let connection = database::open_database(app)?;
    let mut enriched = Vec::with_capacity(games.len());

    for game in games {
        let needs_resolution = force_refresh
            || !has_resolved_media(&game)
            || game.cover_art.trim().is_empty()
            || (game.genres.is_empty() && normalize_title(&game.title) != "uncategorized");
        let resolved = if needs_resolution {
            let updated_game = resolve_missing_media(&game);
            if updated_game != game {
                game_db::update_game_media(&connection, &updated_game)?;
            }
            updated_game
        } else {
            game
        };
        enriched.push(resolved);
    }

    Ok(enriched)
}

pub fn enrich_games(app: &AppHandle, games: Vec<Game>) -> Result<Vec<Game>, String> {
    enrich_games_inner(app, games, false)
}

pub fn force_refresh_games(app: &AppHandle, games: Vec<Game>) -> Result<Vec<Game>, String> {
    enrich_games_inner(app, games, true)
}

#[cfg(test)]
mod tests {
    use super::{ResolvedMedia, merge_media};
    use crate::models::game::Game;

    fn empty_game() -> Game {
        Game {
            id: "test".into(),
            title: "Test Game".into(),
            exe_path: "C:\\Games\\Test\\game.exe".into(),
            cover_art: String::new(),
            cover_vertical: None,
            cover_horizontal: None,
            banner: None,
            icon: None,
            accent_color: None,
            platform: "local".into(),
            total_playtime: 0,
            last_played: None,
            status: "installed".into(),
            genres: Vec::new(),
            description: String::new(),
        }
    }

    #[test]
    fn falls_back_to_placeholders_when_no_media_exists() {
        let game = merge_media(empty_game(), ResolvedMedia::default());
        assert!(game.cover_vertical.unwrap().starts_with("data:image/svg+xml;base64,"));
        assert!(game.cover_horizontal.unwrap().starts_with("data:image/svg+xml;base64,"));
        assert!(game.banner.unwrap().starts_with("data:image/svg+xml;base64,"));
    }

    #[test]
    fn preserves_existing_backend_media() {
        let mut game = empty_game();
        game.cover_vertical = Some("https://example.com/cover.png".into());
        let merged = merge_media(game, ResolvedMedia::default());
        assert_eq!(merged.cover_vertical.as_deref(), Some("https://example.com/cover.png"));
    }
}
