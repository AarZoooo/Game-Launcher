use std::path::Path;
use std::process::Command;

fn escape_powershell_single_quotes(value: &str) -> String {
    value.replace('\'', "''")
}

fn launch_with_elevation(exe_path: &str) -> Result<(), String> {
    let script = format!(
        "Start-Process -FilePath '{}' -Verb RunAs",
        escape_powershell_single_quotes(exe_path)
    );

    let status = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .status()
        .map_err(|error| format!("Failed to request administrator launch: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err("Administrator launch was cancelled or could not be completed.".into())
    }
}

#[tauri::command]
pub fn launch_game(exe_path: String) -> Result<String, String> {
    let trimmed_path = exe_path.trim();

    if trimmed_path.is_empty() {
        return Err("Executable path cannot be empty.".into());
    }

    if !Path::new(trimmed_path).exists() {
        return Err(format!("Executable not found: {trimmed_path}"));
    }

    match Command::new(trimmed_path).spawn() {
        Ok(_) => {}
        Err(error) if error.raw_os_error() == Some(740) => {
            launch_with_elevation(trimmed_path)?;
        }
        Err(error) => return Err(format!("Failed to launch game: {error}")),
    }

    Ok(format!("Launched game: {trimmed_path}"))
}
