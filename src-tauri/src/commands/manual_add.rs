use std::path::Path;
use std::process::Command;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualGameCandidate {
    pub id: String,
    pub title: String,
    pub path: String,
    pub platform: String,
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut last_dash = false;

    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash {
            slug.push('-');
            last_dash = true;
        }
    }

    slug.trim_matches('-').to_string()
}

fn title_from_path(path: &Path) -> String {
    if let Some(parent_name) = path
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|value| value.to_str())
    {
        let trimmed = parent_name.trim();
        if !trimmed.is_empty() {
            return trimmed.replace('_', " ").replace('-', " ");
        }
    }

    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("Unknown Game");

    stem.replace('_', " ").replace('-', " ")
}

fn pick_executable_path() -> Result<Option<String>, String> {
    let script = r#"
Add-Type -AssemblyName System.Windows.Forms
$dialog = New-Object System.Windows.Forms.OpenFileDialog
$dialog.Filter = 'Game executables (*.exe)|*.exe'
$dialog.Title = 'Select a game executable'
$dialog.Multiselect = $false
$dialog.CheckFileExists = $true
$dialog.RestoreDirectory = $true

if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
  Write-Output $dialog.FileName
}
"#;

    let output = Command::new("powershell")
        .args(["-STA", "-NoProfile", "-Command", script])
        .output()
        .map_err(|error| format!("Failed to open executable picker: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            "Executable picker failed.".into()
        } else {
            format!("Executable picker failed: {stderr}")
        });
    }

    let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if selected.is_empty() {
        Ok(None)
    } else {
        Ok(Some(selected))
    }
}

#[tauri::command]
pub fn pick_game_executable() -> Result<Option<ManualGameCandidate>, String> {
    let Some(selected_path) = pick_executable_path()? else {
        return Ok(None);
    };

    let path = Path::new(&selected_path);
    if !path.exists() {
        return Err(format!("Selected executable does not exist: {selected_path}"));
    }

    let title = title_from_path(path);

    Ok(Some(ManualGameCandidate {
        id: format!("local-{}", slugify(&title)),
        title,
        path: selected_path.replace('/', "\\"),
        platform: "local".into(),
    }))
}
