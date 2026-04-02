use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use sysinfo::{Process, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System, UpdateKind};
use tauri::AppHandle;

use super::events::emit_game_process_state;
use super::registry::{replace_tracker, tracker_key, unregister_tracker};

const SHORT_SESSION_THRESHOLD: Duration = Duration::from_secs(15);
const FAST_POLL_INTERVAL: Duration = Duration::from_secs(2);
const FAST_REAPPEARANCE_WINDOW: Duration = Duration::from_secs(30);
const EXTENDED_POLL_INTERVAL: Duration = Duration::from_secs(20);
const EXTENDED_REAPPEARANCE_WINDOW: Duration = Duration::from_secs(900);

fn normalize_path(value: &str) -> String {
    value.trim().replace('/', "\\").to_ascii_lowercase()
}

fn file_name_lowercase(exe_path: &str) -> Option<String> {
    Path::new(exe_path)
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .map(|file_name| file_name.to_ascii_lowercase())
}

fn process_refresh_kind() -> ProcessRefreshKind {
    ProcessRefreshKind::nothing()
        .with_exe(UpdateKind::OnlyIfNotSet)
        .without_tasks()
}

fn system_for_process_tracking() -> System {
    System::new_with_specifics(RefreshKind::nothing().with_processes(process_refresh_kind()))
}

fn is_target_process(process: &Process, normalized_path: &str, expected_name: &str) -> bool {
    let exe_matches = process
        .exe()
        .and_then(|path| path.to_str())
        .map(normalize_path)
        .map(|path| path == normalized_path)
        .unwrap_or(false);

    let name_matches = process
        .name()
        .to_str()
        .map(|name| name.eq_ignore_ascii_case(expected_name))
        .unwrap_or(false);

    exe_matches || name_matches
}

fn is_game_running(system: &System, normalized_path: &str, expected_name: &str) -> bool {
    system
        .processes()
        .values()
        .any(|process| is_target_process(process, normalized_path, expected_name))
}

fn launcher_activity_detected(system: &System, normalized_path: &str) -> bool {
    let path_markers = [
        "\\steam\\",
        "\\steamapps\\",
        "\\epic games\\",
        "\\epicgames\\",
        "\\launcher\\",
        "\\updater\\",
        "\\patch\\",
        "\\bootstrap",
    ];
    let name_markers = [
        "steam.exe",
        "steamservice.exe",
        "steamwebhelper.exe",
        "epicgameslauncher.exe",
        "epiconlineservices",
        "launcher",
        "updater",
        "bootstrap",
        "patch",
    ];

    path_markers.iter().any(|marker| normalized_path.contains(marker))
        || system.processes().values().any(|process| {
            let name_matches = process
                .name()
                .to_str()
                .map(|name| {
                    let lower_name = name.to_ascii_lowercase();
                    name_markers.iter().any(|marker| lower_name.contains(marker))
                })
                .unwrap_or(false);

            let path_matches = process
                .exe()
                .and_then(|path| path.to_str())
                .map(normalize_path)
                .map(|path| path_markers.iter().any(|marker| path.contains(marker)))
                .unwrap_or(false);

            name_matches || path_matches
        })
}

fn monitor_game_process(
    app: AppHandle,
    exe_path: String,
    game_id: Option<String>,
    key: String,
    cancel_token: Arc<AtomicBool>,
) {
    let normalized_path = normalize_path(&exe_path);
    let expected_name = file_name_lowercase(&exe_path).unwrap_or_default();

    if expected_name.is_empty() {
        emit_game_process_state(&app, &game_id, &exe_path, "error", Some("Failed to determine the launched executable name.".into()));
        unregister_tracker(&key, &cancel_token);
        return;
    }

    let mut system = system_for_process_tracking();
    let mut is_marked_running = true;
    let mut session_started_at = Instant::now();
    let mut reappearance_deadline: Option<Instant> = None;
    let mut poll_interval = FAST_POLL_INTERVAL;
    let mut extended_watch_enabled = false;
    let mut short_watch_started_at: Option<Instant> = None;

    loop {
        if cancel_token.load(Ordering::SeqCst) {
            println!("[launch_game] stopping cancelled tracker key={key}");
            break;
        }

        thread::sleep(poll_interval);
        system.refresh_processes_specifics(ProcessesToUpdate::All, true, process_refresh_kind());

        if cancel_token.load(Ordering::SeqCst) {
            println!("[launch_game] tracker cancelled during refresh key={key}");
            break;
        }

        if is_game_running(&system, &normalized_path, &expected_name) {
            if !is_marked_running {
                emit_game_process_state(&app, &game_id, &exe_path, "started", Some("Detected the game process again.".into()));
                is_marked_running = true;
                session_started_at = Instant::now();
            }

            reappearance_deadline = None;
            poll_interval = FAST_POLL_INTERVAL;
            extended_watch_enabled = false;
            short_watch_started_at = None;
            continue;
        }

        if is_marked_running {
            emit_game_process_state(&app, &game_id, &exe_path, "exited", None);
            is_marked_running = false;

            if session_started_at.elapsed() < SHORT_SESSION_THRESHOLD {
                let now = Instant::now();
                println!(
                    "[launch_game] short session detected; watching for restart for {} seconds game_id={:?} exe_path={} tracker_key={}",
                    FAST_REAPPEARANCE_WINDOW.as_secs(), game_id, exe_path, key
                );
                reappearance_deadline = Some(now + FAST_REAPPEARANCE_WINDOW);
                short_watch_started_at = Some(now);
                poll_interval = FAST_POLL_INTERVAL;
            } else {
                println!(
                    "[launch_game] treating exit as final after {} seconds game_id={:?} exe_path={} tracker_key={}",
                    session_started_at.elapsed().as_secs(), game_id, exe_path, key
                );
                break;
            }

            continue;
        }

        if !extended_watch_enabled {
            if let Some(started_at) = short_watch_started_at {
                if started_at.elapsed() >= FAST_REAPPEARANCE_WINDOW
                    && launcher_activity_detected(&system, &normalized_path)
                {
                    println!(
                        "[launch_game] launcher/update activity still present after fast watch; extending restart watch for {} seconds at {} second intervals game_id={:?} exe_path={} tracker_key={}",
                        EXTENDED_REAPPEARANCE_WINDOW.as_secs(),
                        EXTENDED_POLL_INTERVAL.as_secs(),
                        game_id,
                        exe_path,
                        key
                    );
                    reappearance_deadline = Some(Instant::now() + EXTENDED_REAPPEARANCE_WINDOW);
                    poll_interval = EXTENDED_POLL_INTERVAL;
                    extended_watch_enabled = true;
                    short_watch_started_at = None;
                    continue;
                }
            }
        }

        if let Some(deadline) = reappearance_deadline {
            if Instant::now() >= deadline {
                println!(
                    "[launch_game] stopping background tracking for game_id={:?} exe_path={} extended_watch_enabled={} tracker_key={}",
                    game_id, exe_path, extended_watch_enabled, key
                );
                break;
            }
        } else {
            break;
        }
    }

    unregister_tracker(&key, &cancel_token);
}

pub fn start_tracking(app: AppHandle, exe_path: String, game_id: Option<String>) {
    let key = tracker_key(&game_id, &exe_path);
    let cancel_token = replace_tracker(&key);

    println!("[launch_game] registered tracker key={key}");

    thread::spawn(move || monitor_game_process(app, exe_path, game_id, key, cancel_token));
}
