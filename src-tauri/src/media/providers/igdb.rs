use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::media::matching::title::{match_score, normalize_title};

const TWITCH_TOKEN_URL: &str = "https://id.twitch.tv/oauth2/token";
const IGDB_GAMES_URL: &str = "https://api.igdb.com/v4/games";

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Deserialize)]
struct IgdbGame {
    id: i64,
    name: String,
    slug: Option<String>,
    summary: Option<String>,
    cover: Option<IgdbImage>,
    artworks: Option<Vec<IgdbImage>>,
    screenshots: Option<Vec<IgdbImage>>,
    genres: Option<Vec<IgdbGenre>>,
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

fn query_games(
    client: &Client,
    headers: &HeaderMap,
    search_title: &str,
) -> Result<Vec<IgdbSearchResult>, String> {
    let escaped_title = search_title.replace('\\', "\\\\").replace('"', "\\\"");
    let query = format!(
        "search \"{escaped_title}\"; fields name,slug,summary,genres.name,cover.image_id,artworks.image_id,screenshots.image_id; limit 5;"
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

    let results: Vec<IgdbSearchResult> = response
        .json::<Vec<IgdbGame>>()
        .map_err(|error| format!("Failed to parse IGDB games response: {error}"))?
        .into_iter()
        .map(|game| IgdbSearchResult {
            id: game.id,
            name: game.name,
            slug: game.slug,
            genres: game
                .genres
                .unwrap_or_default()
                .into_iter()
                .map(|genre| genre.name)
                .collect(),
            cover_url: game.cover.map(|cover| build_cover_url(&cover.image_id)),
            artwork_urls: game
                .artworks
                .unwrap_or_default()
                .into_iter()
                .map(|image| build_artwork_url(&image.image_id))
                .collect(),
            screenshot_urls: game
                .screenshots
                .unwrap_or_default()
                .into_iter()
                .map(|image| build_screenshot_url(&image.image_id))
                .collect(),
            summary: game.summary,
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
        for (index, result) in results.iter().enumerate() {
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

    let client_id = required_env("TWITCH_CLIENT_ID")?;
    let client_secret = required_env("TWITCH_CLIENT_SECRET")?;
    let client = Client::builder()
        .build()
        .map_err(|error| format!("Failed to build IGDB HTTP client: {error}"))?;
    let access_token = twitch_access_token(&client, &client_id, &client_secret)?;
    let headers = games_headers(&client_id, &access_token)?;
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
            return Ok((results, variant.clone()));
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
    let (results, matched_query) = search_games_with_variant(title)?;
    let best_match = results
        .into_iter()
        .enumerate()
        .map(|result| {
            let (index, result) = result;
            let score = match_score(&matched_query, &result.name, result.slug.as_deref())
                .max(match_score(title, &result.name, result.slug.as_deref()));
            (
                score,
                index,
                IgdbBestMatch {
                    name: result.name,
                    genres: result.genres,
                    cover_url: result.cover_url,
                    artwork_urls: result.artwork_urls,
                    screenshot_urls: result.screenshot_urls,
                    summary: result.summary,
                    score,
                },
            )
        })
        .filter(|(score, _, _)| *score >= 40)
        .max_by_key(|(score, index, _)| (*score, usize::MAX - *index))
        .map(|(_, _, candidate)| candidate);

    match &best_match {
        Some(candidate) => println!(
            "[media] igdb best match: title='{}' matched='{}' score={} cover={} screenshots={}",
            title,
            candidate.name,
            candidate.score,
            candidate.cover_url.as_deref().unwrap_or("<none>"),
            candidate.screenshot_urls.len()
        ),
        None => println!("[media] igdb best match: title='{}' matched=<none>", title),
    }

    Ok(best_match)
}
