use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};
use std::thread;

use tauri::AppHandle;

use crate::db::{database, games as game_db};
use crate::models::game::Game;

use super::events::{emit_game_media_resolution_state, emit_game_media_updated};
use super::fallbacks::local_files;
use super::matching::title::normalize_title;
use super::placeholders::{PlaceholderKind, placeholder_data_url};
use super::providers::igdb;
use crate::models::game::{default_completion, default_coop, default_rating};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct ResolvedMedia {
    cover_vertical: Option<String>,
    cover_horizontal: Option<String>,
    banner: Option<String>,
    icon: Option<String>,
    provider_cover_vertical: bool,
    provider_cover_horizontal: bool,
    provider_banner: bool,
    accent_color: Option<String>,
    genres: Vec<String>,
    description: Option<String>,
    rating: Option<String>,
    coop: Option<String>,
    completion: Option<String>,
}

static MEDIA_RESOLUTION_TASKS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn media_resolution_tasks() -> &'static Mutex<HashSet<String>> {
    MEDIA_RESOLUTION_TASKS.get_or_init(|| Mutex::new(HashSet::new()))
}

fn media_resolution_key(game_id: &str, force_refresh: bool) -> String {
    format!("{game_id}:{force_refresh}")
}

fn claim_media_resolution(game_id: &str, force_refresh: bool) -> bool {
    let key = media_resolution_key(game_id, force_refresh);
    let mut tasks = media_resolution_tasks()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    tasks.insert(key)
}

fn release_media_resolution(game_id: &str, force_refresh: bool) {
    let key = media_resolution_key(game_id, force_refresh);
    let mut tasks = media_resolution_tasks()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    tasks.remove(&key);
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

fn is_placeholder_media(value: &str) -> bool {
    value.starts_with("data:image/svg+xml;base64,")
}

fn is_remote_media(value: &str) -> bool {
    value.starts_with("http://") || value.starts_with("https://")
}

fn should_replace_media(value: Option<&String>) -> bool {
    value
        .map(|item| item.trim().is_empty() || is_placeholder_media(item))
        .unwrap_or(true)
}

fn has_default_genres(genres: &[String]) -> bool {
    genres.is_empty()
        || genres.iter().all(|genre| genre.trim().is_empty())
        || genres.len() == 1
            && matches!(
                genres[0].trim().to_ascii_lowercase().as_str(),
                "uncategorized" | "unknown"
            )
}

fn has_resolved_media(game: &Game) -> bool {
    !should_replace_media(game.cover_vertical.as_ref())
        && !should_replace_media(game.cover_horizontal.as_ref())
}

fn has_provider_priority_media(game: &Game) -> bool {
    game.cover_vertical
        .as_deref()
        .map(is_remote_media)
        .unwrap_or(false)
        && game
            .cover_horizontal
            .as_deref()
            .map(is_remote_media)
            .unwrap_or(false)
}

fn has_default_rating(rating: &str) -> bool {
    let trimmed = rating.trim();
    trimmed.is_empty() || trimmed == default_rating()
}

fn has_default_coop(coop: &str) -> bool {
    let trimmed = coop.trim();
    trimmed.is_empty() || trimmed.eq_ignore_ascii_case(&default_coop())
}

fn has_default_completion(completion: &str) -> bool {
    let trimmed = completion.trim();
    trimmed.is_empty() || trimmed.eq_ignore_ascii_case(&default_completion())
}

fn media_query_signature(game: &Game) -> String {
    format!(
        "{}|{}",
        normalize_title(&game.title),
        game.exe_path.trim().replace('/', "\\").to_ascii_lowercase()
    )
}

fn needs_media_resolution(game: &Game, force_refresh: bool) -> bool {
    if force_refresh {
        return true;
    }

    if !has_resolved_media(game) {
        return true;
    }

    if !has_provider_priority_media(game) {
        return true;
    }

    game
        .media_query_signature
        .as_deref()
        .map(|signature| signature != media_query_signature(game))
        .unwrap_or(false)
}

fn apply_provider_media(game: &Game, media: &mut ResolvedMedia) {
    if let Ok(Some(best_match)) = igdb::search_best_match(&game.title) {
        media.cover_vertical = best_match.cover_url.clone();
        media.provider_cover_vertical = media.cover_vertical.is_some();
        media.cover_horizontal = best_match
            .artwork_urls
            .first()
            .cloned()
            .or_else(|| {
                best_match
            .screenshot_urls
            .first()
            .cloned()
            })
            .or_else(|| best_match.cover_url.clone());
        media.provider_cover_horizontal = media.cover_horizontal.is_some();
        media.banner = best_match
            .artwork_urls
            .first()
            .cloned()
            .or_else(|| media.cover_horizontal.clone());
        media.provider_banner = media.banner.is_some();
        media.genres = best_match.genres.clone();
        media.description = best_match.summary.clone();
        media.rating = Some(best_match.rating.clone());
        media.coop = Some(best_match.coop.clone());
        media.completion = Some(best_match.completion.clone());
    }
}

fn finalize_media(game: &mut Game) {
    game.cover_vertical
        .get_or_insert_with(|| placeholder_data_url(&game.title, PlaceholderKind::Vertical));
    game.cover_horizontal
        .get_or_insert_with(|| placeholder_data_url(&game.title, PlaceholderKind::Horizontal));
    if should_replace_media(game.banner.as_ref()) {
        game.banner = game.cover_horizontal.clone();
    }
    if game.cover_art.trim().is_empty() || is_placeholder_media(&game.cover_art) {
        game.cover_art = game.cover_vertical.clone().unwrap_or_default();
    }
    if game.accent_color.as_deref().map(str::is_empty).unwrap_or(true) {
        game.accent_color = accent_for_platform(&game.platform);
    }
    if has_default_rating(&game.rating) {
        game.rating = default_rating();
    }
    if has_default_coop(&game.coop) {
        game.coop = default_coop();
    }
    if has_default_completion(&game.completion) {
        game.completion = default_completion();
    }
}

fn merge_media(mut game: Game, media: ResolvedMedia) -> Game {
    if media.provider_cover_vertical || should_replace_media(game.cover_vertical.as_ref()) {
        game.cover_vertical = media.cover_vertical;
    }
    if media.provider_cover_horizontal || should_replace_media(game.cover_horizontal.as_ref()) {
        game.cover_horizontal = media.cover_horizontal;
    }
    if media.provider_banner || should_replace_media(game.banner.as_ref()) {
        game.banner = media.banner.or_else(|| game.cover_horizontal.clone());
    }
    if should_replace_media(game.icon.as_ref()) {
        game.icon = media.icon;
    }
    if game.accent_color.as_deref().map(str::is_empty).unwrap_or(true) {
        game.accent_color = media.accent_color;
    }
    if has_default_genres(&game.genres) && !media.genres.is_empty() {
        game.genres = media.genres;
    }
    if game.description.trim().is_empty() {
        game.description = media.description.unwrap_or_default();
    }
    if has_default_rating(&game.rating) {
        game.rating = media.rating.unwrap_or_else(default_rating);
    }
    if has_default_coop(&game.coop) {
        game.coop = media.coop.unwrap_or_else(default_coop);
    }
    if has_default_completion(&game.completion) {
        game.completion = media.completion.unwrap_or_else(default_completion);
    }

    finalize_media(&mut game);
    game
}

fn overwrite_media(mut game: Game, media: ResolvedMedia) -> Game {
    game.cover_vertical = media.cover_vertical;
    game.cover_horizontal = media.cover_horizontal;
    game.banner = game.cover_horizontal.clone();
    game.icon = media.icon;
    game.accent_color = media.accent_color.or_else(|| accent_for_platform(&game.platform));
    if !media.genres.is_empty() {
        game.genres = media.genres;
    }
    if let Some(description) = media.description {
        if !description.trim().is_empty() {
            game.description = description;
        }
    }
    game.rating = media.rating.unwrap_or_else(default_rating);
    game.coop = media.coop.unwrap_or_else(default_coop);
    game.completion = media.completion.unwrap_or_else(default_completion);

    finalize_media(&mut game);
    game
}

fn resolve_media(game: &Game, force_refresh: bool) -> Game {
    let mut media = ResolvedMedia::default();

    if !force_refresh && !game.cover_art.trim().is_empty() && is_missing(game.cover_vertical.as_ref()) {
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

    let mut resolved = if force_refresh {
        overwrite_media(game.clone(), media)
    } else {
        merge_media(game.clone(), media)
    };
    resolved.media_query_signature = Some(media_query_signature(&resolved));
    resolved
}

fn resolve_and_persist_media(
    app: &AppHandle,
    connection: &rusqlite::Connection,
    game: Game,
    force_refresh: bool,
) -> Result<(), String> {
    let updated_game = resolve_media(&game, force_refresh);

    if updated_game != game {
        game_db::update_game_media(connection, &updated_game)?;
        emit_game_media_updated(app, &updated_game);
    }

    Ok(())
}

pub fn queue_media_resolution(app: AppHandle, games: Vec<Game>, force_refresh: bool) {
    let pending_games = games
        .into_iter()
        .filter(|game| needs_media_resolution(game, force_refresh))
        .filter(|game| claim_media_resolution(&game.id, force_refresh))
        .collect::<Vec<_>>();

    if pending_games.is_empty() {
        return;
    }

    thread::spawn(move || {
        for game in &pending_games {
            emit_game_media_resolution_state(&app, &game.id, &game.exe_path, "started");
        }

        let connection = match database::open_database(&app) {
            Ok(connection) => connection,
            Err(error) => {
                println!("[media] failed to start background resolution: {error}");
                for game in pending_games {
                    emit_game_media_resolution_state(&app, &game.id, &game.exe_path, "finished");
                    release_media_resolution(&game.id, force_refresh);
                }
                return;
            }
        };

        for game in pending_games {
            if let Err(error) = resolve_and_persist_media(&app, &connection, game.clone(), force_refresh)
            {
                println!(
                    "[media] background resolution failed game_id={} title='{}' error={}",
                    game.id, game.title, error
                );
            }

            emit_game_media_resolution_state(&app, &game.id, &game.exe_path, "finished");
            release_media_resolution(&game.id, force_refresh);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::{ResolvedMedia, media_query_signature, merge_media, needs_media_resolution};
    use crate::models::game::{default_completion, default_coop, default_rating};
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
            rating: default_rating(),
            coop: default_coop(),
            completion: default_completion(),
            sessions: Vec::new(),
            media_query_signature: None,
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
        game.cover_horizontal = Some("https://example.com/hero.png".into());
        let merged = merge_media(game, ResolvedMedia::default());
        assert_eq!(merged.cover_vertical.as_deref(), Some("https://example.com/cover.png"));
    }

    #[test]
    fn does_not_trigger_auto_resolution_for_missing_legacy_cover_art_or_genres() {
        let mut game = empty_game();
        game.cover_vertical = Some("https://example.com/cover.png".into());
        game.cover_horizontal = Some("https://example.com/hero.png".into());

        assert!(!needs_media_resolution(&game, false));
    }

    #[test]
    fn retries_placeholder_media_on_future_reads_even_with_same_signature() {
        let mut game = empty_game();
        game.cover_vertical = Some("data:image/svg+xml;base64,placeholder".into());
        game.cover_horizontal = Some("data:image/svg+xml;base64,placeholder".into());
        game.media_query_signature = Some(media_query_signature(&game));

        assert!(needs_media_resolution(&game, false));
    }

    #[test]
    fn retries_when_media_is_missing_even_with_same_signature() {
        let mut game = empty_game();
        game.media_query_signature = Some(media_query_signature(&game));

        assert!(needs_media_resolution(&game, false));
    }

    #[test]
    fn retries_when_current_media_is_local_data_url_even_with_same_signature() {
        let mut game = empty_game();
        game.cover_vertical = Some("data:image/png;base64,localcover".into());
        game.cover_horizontal = Some("data:image/png;base64,localhero".into());
        game.media_query_signature = Some(media_query_signature(&game));

        assert!(needs_media_resolution(&game, false));
    }

    #[test]
    fn retries_when_query_signature_changes_for_resolved_media() {
        let mut game = empty_game();
        game.cover_vertical = Some("https://example.com/cover.png".into());
        game.cover_horizontal = Some("https://example.com/hero.png".into());
        game.media_query_signature = Some("old|signature".into());

        assert!(needs_media_resolution(&game, false));
    }

    #[test]
    fn replaces_placeholder_media_and_default_genres() {
        let mut game = empty_game();
        game.cover_vertical = Some("data:image/svg+xml;base64,placeholder".into());
        game.cover_horizontal = Some("data:image/svg+xml;base64,placeholder".into());
        game.banner = Some("data:image/svg+xml;base64,placeholder".into());
        game.cover_art = "data:image/svg+xml;base64,placeholder".into();
        game.genres = vec!["Uncategorized".into()];

        let merged = merge_media(
            game,
            ResolvedMedia {
                cover_vertical: Some("https://example.com/cover.png".into()),
                cover_horizontal: Some("https://example.com/hero.png".into()),
                banner: Some("https://example.com/banner.png".into()),
                genres: vec!["Factory".into(), "Simulation".into()],
                ..ResolvedMedia::default()
            },
        );

        assert_eq!(merged.cover_vertical.as_deref(), Some("https://example.com/cover.png"));
        assert_eq!(merged.cover_horizontal.as_deref(), Some("https://example.com/hero.png"));
        assert_eq!(merged.banner.as_deref(), Some("https://example.com/banner.png"));
        assert_eq!(merged.cover_art, "https://example.com/cover.png");
        assert_eq!(merged.genres, vec!["Factory".to_string(), "Simulation".to_string()]);
    }

    #[test]
    fn provider_media_replaces_existing_local_fallback_media() {
        let mut game = empty_game();
        game.cover_vertical = Some("data:image/png;base64,localcover".into());
        game.cover_horizontal = Some("data:image/png;base64,localhero".into());
        game.banner = Some("data:image/png;base64,localbanner".into());

        let merged = merge_media(
            game,
            ResolvedMedia {
                cover_vertical: Some("https://example.com/provider-cover.png".into()),
                cover_horizontal: Some("https://example.com/provider-hero.png".into()),
                banner: Some("https://example.com/provider-banner.png".into()),
                provider_cover_vertical: true,
                provider_cover_horizontal: true,
                provider_banner: true,
                ..ResolvedMedia::default()
            },
        );

        assert_eq!(
            merged.cover_vertical.as_deref(),
            Some("https://example.com/provider-cover.png")
        );
        assert_eq!(
            merged.cover_horizontal.as_deref(),
            Some("https://example.com/provider-hero.png")
        );
        assert_eq!(
            merged.banner.as_deref(),
            Some("https://example.com/provider-banner.png")
        );
    }
}
