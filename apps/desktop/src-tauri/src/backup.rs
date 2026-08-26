use std::{
    fs::{self, File},
    io::{copy, Write},
    path::{Path, PathBuf},
};
use tauri::{AppHandle, Manager};
use walkdir::WalkDir;
use zip::read::ZipArchive;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

pub struct BackupSummary {
    pub file_count: usize,
}

pub struct ImportBackupSummary {
    pub file_count: usize,
    pub includes_thumbs: bool,
}

const RESTORE_PENDING_DIR: &str = "restore_pending";
fn zip_unix_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn add_file_to_zip<W: Write + std::io::Seek>(
    zip: &mut ZipWriter<W>,
    source: &Path,
    entry_name: &str,
) -> Result<(), String> {
    let mut file = File::open(source).map_err(|e| format!("无法读取 {source:?}: {e}"))?;
    zip.start_file(
        entry_name,
        SimpleFileOptions::default()
            .compression_method(CompressionMethod::Deflated),
    )
    .map_err(|e| e.to_string())?;
    std::io::copy(&mut file, zip).map_err(|e| e.to_string())?;
    Ok(())
}

fn add_dir_to_zip<W: Write + std::io::Seek>(
    zip: &mut ZipWriter<W>,
    dir: &Path,
    prefix: &str,
) -> Result<usize, String> {
    if !dir.is_dir() {
        return Ok(0);
    }
    let mut count = 0usize;
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let rel = path
            .strip_prefix(dir)
            .map_err(|e| e.to_string())?;
        let entry_name = if prefix.is_empty() {
            zip_unix_path(rel)
        } else {
            format!("{prefix}/{}", zip_unix_path(rel))
        };
        add_file_to_zip(zip, path, &entry_name)?;
        count += 1;
    }
    Ok(count)
}

pub fn export_backup_zip(
    app: &AppHandle,
    dest_zip: &Path,
    db_path: &Path,
    settings_path: &Path,
    thumb_dir: &Path,
    include_thumbs: bool,
) -> Result<BackupSummary, String> {
    if let Some(parent) = dest_zip.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let file = File::create(dest_zip).map_err(|e| e.to_string())?;
    let mut zip = ZipWriter::new(file);
    let mut file_count = 0usize;

    if db_path.is_file() {
        add_file_to_zip(&mut zip, db_path, "publishkit.db")?;
        file_count += 1;
    } else {
        return Err("数据库文件不存在".into());
    }

    if settings_path.is_file() {
        add_file_to_zip(&mut zip, settings_path, "settings.json")?;
        file_count += 1;
    }

    let manifest = serde_json::json!({
        "app": "PublishKit",
        "backupVersion": 1,
        "appVersion": env!("CARGO_PKG_VERSION"),
        "createdAt": chrono::Utc::now().to_rfc3339(),
        "includesThumbs": include_thumbs,
    });
    let manifest_raw = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;
    zip.start_file(
        "manifest.json",
        SimpleFileOptions::default()
            .compression_method(CompressionMethod::Deflated),
    )
    .map_err(|e| e.to_string())?;
    zip.write_all(manifest_raw.as_bytes())
        .map_err(|e| e.to_string())?;
    file_count += 1;

    if include_thumbs {
        file_count += add_dir_to_zip(&mut zip, thumb_dir, "cache/thumbs")?;
    }

    zip.finish().map_err(|e| e.to_string())?;
    let _ = app;
    Ok(BackupSummary { file_count })
}

fn restore_pending_dir(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(RESTORE_PENDING_DIR))
}

fn safe_zip_entry_path(name: &str) -> Result<PathBuf, String> {
    if name.contains("..") || name.starts_with('/') || name.starts_with('\\') {
        return Err(format!("备份包包含非法路径: {name}"));
    }
    Ok(PathBuf::from(name.replace('\\', "/")))
}

fn extract_zip_to_dir(zip_path: &Path, dest: &Path) -> Result<usize, String> {
    if dest.exists() {
        fs::remove_dir_all(dest).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(dest).map_err(|e| e.to_string())?;

    let file = File::open(zip_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
    let mut file_count = 0usize;

    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).map_err(|e| e.to_string())?;
        let entry_path = safe_zip_entry_path(entry.name())?;
        let out_path = dest.join(entry_path);
        if entry.is_dir() {
            fs::create_dir_all(&out_path).map_err(|e| e.to_string())?;
            continue;
        }
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut out = File::create(&out_path).map_err(|e| e.to_string())?;
        copy(&mut entry, &mut out).map_err(|e| e.to_string())?;
        file_count += 1;
    }

    Ok(file_count)
}

fn validate_restore_manifest(pending: &Path) -> Result<bool, String> {
    let manifest_path = pending.join("manifest.json");
    if !manifest_path.is_file() {
        return Ok(false);
    }
    let raw = fs::read_to_string(&manifest_path).map_err(|e| e.to_string())?;
    let value: serde_json::Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    if value.get("app").and_then(|v| v.as_str()) != Some("PublishKit") {
        return Err("备份包 manifest 无效".into());
    }
    Ok(value
        .get("includesThumbs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false))
}

pub fn stage_restore_from_zip(app: &AppHandle, zip_path: &Path) -> Result<ImportBackupSummary, String> {
    if !zip_path.is_file() {
        return Err("备份文件不存在".into());
    }

    let pending = restore_pending_dir(app)?;
    let file_count = extract_zip_to_dir(zip_path, &pending)?;

    if !pending.join("publishkit.db").is_file() {
        fs::remove_dir_all(&pending).ok();
        return Err("备份包缺少 publishkit.db".into());
    }

    let includes_thumbs = validate_restore_manifest(&pending)?;
    Ok(ImportBackupSummary {
        file_count,
        includes_thumbs,
    })
}

pub fn apply_pending_restore_if_any(app: &AppHandle) -> Result<bool, String> {
    let pending = restore_pending_dir(app)?;
    let db_src = pending.join("publishkit.db");
    if !db_src.is_file() {
        return Ok(false);
    }

    let app_data = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&app_data).map_err(|e| e.to_string())?;

    let db_dest = app_data.join("publishkit.db");
    if db_dest.exists() {
        fs::copy(&db_dest, app_data.join("publishkit.db.pre-restore"))
            .map_err(|e| e.to_string())?;
    }

    fs::copy(&db_src, &db_dest).map_err(|e| e.to_string())?;

    let settings_src = pending.join("settings.json");
    if settings_src.is_file() {
        let settings_dest = app_data.join("settings.json");
        if settings_dest.exists() {
            fs::copy(&settings_dest, app_data.join("settings.json.pre-restore"))
                .map_err(|e| e.to_string())?;
        }
        fs::copy(&settings_src, &settings_dest).map_err(|e| e.to_string())?;
    }

    let thumbs_src = pending.join("cache").join("thumbs");
    if thumbs_src.is_dir() {
        let thumbs_dest = app_data.join("cache").join("thumbs");
        if thumbs_dest.exists() {
            fs::remove_dir_all(&thumbs_dest).map_err(|e| e.to_string())?;
        }
        copy_dir_recursive(&thumbs_src, &thumbs_dest)?;
    }

    fs::remove_dir_all(&pending).map_err(|e| e.to_string())?;
    Ok(true)
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<(), String> {
    fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let from = entry.path();
        let to = dest.join(entry.file_name());
        if from.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            fs::copy(&from, &to).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}