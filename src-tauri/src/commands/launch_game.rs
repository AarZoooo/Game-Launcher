use std::path::Path;
use std::process::{Child, Command};

fn escape_powershell_single_quotes(value: &str) -> String {
    value.replace('\'', "''")
}

fn launch_with_elevation(exe_path: &str) -> Result<Child, String> {
    let script = format!(
        "$process = Start-Process -FilePath '{}' -Verb RunAs -PassThru; if ($null -eq $process) {{ exit 1 }}; Wait-Process -Id $process.Id",
        escape_powershell_single_quotes(exe_path)
    );

    Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .spawn()
        .map_err(|error| format!("Failed to request administrator launch: {error}"))
}

use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct GameProcessEvent {
    game_id: Option<String>,
    exe_path: String,
    state: &'static str,
    message: Option<String>,
}

fn spawn_game_process(exe_path: &str) -> Result<Child, String> {
    match Command::new(exe_path).spawn() {
        Ok(child) => Ok(child),
        Err(error) if error.raw_os_error() == Some(740) => launch_with_elevation(exe_path),
        Err(error) => Err(format!("Failed to launch game: {error}")),
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

    let mut child = spawn_game_process(trimmed_path)?;

    let tracked_path = trimmed_path.to_string();
    let tracked_game_id = game_id.clone();

    app.emit(
        "game-process-state",
        GameProcessEvent {
            game_id: game_id.clone(),
            exe_path: tracked_path.clone(),
            state: "started",
            message: None,
        },
    )
    .map_err(|error| format!("Failed to emit launch state: {error}"))?;

    std::thread::spawn(move || {
        let state = match child.wait() {
            Ok(_) => GameProcessEvent {
                game_id: tracked_game_id,
                exe_path: tracked_path,
                state: "exited",
                message: None,
            },
            Err(error) => GameProcessEvent {
                game_id: tracked_game_id,
                exe_path: tracked_path,
                state: "error",
                message: Some(error.to_string()),
            },
        };

        let _ = app.emit("game-process-state", state);
    });

    Ok(format!("Launched game: {trimmed_path}"))
}
