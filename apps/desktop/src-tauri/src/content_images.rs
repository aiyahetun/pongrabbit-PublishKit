use crate::{db::list_media_for_content, media, staging};
use rusqlite::Connection;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

pub struct ContentImagePaths {
    pub paths: Vec<String>,
}

pub fn list_content_image_paths(
    conn: &Connection,
    content_item_id: &str,
) -> Result<ContentImagePaths, String> {
    let paths = list_media_for_content(conn, content_item_id)?
        .into_iter()
        .filter(|(_, _, _, kind, _)| kind == "image")
        .map(|(_, path, _, _, _)| path)
        .collect();
    Ok(ContentImagePaths { paths })
}

pub fn copy_first_image(paths: &[String]) -> Result<(String, usize), String> {
    let path = paths
        .first()
        .ok_or_else(|| "没有可复制的关联图片".to_string())?;
    if !Path::new(path).is_file() {
        return Err("关联图片文件不存在或无法读取".into());
    }
    media::copy_image_to_clipboard(path)?;
    let file_name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("image")
        .to_string();
    Ok((file_name, paths.len()))
}

pub fn stage_images(
    app: &AppHandle,
    title: &str,
    paths: &[String],
) -> Result<(PathBuf, usize), String> {
    if paths.is_empty() {
        return Err("没有可复制的关联图片".into());
    }

    let folder_name = sanitize_folder_name(title);
    let dir = staging::prepare_temp_post_folder(&folder_name)?;
    let mut copied_count = 0usize;

    for path in paths {
        if !Path::new(path).is_file() {
            continue;
        }
        let file_name = Path::new(path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("image.jpg");
        let dest = unique_dest_path(&dir, file_name);
        fs::copy(path, &dest).map_err(|e| format!("复制图片失败 {path}: {e}"))?;
        copied_count += 1;
    }

    if copied_count == 0 {
        return Err("关联图片文件不存在或无法读取".into());
    }

    app.opener()
        .reveal_item_in_dir(&dir)
        .map_err(|e| e.to_string())?;

    Ok((dir, copied_count))
}

fn sanitize_folder_name(name: &str) -> String {
    let mut slug: String = name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect();
    while slug.contains("--") {
        slug = slug.replace("--", "-");
    }
    slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        "post".into()
    } else {
        slug.chars().take(48).collect()
    }
}

fn unique_dest_path(dir: &Path, file_name: &str) -> PathBuf {
    let mut candidate = dir.join(file_name);
    if !candidate.exists() {
        return candidate;
    }
    let stem = Path::new(file_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("image");
    let ext = Path::new(file_name)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("jpg");
    for index in 2..100 {
        candidate = dir.join(format!("{stem}-{index}.{ext}"));
        if !candidate.exists() {
            return candidate;
        }
    }
    dir.join(format!("{stem}-copy.{ext}"))
}
