use std::path::Path;
use std::process::Command;

fn escape_powershell_single_quotes(value: &str) -> String {
    value.replace('\'', "''")
}

fn launch_with_elevation(exe_path: &str) -> Result<u32, String> {
    println!(
        "[launch_game] using powershell elevation launch for exe_path={}",
        exe_path
    );

    let script = format!(
        "$process = Start-Process -FilePath '{}' -WorkingDirectory '{}' -Verb RunAs -PassThru; if ($null -eq $process) {{ exit 1 }}; Write-Output $process.Id",
        escape_powershell_single_quotes(exe_path),
        escape_powershell_single_quotes(
            Path::new(exe_path)
                .parent()
                .and_then(|path| path.to_str())
                .unwrap_or_default(),
        )
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
    stdout
        .trim()
        .parse::<u32>()
        .map_err(|error| format!("Failed to read administrator launch PID: {error}"))
}

pub fn spawn_game_process(exe_path: &str) -> Result<u32, String> {
    let mut command = Command::new(exe_path);

    if let Some(parent) = Path::new(exe_path).parent() {
        command.current_dir(parent);
    }

    println!(
        "[launch_game] using rust direct spawn for exe_path={}",
        exe_path
    );

    match command.spawn() {
        Ok(child) => Ok(child.id()),
        Err(error) if error.raw_os_error() == Some(740) => launch_with_elevation(exe_path),
        Err(error) => Err(format!("Failed to launch game: {error}")),
    }
}
