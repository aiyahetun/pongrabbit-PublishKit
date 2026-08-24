use image::ImageReader;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tauri::{AppHandle, Manager};

const THUMB_MAX: u32 = 320;

pub fn thumb_cache_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("cache")
        .join("thumbs");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

pub fn thumb_file_path(app: &AppHandle, asset_id: &str) -> Result<PathBuf, String> {
    Ok(thumb_cache_dir(app)?.join(format!("{asset_id}.webp")))
}

pub fn generate_thumbnail(source: &Path, dest: &Path) -> Result<(), String> {
    if !source.is_file() {
        return Err("源文件不存在".into());
    }
    let img = ImageReader::open(source)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;
    let thumb = img.thumbnail(THUMB_MAX, THUMB_MAX);
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    thumb
        .save_with_format(dest, image::ImageFormat::WebP)
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn thumb_path_if_exists(app: &AppHandle, asset_id: &str, kind: &str) -> Option<String> {
    if kind != "image" {
        return None;
    }
    let dest = thumb_file_path(app, asset_id).ok()?;
    if dest.exists() {
        dest.to_str().map(str::to_string)
    } else {
        None
    }
}

pub fn ensure_thumbnail(
    app: &AppHandle,
    asset_id: &str,
    source_path: &str,
    kind: &str,
) -> Option<String> {
    if kind != "image" {
        return None;
    }
    if let Some(existing) = thumb_path_if_exists(app, asset_id, kind) {
        return Some(existing);
    }
    let source = PathBuf::from(source_path);
    let dest = thumb_file_path(app, asset_id).ok()?;
    generate_thumbnail(&source, &dest).ok()?;
    dest.to_str().map(str::to_string)
}

pub fn generate_thumbnail_batch(
    app: &AppHandle,
    assets: &[(String, String, String)],
    batch_size: usize,
) -> (usize, usize) {
    let missing: Vec<(&String, &String)> = assets
        .iter()
        .filter(|(id, _, kind)| kind == "image" && thumb_path_if_exists(app, id, kind).is_none())
        .map(|(id, path, _)| (id, path))
        .collect();

    let mut generated = 0usize;
    for (id, path) in missing.iter().take(batch_size) {
        if ensure_thumbnail(app, id, path, "image").is_some() {
            generated += 1;
        }
    }
    let remaining = missing.len().saturating_sub(generated);
    (generated, remaining)
}

pub fn copy_image_to_clipboard(path: &str) -> Result<(), String> {
    let img = ImageReader::open(path)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    clipboard
        .set_image(arboard::ImageData {
            width: width as usize,
            height: height as usize,
            bytes: rgba.into_raw().into(),
        })
        .map_err(|e| e.to_string())?;
    Ok(())
}
