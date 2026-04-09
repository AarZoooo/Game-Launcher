use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use crate::media::matching::title::{match_score, normalize_title};
use crate::models::game::{default_completion, default_coop, default_rating};

const TWITCH_TOKEN_URL: &str = "https://id.twitch.tv/oauth2/token";
const IGDB_GAMES_URL: &str = "https://api.igdb.com/v4/games";
const IGDB_CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
const IGDB_REQUEST_TIMEOUT: Duration = Duration::from_secs(8);

static SESSION_TWITCH_TOKEN: OnceLock<Mutex<Option<String>>> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IgdbSearchResult {
    pub id: i64,
    pub name: String,
    pub slug: Option<String>,
    pub genres: Vec<String>,
    pub cover_url: Option<String>,
    pub artwork_urls: Vec<String>,
    pub screenshot_urls: Vec<String>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IgdbBestMatch {
    pub name: String,
    pub genres: Vec<String>,
    pub cover_url: Option<String>,
    pub artwork_urls: Vec<String>,
    pub screenshot_urls: Vec<String>,
    pub summary: Option<String>,
    pub rating: String,
    pub coop: String,
    pub completion: String,
    pub score: i32,
}

#[derive(Debug, Deserialize)]
struct TwitchTokenResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct IgdbImage {
    image_id: String,
}

#[derive(Debug, Deserialize)]
struct IgdbGenre {
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct IgdbGameMode {
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct IgdbMultiplayerMode {
    campaigncoop: Option<bool>,
    dropin: Option<bool>,
    lancoop: Option<bool>,
    offlinecoop: Option<bool>,
    offlinecoopmax: Option<i64>,
    onlinecoop: Option<bool>,
    onlinecoopmax: Option<i64>,
    splitscreen: Option<bool>,
    splitscreenonline: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
struct IgdbGameTimeToBeat {
    hastily: Option<i64>,
    normally: Option<i64>,
    completely: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct IgdbGame {
    id: i64,
    name: String,
    slug: Option<String>,
    summary: Option<String>,
    aggregated_rating: Option<f64>,
    cover: Option<IgdbImage>,
    artworks: Option<Vec<IgdbImage>>,
    game_modes: Option<Vec<IgdbGameMode>>,
    genres: Option<Vec<IgdbGenre>>,
    multiplayer_modes: Option<Vec<IgdbMultiplayerMode>>,
    rating: Option<f64>,
    screenshots: Option<Vec<IgdbImage>>,
    total_rating: Option<f64>,
}

fn session_twitch_token() -> &'static Mutex<Option<String>> {
    SESSION_TWITCH_TOKEN.get_or_init(|| Mutex::new(None))
}

#[derive(Debug, Clone)]
struct IgdbSearchCandidate {
    result: IgdbSearchResult,
    aggregated_rating: Option<f64>,
    game_modes: Vec<String>,
    multiplayer_modes: Vec<IgdbMultiplayerMode>,
    rating: Option<f64>,
    total_rating: Option<f64>,
}

fn required_env(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("Missing required environment variable: {name}"))
}

fn twitch_access_token(client: &Client, client_id: &str, client_secret: &str) -> Result<String, String> {
    let response = client
        .post(TWITCH_TOKEN_URL)
        .query(&[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("grant_type", "client_credentials"),
        ])
        .send()
        .map_err(|error| format!("Failed to request Twitch access token: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("Twitch token request failed with {status}: {body}"));
    }

    response
        .json::<TwitchTokenResponse>()
        .map(|payload| payload.access_token)
        .map_err(|error| format!("Failed to parse Twitch access token response: {error}"))
}

fn build_cover_url(image_id: &str) -> String {
    format!("https://images.igdb.com/igdb/image/upload/t_cover_big_2x/{image_id}.png")
}

fn build_screenshot_url(image_id: &str) -> String {
    format!("https://images.igdb.com/igdb/image/upload/t_1080p/{image_id}.png")
}

fn build_artwork_url(image_id: &str) -> String {
    format!("https://images.igdb.com/igdb/image/upload/t_1080p/{image_id}.png")
}

fn games_headers(client_id: &str, access_token: &str) -> Result<HeaderMap, String> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Client-ID",
        HeaderValue::from_str(client_id)
            .map_err(|error| format!("Invalid Twitch client id header value: {error}"))?,
    );
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {access_token}"))
            .map_err(|error| format!("Invalid Twitch authorization header value: {error}"))?,
    );
    Ok(headers)
}

fn split_compound_title(value: &str) -> String {
    let mut result = String::with_capacity(value.len() + 8);
    let mut previous: Option<char> = None;

    for current in value.chars() {
        if let Some(last) = previous {
            let should_insert_space =
                (last.is_ascii_lowercase() && current.is_ascii_uppercase())
                    || (last.is_ascii_alphabetic() && current.is_ascii_digit())
                    || (last.is_ascii_digit() && current.is_ascii_alphabetic());

            if should_insert_space && !result.ends_with(' ') {
                result.push(' ');
            }
        }

        result.push(current);
        previous = Some(current);
    }

    result
}

fn search_variants(title: &str) -> Vec<String> {
    let mut variants = Vec::new();

    for candidate in [
        title.trim().to_string(),
        split_compound_title(title).trim().to_string(),
        normalize_title(&split_compound_title(title)),
    ] {
        if candidate.is_empty() || variants.contains(&candidate) {
            continue;
        }

        variants.push(candidate);
    }

    variants
}

fn build_client_and_headers() -> Result<(Client, HeaderMap), String> {
    let client_id = required_env("TWITCH_CLIENT_ID")?;
    let client_secret = required_env("TWITCH_CLIENT_SECRET")?;
    let client = Client::builder()
        .connect_timeout(IGDB_CONNECT_TIMEOUT)
        .timeout(IGDB_REQUEST_TIMEOUT)
        .build()
        .map_err(|error| format!("Failed to build IGDB HTTP client: {error}"))?;
    let access_token = {
        let mut cached = session_twitch_token()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if let Some(existing) = cached.clone() {
            existing
        } else {
            let fetched = twitch_access_token(&client, &client_id, &client_secret)?;
            *cached = Some(fetched.clone());
            fetched
        }
    };
    let headers = games_headers(&client_id, &access_token)?;
    Ok((client, headers))
}

fn boolish(value: Option<bool>) -> bool {
    value.unwrap_or(false)
}

fn has_local_coop(modes: &[IgdbMultiplayerMode]) -> bool {
    modes.iter().any(|mode| {
        boolish(mode.offlinecoop)
            || boolish(mode.campaigncoop)
            || boolish(mode.lancoop)
            || boolish(mode.splitscreen)
            || boolish(mode.splitscreenonline)
            || mode.offlinecoopmax.unwrap_or_default() > 1
    })
}

fn has_online_coop(modes: &[IgdbMultiplayerMode]) -> bool {
    modes.iter().any(|mode| {
        boolish(mode.onlinecoop)
            || boolish(mode.dropin)
            || mode.onlinecoopmax.unwrap_or_default() > 1
    })
}

fn has_mode(game_modes: &[String], needle: &str) -> bool {
    game_modes
        .iter()
        .any(|mode| mode.eq_ignore_ascii_case(needle))
}

fn format_rating(total_rating: Option<f64>, aggregated_rating: Option<f64>, rating: Option<f64>) -> String {
    let raw_score = total_rating.or(aggregated_rating).or(rating);
    raw_score
        .map(|score| (score / 10.0).clamp(0.0, 10.0))
        .map(|score| format!("{score:.1}"))
        .unwrap_or_else(default_rating)
}

fn derive_coop(game_modes: &[String], multiplayer_modes: &[IgdbMultiplayerMode]) -> String {
    let local_coop = has_local_coop(multiplayer_modes);
    let online_coop = has_online_coop(multiplayer_modes);

    if local_coop && online_coop {
        "Online & Local Co-op".to_string()
    } else if online_coop {
        "Online Co-op".to_string()
    } else if local_coop {
        "Local Co-op".to_string()
    } else if has_mode(game_modes, "Co-operative") {
        "Yes".to_string()
    } else if has_mode(game_modes, "Single player") || has_mode(game_modes, "Multiplayer") {
        "No".to_string()
    } else {
        default_coop()
    }
}

fn format_hours(seconds: i64) -> String {
    let hours = ((seconds as f64) / 3600.0).max(0.5);
    if hours >= 10.0 {
        format!("{hours:.0}h")
    } else {
        format!("{hours:.1}h")
    }
}

fn derive_completion(
    game_modes: &[String],
    genres: &[String],
    time_to_beat: Option<&IgdbGameTimeToBeat>,
    coop: &str,
) -> String {
    if let Some(time_to_beat) = time_to_beat {
        if let Some(seconds) = time_to_beat.normally.filter(|value| *value > 0) {
            return format!("{} main", format_hours(seconds));
        }
        if let Some(seconds) = time_to_beat.completely.filter(|value| *value > 0) {
            return format!("{} 100%", format_hours(seconds));
        }
        if let Some(seconds) = time_to_beat.hastily.filter(|value| *value > 0) {
            return format!("{} rush", format_hours(seconds));
        }
    }

    if game_modes
        .iter()
        .any(|mode| mode.eq_ignore_ascii_case("Battle Royale") || mode.eq_ignore_ascii_case("Massively Multiplayer Online (MMO)"))
    {
        return "Persistent".to_string();
    }

    if genres
        .iter()
        .any(|genre| genre.eq_ignore_ascii_case("Simulator") || genre.eq_ignore_ascii_case("Strategy"))
    {
        return "Sandbox".to_string();
    }

    if has_mode(game_modes, "Single player") {
        if genres
            .iter()
            .any(|genre| genre.eq_ignore_ascii_case("Adventure") || genre.eq_ignore_ascii_case("Role-playing (RPG)"))
        {
            return "Story".to_string();
        }
        return "Single player".to_string();
    }

    if has_mode(game_modes, "Multiplayer")
        || has_mode(game_modes, "Co-operative")
        || !coop.eq_ignore_ascii_case("No")
    {
        return "Multiplayer".to_string();
    }

    default_completion()
}

fn query_time_to_beat(
    client: &Client,
    headers: &HeaderMap,
    game_id: i64,
) -> Result<Option<IgdbGameTimeToBeat>, String> {
    let query = format!(
        "fields game_id,hastily,normally,completely; where game_id = {game_id}; limit 1;"
    );

    let response = client
        .post("https://api.igdb.com/v4/game_time_to_beats")
        .headers(headers.clone())
        .body(query)
        .send()
        .map_err(|error| format!("Failed to query IGDB time-to-beat endpoint: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("IGDB time-to-beat lookup failed with {status}: {body}"));
    }

    response
        .json::<Vec<IgdbGameTimeToBeat>>()
        .map_err(|error| format!("Failed to parse IGDB time-to-beat response: {error}"))
        .map(|mut items| items.pop())
}

fn query_screenshots(
    client: &Client,
    headers: &HeaderMap,
    game_id: i64,
) -> Result<Vec<String>, String> {
    let query = format!("fields screenshots.image_id; where id = {game_id}; limit 1;");

    let response = client
        .post(IGDB_GAMES_URL)
        .headers(headers.clone())
        .body(query)
        .send()
        .map_err(|error| format!("Failed to query IGDB screenshots: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("IGDB screenshots lookup failed with {status}: {body}"));
    }

    let mut games = response
        .json::<Vec<IgdbGame>>()
        .map_err(|error| format!("Failed to parse IGDB screenshots response: {error}"))?;

    Ok(games
        .pop()
        .and_then(|game| game.screenshots)
        .unwrap_or_default()
        .into_iter()
        .map(|image| build_screenshot_url(&image.image_id))
        .collect())
}

fn query_games(
    client: &Client,
    headers: &HeaderMap,
    search_title: &str,
) -> Result<Vec<IgdbSearchCandidate>, String> {
    let escaped_title = search_title.replace('\\', "\\\\").replace('"', "\\\"");
    let query = format!(
        "search \"{escaped_title}\"; fields name,slug,summary,genres.name,game_modes.name,multiplayer_modes.campaigncoop,multiplayer_modes.dropin,multiplayer_modes.lancoop,multiplayer_modes.offlinecoop,multiplayer_modes.offlinecoopmax,multiplayer_modes.onlinecoop,multiplayer_modes.onlinecoopmax,multiplayer_modes.splitscreen,multiplayer_modes.splitscreenonline,cover.image_id,artworks.image_id,total_rating,aggregated_rating,rating; limit 5;"
    );
    println!("[media] igdb api call: search title='{}'", search_title);

    let response = client
        .post(IGDB_GAMES_URL)
        .headers(headers.clone())
        .body(query)
        .send()
        .map_err(|error| format!("Failed to query IGDB games endpoint: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("IGDB games search failed with {status}: {body}"));
    }

    let results: Vec<IgdbSearchCandidate> = response
        .json::<Vec<IgdbGame>>()
        .map_err(|error| format!("Failed to parse IGDB games response: {error}"))?
        .into_iter()
        .map(|game| {
            let genres = game
                .genres
                .unwrap_or_default()
                .into_iter()
                .map(|genre| genre.name)
                .collect::<Vec<_>>();

            IgdbSearchCandidate {
                result: IgdbSearchResult {
                    id: game.id,
                    name: game.name,
                    slug: game.slug,
                    genres: genres.clone(),
                    cover_url: game.cover.map(|cover| build_cover_url(&cover.image_id)),
                    artwork_urls: game
                        .artworks
                        .unwrap_or_default()
                        .into_iter()
                        .map(|image| build_artwork_url(&image.image_id))
                        .collect(),
                    screenshot_urls: Vec::new(),
                    summary: game.summary,
                },
                aggregated_rating: game.aggregated_rating,
                game_modes: game
                    .game_modes
                    .unwrap_or_default()
                    .into_iter()
                    .map(|mode| mode.name)
                    .collect(),
                multiplayer_modes: game.multiplayer_modes.unwrap_or_default(),
                rating: game.rating,
                total_rating: game.total_rating,
            }
        })
        .collect();

    if results.is_empty() {
        println!("[media] igdb api result: 0 matches for '{}'", search_title);
    } else {
        println!(
            "[media] igdb api result: {} match(es) for '{}'",
            results.len(),
            search_title
        );
        for (index, candidate) in results.iter().enumerate() {
            let result = &candidate.result;
            println!(
                "  [{}] id={} name='{}' slug={} genres={} cover={} artworks={} screenshots={}",
                index + 1,
                result.id,
                result.name,
                result.slug.as_deref().unwrap_or("<none>"),
                if result.genres.is_empty() {
                    "<none>".to_string()
                } else {
                    result.genres.join(", ")
                },
                result.cover_url.as_deref().unwrap_or("<none>"),
                result.artwork_urls.len(),
                result.screenshot_urls.len()
            );
        }
    }

    Ok(results)
}

fn search_games_with_variant(title: &str) -> Result<(Vec<IgdbSearchResult>, String), String> {
    let trimmed_title = title.trim();
    if trimmed_title.is_empty() {
        return Err("Search title cannot be empty.".into());
    }

    let (client, headers) = build_client_and_headers()?;
    let variants = search_variants(trimmed_title);

    for (index, variant) in variants.iter().enumerate() {
        let results = query_games(&client, &headers, variant)?;
        if !results.is_empty() {
            if index > 0 {
                println!(
                    "[media] igdb fallback search succeeded: original='{}' variant='{}'",
                    trimmed_title, variant
                );
            }
            return Ok((
                results.into_iter().map(|candidate| candidate.result).collect(),
                variant.clone(),
            ));
        }
        if index + 1 < variants.len() {
            println!(
                "[media] igdb fallback search retry: original='{}' next_variant='{}'",
                trimmed_title,
                variants[index + 1]
            );
        }
    }

    Ok((Vec::new(), trimmed_title.to_string()))
}

pub fn search_games(title: &str) -> Result<Vec<IgdbSearchResult>, String> {
    search_games_with_variant(title).map(|(results, _)| results)
}

pub fn search_best_match(title: &str) -> Result<Option<IgdbBestMatch>, String> {
    let trimmed_title = title.trim();
    if trimmed_title.is_empty() {
        return Err("Search title cannot be empty.".into());
    }

    let (client, headers) = build_client_and_headers()?;
    let variants = search_variants(trimmed_title);
    let mut results = Vec::new();
    let mut matched_query = trimmed_title.to_string();

    for (index, variant) in variants.iter().enumerate() {
        let attempted = query_games(&client, &headers, variant)?;
        if !attempted.is_empty() {
            if index > 0 {
                println!(
                    "[media] igdb fallback search succeeded: original='{}' variant='{}'",
                    trimmed_title, variant
                );
            }
            results = attempted;
            matched_query = variant.clone();
            break;
        }
        if index + 1 < variants.len() {
            println!(
                "[media] igdb fallback search retry: original='{}' next_variant='{}'",
                trimmed_title,
                variants[index + 1]
            );
        }
    }

    let best_match = results
        .into_iter()
        .enumerate()
        .map(|result| {
            let (index, candidate) = result;
            let score = match_score(&matched_query, &candidate.result.name, candidate.result.slug.as_deref())
                .max(match_score(title, &candidate.result.name, candidate.result.slug.as_deref()));
            let coop = derive_coop(&candidate.game_modes, &candidate.multiplayer_modes);
            let time_to_beat = query_time_to_beat(&client, &headers, candidate.result.id).ok().flatten();
            let completion =
                derive_completion(&candidate.game_modes, &candidate.result.genres, time_to_beat.as_ref(), &coop);
            let screenshot_urls = if candidate.result.artwork_urls.is_empty() {
                query_screenshots(&client, &headers, candidate.result.id).unwrap_or_default()
            } else {
                Vec::new()
            };
            (
                score,
                index,
                IgdbBestMatch {
                    name: candidate.result.name,
                    genres: candidate.result.genres,
                    cover_url: candidate.result.cover_url,
                    artwork_urls: candidate.result.artwork_urls,
                    screenshot_urls,
                    summary: candidate.result.summary,
                    rating: format_rating(
                        candidate.total_rating,
                        candidate.aggregated_rating,
                        candidate.rating,
                    ),
                    coop,
                    completion,
                    score,
                },
            )
        })
        .filter(|(score, _, _)| *score >= 40)
        .max_by_key(|(score, index, _)| (*score, usize::MAX - *index))
        .map(|(_, _, candidate)| candidate);

    match &best_match {
        Some(candidate) => println!(
            "[media] igdb best match: title='{}' matched='{}' score={} rating={} coop='{}' completion='{}' cover={} screenshots={}",
            title,
            candidate.name,
            candidate.score,
            candidate.rating,
            candidate.coop,
            candidate.completion,
            candidate.cover_url.as_deref().unwrap_or("<none>"),
            candidate.screenshot_urls.len()
        ),
        None => println!("[media] igdb best match: title='{}' matched=<none>", title),
    }

    Ok(best_match)
}

#[cfg(test)]
mod tests {
    use super::{
        IgdbGameTimeToBeat, IgdbMultiplayerMode, default_completion, default_coop, default_rating,
        derive_completion, derive_coop, format_rating,
    };

    #[test]
    fn formats_rating_into_ten_point_scale() {
        assert_eq!(format_rating(Some(87.3), None, None), "8.7");
        assert_eq!(format_rating(None, None, None), default_rating());
    }

    #[test]
    fn derives_specific_coop_labels() {
        let online = vec![IgdbMultiplayerMode {
            campaigncoop: None,
            dropin: None,
            lancoop: None,
            offlinecoop: None,
            offlinecoopmax: None,
            onlinecoop: Some(true),
            onlinecoopmax: Some(4),
            splitscreen: None,
            splitscreenonline: None,
        }];
        let local = vec![IgdbMultiplayerMode {
            campaigncoop: Some(true),
            dropin: None,
            lancoop: None,
            offlinecoop: Some(true),
            offlinecoopmax: Some(2),
            onlinecoop: None,
            onlinecoopmax: None,
            splitscreen: Some(true),
            splitscreenonline: None,
        }];

        assert_eq!(derive_coop(&[], &online), "Online Co-op");
        assert_eq!(derive_coop(&[], &local), "Local Co-op");
        assert_eq!(derive_coop(&["Single player".into()], &[]), "No");
        assert_eq!(derive_coop(&[], &[]), default_coop());
    }

    #[test]
    fn prefers_time_to_beat_for_completion() {
        let time_to_beat = IgdbGameTimeToBeat {
            hastily: None,
            normally: Some(18 * 3600),
            completely: Some(32 * 3600),
        };

        assert_eq!(
            derive_completion(
                &["Single player".into()],
                &["Adventure".into()],
                Some(&time_to_beat),
                "No",
            ),
            "18h main"
        );
    }

    #[test]
    fn falls_back_to_mode_or_genre_completion_labels() {
        assert_eq!(
            derive_completion(
                &["Single player".into()],
                &["Role-playing (RPG)".into()],
                None,
                "No",
            ),
            "Story"
        );
        assert_eq!(
            derive_completion(
                &["Multiplayer".into()],
                &["Racing".into()],
                None,
                "Online Co-op",
            ),
            "Multiplayer"
        );
        assert_eq!(
            derive_completion(&[], &["Strategy".into()], None, "Unknown"),
            "Sandbox"
        );
        assert_eq!(derive_completion(&[], &[], None, "No"), default_completion());
    }
}
