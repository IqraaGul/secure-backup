use crate::backup::{create_local_backup, list_local_backups};
use crate::models::{BackupManifest, BackupResult, CommandResult, FolderInfo};
use std::fs;
use std::path::Path;

#[tauri::command]
pub fn inspect_folder(path: String) -> CommandResult<FolderInfo> {
    let p = Path::new(&path);
    if !p.exists() {
        return CommandResult {
            success: false,
            data: None,
            error: Some("The selected path does not exist.".to_string()),
        };
    }

    if !p.is_dir() {
        return CommandResult {
            success: false,
            data: None,
            error: Some("The selected path is not a directory.".to_string()),
        };
    }

    let folder_name = p
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| path.clone());

    let mut file_count = 0;
    let mut total_size_bytes = 0;

    if let Ok(entries) = fs::read_dir(p) {
        for entry in entries.flatten() {
            file_count += 1;
            if let Ok(meta) = entry.metadata() {
                total_size_bytes += meta.len();
            }
        }
    }

    CommandResult {
        success: true,
        data: Some(FolderInfo {
            path,
            name: folder_name,
            exists: true,
            is_dir: true,
            file_count,
            total_size_bytes,
        }),
        error: None,
    }
}

#[tauri::command]
pub fn start_local_backup(source_path: String) -> CommandResult<BackupResult> {
    match create_local_backup(&source_path) {
        Ok(result) => CommandResult {
            success: true,
            data: Some(result),
            error: None,
        },
        Err(err) => CommandResult {
            success: false,
            data: None,
            error: Some(format!("Backup failed: {}", err)),
        },
    }
}

#[tauri::command]
pub fn get_backup_history() -> CommandResult<Vec<BackupManifest>> {
    match list_local_backups() {
        Ok(manifests) => CommandResult {
            success: true,
            data: Some(manifests),
            error: None,
        },
        Err(err) => CommandResult {
            success: false,
            data: None,
            error: Some(format!("Failed to retrieve backup history: {}", err)),
        },
    }
}
