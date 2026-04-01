use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

const GAME_PROCESS_EVENT: &str = "game-process-state";
const PROCESS_POLL_INTERVAL: Duration = Duration::from_secs(2);
const SHORT_SESSION_THRESHOLD: Duration = Duration::from_secs(15);
const REAPPEARANCE_WINDOW: Duration = Duration::from_secs(30);

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct GameProcessEvent {
    game_id: Option<String>,
    exe_path: String,
    state: &'static str,
    message: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ProcessSnapshot {
    #[allow(dead_code)]
    process_id: u32,
    name: Option<String>,
    executable_path: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ProcessSnapshotResponse {
    One(ProcessSnapshot),
    Many(Vec<ProcessSnapshot>),
}

fn escape_powershell_single_quotes(value: &str) -> String {
    value.replace('\'', "''")
}

fn normalize_path(value: &str) -> String {
    value.trim().replace('/', "\\").to_ascii_lowercase()
}

fn file_name_lowercase(exe_path: &str) -> Option<String> {
    Path::new(exe_path)
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .map(|file_name| file_name.to_ascii_lowercase())
}

fn emit_game_process_state(
    app: &AppHandle,
    game_id: &Option<String>,
    exe_path: &str,
    state: &'static str,
    message: Option<String>,
) {
    println!(
        "[launch_game] emitting event state={state} game_id={:?} exe_path={exe_path} message={message:?}",
        game_id
    );

    let _ = app.emit(
        GAME_PROCESS_EVENT,
        GameProcessEvent {
            game_id: game_id.clone(),
            exe_path: exe_path.to_string(),
            state,
            message,
        },
    );
}

fn launch_with_elevation(exe_path: &str) -> Result<u32, String> {
    let script = format!(
        "$process = Start-Process -FilePath '{}' -Verb RunAs -PassThru; if ($null -eq $process) {{ exit 1 }}; Write-Output $process.Id",
        escape_powershell_single_quotes(exe_path)
    );

    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output()
        .map_err(|error| format!("Failed to request administrator launch: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            "Administrator launch was cancelled or failed.".into()
        } else {
            format!("Failed to request administrator launch: {stderr}")
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let pid = stdout
        .trim()
        .parse::<u32>()
        .map_err(|error| format!("Failed to read administrator launch PID: {error}"))?;

    Ok(pid)
}

fn spawn_game_process(exe_path: &str) -> Result<u32, String> {
    match Command::new(exe_path).spawn() {
        Ok(child) => Ok(child.id()),
        Err(error) if error.raw_os_error() == Some(740) => launch_with_elevation(exe_path),
        Err(error) => Err(format!("Failed to launch game: {error}")),
    }
}

fn process_snapshots() -> Result<Vec<ProcessSnapshot>, String> {
    let script = "Get-CimInstance Win32_Process | Select-Object ProcessId,Name,ExecutablePath | ConvertTo-Json -Compress";
    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
        .map_err(|error| format!("Failed to inspect running processes: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            "Failed to inspect running processes.".into()
        } else {
            format!("Failed to inspect running processes: {stderr}")
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        return Ok(Vec::new());
    }

    let parsed = serde_json::from_str::<ProcessSnapshotResponse>(&stdout)
        .map_err(|error| format!("Failed to parse process list: {error}"))?;

    Ok(match parsed {
        ProcessSnapshotResponse::One(snapshot) => vec![snapshot],
        ProcessSnapshotResponse::Many(snapshots) => snapshots,
    })
}

fn is_target_process(process: &ProcessSnapshot, normalized_path: &str, expected_name: &str) -> bool {
    if let Some(executable_path) = &process.executable_path {
        if normalize_path(executable_path) == normalized_path {
            return true;
        }
    }

    process
        .name
        .as_deref()
        .map(|name| name.eq_ignore_ascii_case(expected_name))
        .unwrap_or(false)
}

fn is_game_running(normalized_path: &str, expected_name: &str) -> Result<bool, String> {
    let snapshots = process_snapshots()?;
    Ok(snapshots
        .iter()
        .any(|process| is_target_process(process, normalized_path, expected_name)))
}

fn monitor_game_process(app: AppHandle, exe_path: String, game_id: Option<String>) {
    let normalized_path = normalize_path(&exe_path);
    let expected_name = file_name_lowercase(&exe_path).unwrap_or_default();

    if expected_name.is_empty() {
        emit_game_process_state(
            &app,
            &game_id,
            &exe_path,
            "error",
            Some("Failed to determine the launched executable name.".into()),
        );
        return;
    }

    let mut is_marked_running = true;
    let mut session_started_at = Instant::now();
    let mut reappearance_deadline: Option<Instant> = None;

    loop {
        thread::sleep(PROCESS_POLL_INTERVAL);

        let is_running = match is_game_running(&normalized_path, &expected_name) {
            Ok(is_running) => is_running,
            Err(error) => {
                emit_game_process_state(&app, &game_id, &exe_path, "error", Some(error));
                break;
            }
        };

        if is_running {
            if !is_marked_running {
                emit_game_process_state(
                    &app,
                    &game_id,
                    &exe_path,
                    "started",
                    Some("Detected the game process again.".into()),
                );
                is_marked_running = true;
                session_started_at = Instant::now();
            }

            reappearance_deadline = None;
            continue;
        }

        if is_marked_running {
            emit_game_process_state(&app, &game_id, &exe_path, "exited", None);
            is_marked_running = false;

            if session_started_at.elapsed() < SHORT_SESSION_THRESHOLD {
                println!(
                    "[launch_game] short session detected; watching for restart for {} seconds game_id={:?} exe_path={}",
                    REAPPEARANCE_WINDOW.as_secs(),
                    game_id,
                    exe_path
                );
                reappearance_deadline = Some(Instant::now() + REAPPEARANCE_WINDOW);
            } else {
                println!(
                    "[launch_game] treating exit as final after {} seconds game_id={:?} exe_path={}",
                    session_started_at.elapsed().as_secs(),
                    game_id,
                    exe_path
                );
                break;
            }

            continue;
        }

        if let Some(deadline) = reappearance_deadline {
            if Instant::now() >= deadline {
                println!(
                    "[launch_game] stopping background tracking for game_id={:?} exe_path={}",
                    game_id, exe_path
                );
                break;
            }
        } else {
            break;
        }
    }
}

#[tauri::command]
pub fn launch_game(
    app: AppHandle,
    exe_path: String,
    game_id: Option<String>,
) -> Result<String, String> {
    let trimmed_path = exe_path.trim();

    if trimmed_path.is_empty() {
        return Err("Executable path cannot be empty.".into());
    }

    if !Path::new(trimmed_path).exists() {
        return Err(format!("Executable not found: {trimmed_path}"));
    }

    println!(
        "[launch_game] launching exe_path={} game_id={:?}",
        trimmed_path, game_id
    );

    let pid = spawn_game_process(trimmed_path)?;
    println!("[launch_game] initial pid={pid} exe_path={trimmed_path}");

    emit_game_process_state(&app, &game_id, trimmed_path, "started", None);

    let tracked_app = app.clone();
    let tracked_path = trimmed_path.to_string();
    let tracked_game_id = game_id.clone();

    thread::spawn(move || monitor_game_process(tracked_app, tracked_path, tracked_game_id));

    Ok(format!("Launched game: {trimmed_path}"))
}
