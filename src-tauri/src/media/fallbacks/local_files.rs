use std::fs;
use std::path::{Path, PathBuf};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use image::ImageReader;

use crate::media::matching::title::normalize_title;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LocalMediaSelection {
    pub cover_vertical: Option<String>,
    pub cover_horizontal: Option<String>,
    pub banner: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Clone)]
struct Candidate {
    path: PathBuf,
    file_name: String,
    search_rank: usize,
    width: u32,
    height: u32,
}

const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "webp", "bmp"];
const TECHNICAL_DIRECTORY_MARKERS: &[&str] = &[
    "binaries",
    "bin",
    "win64",
    "win32",
    "shipping",
    "x64",
    "x86",
    "windows",
    "linux",
    "macos",
];

fn collect_candidate_files(root: &Path, depth: usize, search_rank: usize, results: &mut Vec<(PathBuf, usize)>) {
    if depth > 2 || !root.exists() {
        return;
    }

    let Ok(entries) = fs::read_dir(root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };

        if file_type.is_dir() {
            collect_candidate_files(&path, depth + 1, search_rank, results);
            continue;
        }

        if file_type.is_file()
            && path
                .extension()
                .and_then(|value| value.to_str())
                .map(|value| IMAGE_EXTENSIONS.iter().any(|extension| value.eq_ignore_ascii_case(extension)))
                .unwrap_or(false)
        {
            results.push((path, search_rank));
        }
    }
}

fn normalized_name(value: &str) -> String {
    normalize_title(value).replace(' ', "")
}

fn is_technical_directory(path: &Path, exe_stem: &str) -> bool {
    let directory_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    let normalized = normalized_name(directory_name);

    !normalized.is_empty()
        && (TECHNICAL_DIRECTORY_MARKERS.contains(&normalized.as_str())
            || (!exe_stem.is_empty() && normalized == format!("{exe_stem}data")))
}

fn collect_search_roots(exe: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    let mut seen = Vec::new();
    let exe_stem = exe
        .file_stem()
        .and_then(|value| value.to_str())
        .map(normalized_name)
        .unwrap_or_default();

    let Some(parent) = exe.parent() else {
        return roots;
    };

    let mut current = parent.to_path_buf();
    loop {
        let normalized = current.to_string_lossy().to_ascii_lowercase();
        if !seen.contains(&normalized) {
            seen.push(normalized);
            roots.push(current.clone());
        }

        let Some(ancestor) = current.parent() else {
            break;
        };

        if !is_technical_directory(&current, &exe_stem) {
            break;
        }

        current = ancestor.to_path_buf();
    }

    roots
}

fn image_dimensions(path: &Path) -> Option<(u32, u32)> {
    ImageReader::open(path)
        .ok()?
        .with_guessed_format()
        .ok()?
        .into_dimensions()
        .ok()
}

fn candidate_from_path(path: PathBuf, search_rank: usize) -> Option<Candidate> {
    let (width, height) = image_dimensions(&path)?;
    Some(Candidate {
        file_name: path.file_name()?.to_string_lossy().to_ascii_lowercase(),
        path,
        search_rank,
        width,
        height,
    })
}

fn mime_for_path(path: &Path) -> &'static str {
    match path.extension().and_then(|value| value.to_str()).unwrap_or_default().to_ascii_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        _ => "image/png",
    }
}

fn data_url_from_path(path: &Path) -> Option<String> {
    let bytes = fs::read(path).ok()?;
    Some(format!(
        "data:{};base64,{}",
        mime_for_path(path),
        STANDARD.encode(bytes)
    ))
}

fn title_bonus(file_name: &str, title: &str) -> i32 {
    let normalized_title = normalized_name(title);
    let normalized_file = normalized_name(file_name);

    if normalized_title.is_empty() {
        0
    } else if normalized_file == normalized_title {
        24
    } else if normalized_file.contains(&normalized_title) {
        16
    } else {
        0
    }
}

fn proximity_bonus(candidate: &Candidate) -> i32 {
    match candidate.search_rank {
        0 => 32,
        1 => 18,
        2 => 8,
        _ => 0,
    }
}

fn portrait_score(candidate: &Candidate, title: &str) -> i32 {
    let ratio = candidate.width as f32 / candidate.height as f32;
    let mut score = title_bonus(&candidate.file_name, title) + proximity_bonus(candidate);

    if (0.55..=0.8).contains(&ratio) {
        score += 24;
    }
    if ["cover", "poster", "vertical", "portrait"]
        .iter()
        .any(|needle| candidate.file_name.contains(needle))
    {
        score += 18;
    }
    score
}

fn horizontal_score(candidate: &Candidate, title: &str) -> i32 {
    let ratio = candidate.width as f32 / candidate.height as f32;
    let mut score = title_bonus(&candidate.file_name, title) + proximity_bonus(candidate);

    if ratio >= 1.4 {
        score += 18;
    }
    if ["header", "wide", "landscape", "horizontal", "screenshot", "logo"]
        .iter()
        .any(|needle| candidate.file_name.contains(needle))
    {
        score += 16;
    }
    score
}

fn banner_score(candidate: &Candidate, title: &str) -> i32 {
    let ratio = candidate.width as f32 / candidate.height as f32;
    let mut score = title_bonus(&candidate.file_name, title) + proximity_bonus(candidate);

    if ratio >= 2.0 {
        score += 22;
    }
    if ["banner", "hero", "background", "splash", "keyart", "logo"]
        .iter()
        .any(|needle| candidate.file_name.contains(needle))
    {
        score += 20;
    }
    score
}

fn icon_score(candidate: &Candidate, title: &str) -> i32 {
    let ratio = candidate.width.max(candidate.height) as f32 / candidate.width.min(candidate.height) as f32;
    let mut score = title_bonus(&candidate.file_name, title) + proximity_bonus(candidate);

    if ratio <= 1.2 {
        score += 18;
    }
    if ["icon", "logo", "badge"]
        .iter()
        .any(|needle| candidate.file_name.contains(needle))
    {
        score += 20;
    }
    score
}

fn best_candidate<F>(candidates: &[Candidate], scorer: F) -> Option<&Candidate>
where
    F: Fn(&Candidate) -> i32,
{
    candidates.iter().max_by_key(|candidate| scorer(candidate))
}

pub fn find_local_media(exe_path: &str, title: &str) -> LocalMediaSelection {
    let mut paths = Vec::new();
    let exe = Path::new(exe_path);

    for (search_rank, root) in collect_search_roots(exe).into_iter().enumerate() {
        collect_candidate_files(&root, 0, search_rank, &mut paths);
    }

    let candidates = paths
        .into_iter()
        .filter_map(|(path, search_rank)| candidate_from_path(path, search_rank))
        .collect::<Vec<_>>();

    let cover_vertical = best_candidate(&candidates, |candidate| portrait_score(candidate, title))
        .and_then(|candidate| data_url_from_path(&candidate.path));
    let cover_horizontal =
        best_candidate(&candidates, |candidate| horizontal_score(candidate, title))
            .and_then(|candidate| data_url_from_path(&candidate.path));
    let banner = best_candidate(&candidates, |candidate| banner_score(candidate, title))
        .and_then(|candidate| data_url_from_path(&candidate.path));
    let icon = best_candidate(&candidates, |candidate| icon_score(candidate, title))
        .and_then(|candidate| data_url_from_path(&candidate.path));

    LocalMediaSelection {
        cover_vertical,
        cover_horizontal,
        banner,
        icon,
    }
}

#[cfg(test)]
mod tests {
    use super::{Candidate, banner_score, collect_search_roots, icon_score, portrait_score};
    use std::path::{Path, PathBuf};

    fn candidate(name: &str, search_rank: usize, width: u32, height: u32) -> Candidate {
        Candidate {
            path: PathBuf::from(name),
            file_name: name.to_ascii_lowercase(),
            search_rank,
            width,
            height,
        }
    }

    #[test]
    fn prefers_vertical_cover_like_names_for_portraits() {
        let portrait = candidate("satisfactory-cover.png", 0, 600, 900);
        let screenshot = candidate("satisfactory-screenshot.png", 0, 1280, 720);
        assert!(portrait_score(&portrait, "Satisfactory") > portrait_score(&screenshot, "Satisfactory"));
    }

    #[test]
    fn prefers_banner_and_icon_shapes() {
        let banner = candidate("hero-banner.png", 0, 1600, 500);
        let icon = candidate("game-icon.png", 0, 512, 512);
        assert!(banner_score(&banner, "Satisfactory") > icon_score(&banner, "Satisfactory"));
        assert!(icon_score(&icon, "Satisfactory") > icon_score(&banner, "Satisfactory"));
    }

    #[test]
    fn generic_cover_name_beats_unrelated_title_when_local() {
        let local_cover = candidate("cover.png", 0, 600, 900);
        let unrelated_sibling = candidate("naughty-dog-cover.png", 2, 600, 900);
        assert!(portrait_score(&local_cover, "Hell Let Loose") > portrait_score(&unrelated_sibling, "Hell Let Loose"));
    }

    #[test]
    fn search_roots_stop_before_shared_library_folder() {
        let exe = Path::new(r"D:\Games\Hell Let Loose\HellLetLoose.exe");
        let roots = collect_search_roots(exe);
        assert_eq!(roots, vec![PathBuf::from(r"D:\Games\Hell Let Loose")]);
    }

    #[test]
    fn search_roots_expand_out_of_technical_subfolders_only() {
        let exe = Path::new(r"D:\Games\Hell Let Loose\Binaries\Win64\HLL-Win64-Shipping.exe");
        let roots = collect_search_roots(exe);
        assert_eq!(
            roots,
            vec![
                PathBuf::from(r"D:\Games\Hell Let Loose\Binaries\Win64"),
                PathBuf::from(r"D:\Games\Hell Let Loose\Binaries"),
                PathBuf::from(r"D:\Games\Hell Let Loose"),
            ]
        );
    }

    #[test]
    fn bmp_extension_is_supported() {
        assert_eq!(super::mime_for_path(Path::new("cover.bmp")), "image/bmp");
    }
}
