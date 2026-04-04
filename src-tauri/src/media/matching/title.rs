use regex::Regex;

fn alias_patterns() -> Vec<(Regex, &'static str)> {
    vec![
        (Regex::new(r"\bcs\s*2\b").expect("valid regex"), "counter strike 2"),
        (
            Regex::new(r"\bgta\s*vi\b").expect("valid regex"),
            "grand theft auto 6",
        ),
        (
            Regex::new(r"\bre\s*([2-9])\b").expect("valid regex"),
            "resident evil $1",
        ),
    ]
}

fn edition_pattern() -> Regex {
    Regex::new(
        r"\b(game of the year|goty|definitive edition|director'?s cut|complete edition|ultimate edition|deluxe edition|remastered|update \d+|steam edition|demo|beta)\b",
    )
    .expect("valid regex")
}

fn punctuation_pattern() -> Regex {
    Regex::new(r"[^a-z0-9\s]").expect("valid regex")
}

fn whitespace_pattern() -> Regex {
    Regex::new(r"\s+").expect("valid regex")
}

fn roman_token(token: &str) -> Option<&'static str> {
    match token {
        "i" => Some("1"),
        "ii" => Some("2"),
        "iii" => Some("3"),
        "iv" => Some("4"),
        "v" => Some("5"),
        "vi" => Some("6"),
        "vii" => Some("7"),
        "viii" => Some("8"),
        "ix" => Some("9"),
        "x" => Some("10"),
        _ => None,
    }
}

pub fn normalize_title(value: &str) -> String {
    let mut normalized = value.to_ascii_lowercase();

    for (pattern, replacement) in alias_patterns() {
        normalized = pattern.replace_all(&normalized, replacement).into_owned();
    }

    normalized = edition_pattern().replace_all(&normalized, " ").into_owned();
    normalized = punctuation_pattern().replace_all(&normalized, " ").into_owned();

    let normalized_tokens = normalized
        .split_whitespace()
        .map(|token| roman_token(token).unwrap_or(token))
        .collect::<Vec<_>>();

    whitespace_pattern()
        .replace_all(&normalized_tokens.join(" "), " ")
        .trim()
        .to_string()
}

pub fn match_score(query: &str, candidate_name: &str, candidate_slug: Option<&str>) -> i32 {
    let normalized_query = normalize_title(query);
    let normalized_name = normalize_title(candidate_name);
    let normalized_slug = candidate_slug.map(normalize_title).unwrap_or_default();
    let variant_markers = ["update", "dlc", "demo", "beta", "alpha", "modeler", "editor"];
    let lower_name = candidate_name.to_ascii_lowercase();
    let lower_slug = candidate_slug.unwrap_or_default().to_ascii_lowercase();
    let is_variant = variant_markers
        .iter()
        .any(|marker| lower_name.contains(marker) || lower_slug.contains(marker));

    if normalized_query.is_empty() || normalized_name.is_empty() {
        return 0;
    }

    if normalized_query == normalized_name && !is_variant {
        return 1_000;
    }

    if !normalized_slug.is_empty() && normalized_query == normalized_slug && !is_variant {
        return 950;
    }

    let mut score = 0;

    if normalized_name.starts_with(&normalized_query) || normalized_query.starts_with(&normalized_name)
    {
        score += 40;
    }

    if !normalized_slug.is_empty()
        && (normalized_slug.starts_with(&normalized_query)
            || normalized_query.starts_with(&normalized_slug))
    {
        score += 35;
    }

    if normalized_name.contains(&normalized_query) || normalized_query.contains(&normalized_name) {
        score += 25;
    }

    let query_tokens = normalized_query.split_whitespace().collect::<Vec<_>>();
    let name_tokens = normalized_name.split_whitespace().collect::<Vec<_>>();
    let shared_tokens = query_tokens
        .iter()
        .filter(|token| name_tokens.contains(token))
        .count() as i32;

    let mut final_score =
        score + shared_tokens * 8 - (name_tokens.len() as i32 - query_tokens.len() as i32).abs() * 2;
    if is_variant {
        final_score -= if normalized_query == normalized_name { 48 } else { 18 };
    }

    final_score
}

#[cfg(test)]
mod tests {
    use super::{match_score, normalize_title};

    #[test]
    fn normalizes_titles_with_suffixes_and_punctuation() {
        assert_eq!(
            normalize_title("Resident Evil IV: Deluxe Edition"),
            "resident evil 4"
        );
        assert_eq!(normalize_title("Counter-Strike 2"), "counter strike 2");
    }

    #[test]
    fn prefers_exact_title_matches() {
        assert!(
            match_score("Satisfactory", "Satisfactory", Some("satisfactory"))
                > match_score("Satisfactory", "Satisfactory Update 8", Some("satisfactory-update-8"))
        );
    }

    #[test]
    fn exact_base_game_beats_variant_suffixes() {
        assert!(
            match_score("Rocket League", "Rocket League", Some("rocket-league"))
                > match_score(
                    "Rocket League",
                    "Rocket League: Season 9",
                    Some("rocket-league-season-9"),
                )
        );
        assert!(
            match_score(
                "Human Fall Flat",
                "Human: Fall Flat",
                Some("human-fall-flat"),
            ) > match_score(
                "Human Fall Flat",
                "Human Fall Flat VR",
                Some("human-fall-flat-vr"),
            )
        );
        assert!(
            match_score(
                "Golf With Your Friends",
                "Golf With Your Friends",
                Some("golf-with-your-friends"),
            ) > match_score(
                "Golf With Your Friends",
                "Golf With Your Friends 2",
                Some("golf-with-your-friends-2"),
            )
        );
        assert!(
            match_score("Trackmania", "Trackmania", Some("trackmania"))
                > match_score(
                    "Trackmania",
                    "TrackMania United Forever",
                    Some("trackmania-united-forever"),
                )
        );
    }
}
