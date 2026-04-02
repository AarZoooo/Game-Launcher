use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::media::matching::title::match_score;

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
    pub screenshot_urls: Vec<String>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IgdbBestMatch {
    pub name: String,
    pub genres: Vec<String>,
    pub cover_url: Option<String>,
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
    format!("https://images.igdb.com/igdb/image/upload/t_cover_big/{image_id}.png")
}

fn build_screenshot_url(image_id: &str) -> String {
    format!("https://images.igdb.com/igdb/image/upload/t_screenshot_big/{image_id}.png")
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

pub fn search_games(title: &str) -> Result<Vec<IgdbSearchResult>, String> {
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
    let escaped_title = trimmed_title.replace('\\', "\\\\").replace('"', "\\\"");
    let query = format!(
        "search \"{escaped_title}\"; fields name,slug,summary,genres.name,cover.image_id,screenshots.image_id; limit 5;"
    );
    println!("[media] igdb api call: search title='{}'", trimmed_title);

    let response = client
        .post(IGDB_GAMES_URL)
        .headers(headers)
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
        .map(|game| {
            IgdbSearchResult {
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
                screenshot_urls: game
                    .screenshots
                    .unwrap_or_default()
                    .into_iter()
                    .map(|image| build_screenshot_url(&image.image_id))
                    .collect(),
                summary: game.summary,
            }
        })
        .collect();

    if results.is_empty() {
        println!("[media] igdb api result: 0 matches for '{}'", trimmed_title);
    } else {
        println!(
            "[media] igdb api result: {} match(es) for '{}'",
            results.len(),
            trimmed_title
        );
        for (index, result) in results.iter().enumerate() {
            println!(
                "  [{}] id={} name='{}' slug={} genres={} cover={} screenshots={}",
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
                result.screenshot_urls.len()
            );
        }
    }

    Ok(results)
}

pub fn search_best_match(title: &str) -> Result<Option<IgdbBestMatch>, String> {
    let results = search_games(title)?;
    let best_match = results
        .into_iter()
        .map(|result| {
            let score = match_score(title, &result.name, result.slug.as_deref());
            IgdbBestMatch {
                name: result.name,
                genres: result.genres,
                cover_url: result.cover_url,
                screenshot_urls: result.screenshot_urls,
                summary: result.summary,
                score,
            }
        })
        .filter(|candidate| candidate.score >= 40)
        .max_by_key(|candidate| candidate.score);

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
