use std::path::Path;
use std::process::Command;

#[tauri::command]
pub fn launch_game(exe_path: String) -> Result<String, String> {
    let trimmed_path = exe_path.trim();

    if trimmed_path.is_empty() {
        return Err("Executable path cannot be empty.".into());
    }

    if !Path::new(trimmed_path).exists() {
        return Err(format!("Executable not found: {trimmed_path}"));
    }

    Command::new(trimmed_path)
        .spawn()
        .map_err(|error| format!("Failed to launch game: {error}"))?;

    Ok(format!("Launched game: {trimmed_path}"))
}
