use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};
use tauri::{AppHandle, Manager};

const TEMP_ROOT_NAME: &str = "PublishKit";
const STAGING_MAX_AGE_SECS: u64 = 24 * 3600;

pub fn temp_staging_root() -> PathBuf {
    std::env::temp_dir().join(TEMP_ROOT_NAME)
}

pub fn prepare_temp_post_folder(title_slug: &str) -> Result<PathBuf, String> {
    let root = temp_staging_root();
    fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    purge_post_folders(&root)?;

    let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
    let dir = root.join(format!("post-{stamp}-{title_slug}"));
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn purge_post_folders(root: &Path) -> Result<(), String> {
    if !root.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir()
            && path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("post-"))
        {
            let _ = fs::remove_dir_all(&path);
        }
    }
    Ok(())
}

pub fn cleanup_temp_post_folders(root: &Path, keep: Option<&Path>) -> Result<usize, String> {
    if !root.is_dir() {
        return Ok(0);
    }
    let now = SystemTime::now();
    let mut removed = 0usize;
    for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        if keep.is_some_and(|keep_path| keep_path == path) {
            continue;
        }
        let should_remove = match entry.metadata().and_then(|meta| meta.modified()) {
            Ok(modified) => now
                .duration_since(modified)
                .map(|age| age.as_secs() >= STAGING_MAX_AGE_SECS)
                .unwrap_or(true),
            Err(_) => true,
        };
        if should_remove && fs::remove_dir_all(&path).is_ok() {
            removed += 1;
        }
    }
    Ok(removed)
}

pub fn cleanup_all_temp_post_folders() -> Result<usize, String> {
    cleanup_temp_post_folders(&temp_staging_root(), None)
}

pub fn cleanup_legacy_app_staging(app: &AppHandle) -> Result<(), String> {
    let legacy = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("staging");
    if legacy.exists() {
        fs::remove_dir_all(&legacy).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn run_startup_staging_cleanup(app: &AppHandle) {
    let _ = cleanup_legacy_app_staging(app);
    let _ = cleanup_all_temp_post_folders();
}
