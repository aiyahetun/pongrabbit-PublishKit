pub mod doc_import;
pub mod md_split;
pub mod table_import;
mod api;
mod backup;
mod content_images;
mod db;
mod media;
mod media_suggest;
mod rich_text;
mod staging;

use db::{
    create_custom_channel, create_publish_task, delete_content_items, delete_content_items_for_source,
    delete_custom_channel, fields_body, get_content_item_by_id, insert_content_item, link_content_media,
    linked_media_ids, list_calendar_tasks, list_channels, list_content_items, list_media_assets,
    list_media_for_content, list_publish_tasks, unlink_content_media, update_publish_task_status,
    update_task_scheduled_at, upsert_media_asset, upsert_source_document, DbState,
};
use md_split::{anchor_json, preview_splits, SourceAnchor, SplitPreview, SplitStrategy};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tauri::Manager;
use tauri_plugin_opener::OpenerExt;
use uuid::Uuid;
use walkdir::WalkDir;

const IGNORE_DIRS: &[&str] = &["node_modules", ".git", ".cursor", "target", "dist"];
const MANUAL_SOURCE_PATH: &str = "manual://publishkit";
const IMAGE_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "webp", "svg", "bmp", "heic", "heif", "tif", "tiff",
];
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mov", "webm", "avi", "mkv", "m4v"];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSettings {
    pub ui_locale: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_root: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_root: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_port: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pairing_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownScanItem {
    pub path: String,
    pub title: String,
    pub size_bytes: u64,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentItemRow {
    pub id: String,
    pub title: String,
    pub source_path: String,
    pub language: String,
    pub body: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportSplitsResult {
    pub imported_count: usize,
    pub content_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelRow {
    pub id: String,
    pub name: String,
    pub market: String,
    pub color: String,
    pub is_custom: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskContentRef {
    pub id: String,
    pub title: String,
    pub language: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskChannelRef {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishTaskRow {
    pub id: String,
    pub status: String,
    pub publish_url: String,
    pub note: String,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub scheduled_at: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub published_at: String,
    pub channel: TaskChannelRef,
    pub content: TaskContentRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaAssetRow {
    pub id: String,
    pub path: String,
    pub file_name: String,
    pub kind: String,
    pub size_bytes: u64,
    pub indexed_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumb_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbBatchResult {
    pub generated: usize,
    pub remaining: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSuggestionRow {
    pub id: String,
    pub path: String,
    pub file_name: String,
    pub kind: String,
    pub size_bytes: u64,
    pub thumb_path: Option<String>,
    pub reason: String,
    pub score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportContentPackResult {
    pub folder_path: String,
    pub media_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportTasksCsvResult {
    pub path: String,
    pub row_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StageImagesResult {
    pub folder_path: String,
    pub copied_count: usize,
    pub is_temporary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteContentResult {
    pub deleted_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportBackupResult {
    pub path: String,
    pub file_count: usize,
    pub includes_thumbs: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportBackupResult {
    pub file_count: usize,
    pub includes_thumbs: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiStatusRow {
    pub port: u16,
    pub pairing_token: String,
    pub base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanMediaResult {
    pub indexed_count: usize,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEntryRow {
    pub id: String,
    pub status: String,
    pub date: String,
    pub channel_name: String,
    pub channel_color: String,
    pub content_title: String,
    pub publish_url: String,
}

fn map_task_rows(
    rows: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )>,
) -> Vec<PublishTaskRow> {
    rows.into_iter()
        .map(|row| PublishTaskRow {
            id: row.0.clone(),
            status: row.1.clone(),
            publish_url: row.2.clone(),
            note: row.3.clone(),
            updated_at: row.4.clone(),
            scheduled_at: row.5.clone(),
            published_at: row.6.clone(),
            channel: TaskChannelRef {
                id: row.7.clone(),
                name: row.8.clone(),
                color: row.9.clone(),
            },
            content: TaskContentRef {
                id: row.10.clone(),
                title: row.11.clone(),
                language: row.12.clone(),
                body: fields_body(&row.13),
            },
        })
        .collect()
}

fn parse_scheduled_date(date: &str) -> Result<String, String> {
    let trimmed = date.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    if trimmed.len() != 10 || !trimmed.chars().all(|c| c.is_ascii_digit() || c == '-') {
        return Err("日期格式应为 YYYY-MM-DD".into());
    }
    Ok(format!("{trimmed}T12:00:00Z"))
}

fn scheduled_date_input(iso: &str) -> String {
    if iso.len() >= 10 {
        iso[..10].to_string()
    } else {
        String::new()
    }
}

fn sanitize_folder_name(name: &str) -> String {
    let invalid = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    let cleaned: String = name
        .chars()
        .map(|c| if invalid.contains(&c) { '_' } else { c })
        .collect();
    let trimmed = cleaned.trim().trim_end_matches('.').to_string();
    if trimmed.is_empty() {
        "content".to_string()
    } else {
        trimmed
    }
}

fn build_media_row(
    app: &tauri::AppHandle,
    id: &str,
    path: &str,
    file_name: &str,
    kind: &str,
    size_bytes: u64,
    indexed_at: &str,
) -> MediaAssetRow {
    let thumb_path = media::thumb_path_if_exists(app, id, kind);
    MediaAssetRow {
        id: id.to_string(),
        path: path.to_string(),
        file_name: file_name.to_string(),
        kind: kind.to_string(),
        size_bytes,
        indexed_at: indexed_at.to_string(),
        thumb_path,
    }
}

fn settings_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("settings.json"))
}

pub fn load_settings(app: &tauri::AppHandle) -> Result<WorkspaceSettings, String> {
    let path = settings_path(app)?;
    if !path.exists() {
        return Ok(WorkspaceSettings {
            ui_locale: detect_ui_locale(),
            copy_root: None,
            media_root: None,
            api_port: None,
            pairing_token: None,
        });
    }
    let raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| e.to_string())
}

fn save_settings(app: &tauri::AppHandle, settings: &WorkspaceSettings) -> Result<(), String> {
    let path = settings_path(app)?;
    let raw = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, raw).map_err(|e| e.to_string())
}

fn detect_ui_locale() -> String {
    let locale = sys_locale::get_locale().unwrap_or_else(|| "en-US".into());
    if locale.to_lowercase().starts_with("zh") {
        "zh-CN".into()
    } else {
        "en".into()
    }
}

fn normalize_locale(locale: &str) -> String {
    if locale.eq_ignore_ascii_case("zh-cn") || locale.starts_with("zh") {
        "zh-CN".into()
    } else {
        "en".into()
    }
}

fn should_skip(path: &Path) -> bool {
    path.components().any(|c| {
        if let Some(name) = c.as_os_str().to_str() {
            IGNORE_DIRS.contains(&name)
        } else {
            false
        }
    })
}

fn media_kind(ext: &str) -> Option<&'static str> {
    if IMAGE_EXTENSIONS.contains(&ext) {
        Some("image")
    } else if VIDEO_EXTENSIONS.contains(&ext) {
        Some("video")
    } else {
        None
    }
}

fn event_date(iso: &str) -> Option<String> {
    chrono::DateTime::parse_from_rfc3339(iso)
        .ok()
        .map(|dt| dt.date_naive().format("%Y-%m-%d").to_string())
}

fn title_from_markdown(path: &Path, content: &str) -> String {
    for line in content.lines().take(20) {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("# ") {
            if !rest.is_empty() {
                return rest.to_string();
            }
        }
    }
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .to_string()
}

#[tauri::command]
fn get_ui_locale(app: tauri::AppHandle) -> Result<String, String> {
    Ok(load_settings(&app)?.ui_locale)
}

#[tauri::command]
fn set_ui_locale(app: tauri::AppHandle, locale: String) -> Result<WorkspaceSettings, String> {
    let mut settings = load_settings(&app)?;
    settings.ui_locale = normalize_locale(&locale);
    save_settings(&app, &settings)?;
    Ok(settings)
}

#[tauri::command]
fn get_workspace_settings(app: tauri::AppHandle) -> Result<WorkspaceSettings, String> {
    load_settings(&app)
}

#[tauri::command]
fn set_copy_root(app: tauri::AppHandle, path: String) -> Result<WorkspaceSettings, String> {
    let mut settings = load_settings(&app)?;
    settings.copy_root = Some(path);
    save_settings(&app, &settings)?;
    Ok(settings)
}

#[tauri::command]
fn set_media_root(app: tauri::AppHandle, path: String) -> Result<WorkspaceSettings, String> {
    let mut settings = load_settings(&app)?;
    settings.media_root = Some(path);
    save_settings(&app, &settings)?;
    Ok(settings)
}

#[tauri::command]
fn scan_markdown(root: String) -> Result<Vec<MarkdownScanItem>, String> {
    let root_path = PathBuf::from(&root);
    if !root_path.is_dir() {
        return Err("路径不是文件夹".into());
    }

    let mut items = Vec::new();
    for entry in WalkDir::new(&root_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() || should_skip(path) {
            continue;
        }
        let Some(format) = doc_import::source_format(path) else {
            continue;
        };
        let meta = fs::metadata(path).map_err(|e| e.to_string())?;
        let title = if doc_import::is_split_format(format) && format != "pdf" && format != "docx" {
            let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
            title_from_markdown(path, &content)
        } else {
            db::title_from_path(path)
        };
        items.push(MarkdownScanItem {
            path: path.to_string_lossy().into_owned(),
            title,
            size_bytes: meta.len(),
            format: format.to_string(),
        });
    }

    items.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(items)
}

#[tauri::command]
fn preview_md_splits(path: String, strategy: String) -> Result<Vec<SplitPreview>, String> {
    let strategy = SplitStrategy::parse(&strategy)?;
    preview_splits(&path, strategy)
}

#[tauri::command]
fn import_md_splits(
    state: tauri::State<'_, DbState>,
    path: String,
    strategy: String,
    selected_indexes: Vec<usize>,
    replace_existing: bool,
) -> Result<ImportSplitsResult, String> {
    let strategy = SplitStrategy::parse(&strategy)?;
    let previews = preview_splits(&path, strategy)?;
    if previews.is_empty() {
        return Err("没有可导入的拆分块".into());
    }

    let selected: Vec<&SplitPreview> = if selected_indexes.is_empty() {
        previews.iter().collect()
    } else {
        previews
            .iter()
            .filter(|p| selected_indexes.contains(&p.index))
            .collect()
    };

    if selected.is_empty() {
        return Err("请至少选择一个拆分块".into());
    }

    let source_path = PathBuf::from(&path);
    let doc_title = doc_import::read_source_content(&path)
        .ok()
        .map(|content| title_from_markdown(&source_path, &content))
        .unwrap_or_else(|| db::title_from_path(&source_path));
    let doc_id = Uuid::new_v4().to_string();

    db::with_conn(&state, |conn| {
        if replace_existing {
            delete_content_items_for_source(conn, &path)?;
        }
        upsert_source_document(conn, &doc_id, &path, &doc_title)?;

        let existing_doc_id: String = conn
            .query_row(
                "SELECT id FROM source_documents WHERE path = ?1",
                [&path],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;

        let mut content_ids = Vec::new();
        for preview in selected {
            let anchor = SourceAnchor {
                start_line: preview.start_line,
                end_line: preview.end_line,
                heading: preview.heading.clone(),
            };
            let fields_json = serde_json::json!({
                "title": preview.title,
                "body": preview.body,
            })
            .to_string();
            let item_id = Uuid::new_v4().to_string();
            insert_content_item(
                conn,
                &item_id,
                &existing_doc_id,
                &path,
                &anchor_json(&anchor)?,
                &preview.title,
                &preview.language,
                &fields_json,
            )?;
            content_ids.push(item_id);
        }

        Ok(ImportSplitsResult {
            imported_count: content_ids.len(),
            content_ids,
        })
    })
}

#[tauri::command]
fn preview_table_import_cmd(path: String) -> Result<table_import::TableImportPreview, String> {
    table_import::preview_table_import(&path)
}

#[tauri::command]
fn import_table_rows_cmd(
    state: tauri::State<'_, DbState>,
    path: String,
    selected_indexes: Vec<usize>,
    replace_existing: bool,
) -> Result<ImportSplitsResult, String> {
    let rows = table_import::selected_table_rows(&path, &selected_indexes)?;
    if rows.is_empty() {
        return Err("请至少选择一行".into());
    }

    let source_path = PathBuf::from(&path);
    let doc_title = db::title_from_path(&source_path);
    let doc_id = Uuid::new_v4().to_string();

    db::with_conn(&state, |conn| {
        if replace_existing {
            delete_content_items_for_source(conn, &path)?;
        }
        upsert_source_document(conn, &doc_id, &path, &doc_title)?;

        let existing_doc_id: String = conn
            .query_row(
                "SELECT id FROM source_documents WHERE path = ?1",
                [&path],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;

        let mut content_ids = Vec::new();
        for row in rows {
            let anchor = SourceAnchor {
                start_line: row.index as u32,
                end_line: row.index as u32,
                heading: Some(format!("row:{}", row.index + 1)),
            };
            let fields_json = serde_json::json!({
                "title": row.title,
                "body": row.body,
            })
            .to_string();
            let item_id = Uuid::new_v4().to_string();
            insert_content_item(
                conn,
                &item_id,
                &existing_doc_id,
                &path,
                &anchor_json(&anchor)?,
                &row.title,
                &row.language,
                &fields_json,
            )?;
            content_ids.push(item_id);
        }

        Ok(ImportSplitsResult {
            imported_count: content_ids.len(),
            content_ids,
        })
    })
}

#[tauri::command]
fn create_manual_content(
    state: tauri::State<'_, DbState>,
    title: String,
    body: String,
    language: Option<String>,
) -> Result<ContentItemRow, String> {
    let title = title.trim().to_string();
    let body = body.trim().to_string();
    if title.is_empty() || body.is_empty() {
        return Err("标题和正文不能为空".into());
    }

    let language = language.unwrap_or_else(|| md_split::detect_language(&body));
    let item_id = Uuid::new_v4().to_string();
    let doc_id = Uuid::new_v4().to_string();
    let anchor = SourceAnchor {
        start_line: 0,
        end_line: 0,
        heading: Some("manual".into()),
    };
    let fields_json = serde_json::json!({
        "title": title,
        "body": body,
    })
    .to_string();

    db::with_conn(&state, |conn| {
        upsert_source_document(conn, &doc_id, MANUAL_SOURCE_PATH, "Manual entries")?;
        let existing_doc_id: String = conn
            .query_row(
                "SELECT id FROM source_documents WHERE path = ?1",
                [MANUAL_SOURCE_PATH],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;

        insert_content_item(
            conn,
            &item_id,
            &existing_doc_id,
            MANUAL_SOURCE_PATH,
            &anchor_json(&anchor)?,
            &title,
            &language,
            &fields_json,
        )?;

        Ok(ContentItemRow {
            id: item_id.clone(),
            title,
            source_path: MANUAL_SOURCE_PATH.to_string(),
            language,
            body,
            created_at: chrono::Utc::now().to_rfc3339(),
        })
    })
}

fn fetch_publish_tasks(state: &DbState, status: Option<String>) -> Result<Vec<PublishTaskRow>, String> {
    db::with_conn(state, |conn| {
        let rows = list_publish_tasks(conn, status.as_deref())?;
        Ok(map_task_rows(rows))
    })
}

#[tauri::command]
fn list_channels_cmd(state: tauri::State<'_, DbState>) -> Result<Vec<ChannelRow>, String> {
    db::with_conn(&state, |conn| {
        Ok(list_channels(conn)?
            .into_iter()
            .map(|(id, name, market, color, is_custom)| ChannelRow {
                id,
                name,
                market,
                color,
                is_custom: is_custom != 0,
            })
            .collect())
    })
}

#[tauri::command]
fn create_channel_cmd(
    state: tauri::State<'_, DbState>,
    name: String,
    market: String,
    color: Option<String>,
) -> Result<ChannelRow, String> {
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err("平台名称不能为空".into());
    }
    let market = match market.as_str() {
        "domestic" | "overseas" | "both" => market,
        _ => "both".to_string(),
    };
    let color = color.unwrap_or_else(|| "#8a8278".to_string());
    let id = format!("custom_{}", Uuid::new_v4());

    db::with_conn(&state, |conn| {
        create_custom_channel(conn, &id, &name, &market, &color)?;
        Ok(ChannelRow {
            id: id.clone(),
            name: name.clone(),
            market: market.clone(),
            color: color.clone(),
            is_custom: true,
        })
    })
}

#[tauri::command]
fn delete_channel_cmd(state: tauri::State<'_, DbState>, channel_id: String) -> Result<(), String> {
    db::with_conn(&state, |conn| delete_custom_channel(conn, &channel_id))
}

#[tauri::command]
fn create_publish_task_cmd(
    state: tauri::State<'_, DbState>,
    content_item_id: String,
    channel_id: String,
) -> Result<PublishTaskRow, String> {
    let task_id = Uuid::new_v4().to_string();
    db::with_conn(&state, |conn| {
        create_publish_task(conn, &task_id, &content_item_id, &channel_id)?;
        Ok(())
    })?;
    fetch_publish_tasks(&state, None).and_then(|tasks| {
        tasks
            .into_iter()
            .find(|t| t.id == task_id)
            .ok_or_else(|| "创建任务失败".into())
    })
}

#[tauri::command]
fn list_publish_tasks_cmd(
    state: tauri::State<'_, DbState>,
    status: Option<String>,
) -> Result<Vec<PublishTaskRow>, String> {
    fetch_publish_tasks(&state, status)
}

#[tauri::command]
fn list_today_tasks_cmd(state: tauri::State<'_, DbState>) -> Result<Vec<PublishTaskRow>, String> {
    fetch_publish_tasks(&state, Some("ready".into()))
}

#[tauri::command]
fn update_publish_task_status_cmd(
    state: tauri::State<'_, DbState>,
    task_id: String,
    status: String,
    publish_url: Option<String>,
    note: Option<String>,
) -> Result<PublishTaskRow, String> {
    db::with_conn(&state, |conn| {
        update_publish_task_status(
            conn,
            &task_id,
            &status,
            publish_url.as_deref(),
            note.as_deref(),
        )
    })?;
    fetch_publish_tasks(&state, None).and_then(|tasks| {
        tasks
            .into_iter()
            .find(|t| t.id == task_id)
            .ok_or_else(|| "任务不存在".into())
    })
}

#[tauri::command]
fn list_content_items_cmd(state: tauri::State<'_, DbState>) -> Result<Vec<ContentItemRow>, String> {
    db::with_conn(&state, |conn| {
        let rows = list_content_items(conn)?;
        let mut items = Vec::new();
        for (id, title, source_path, language, fields_json, created_at) in rows {
            let body = serde_json::from_str::<serde_json::Value>(&fields_json)
                .ok()
                .and_then(|v| v.get("body").and_then(|b| b.as_str()).map(str::to_string))
                .unwrap_or_default();
            items.push(ContentItemRow {
                id,
                title,
                source_path,
                language,
                body,
                created_at,
            });
        }
        Ok(items)
    })
}

#[tauri::command]
fn scan_media_cmd(app: tauri::AppHandle, state: tauri::State<'_, DbState>) -> Result<ScanMediaResult, String> {
    let settings = load_settings(&app)?;
    let root = settings
        .media_root
        .ok_or_else(|| "请先在「来源文件」选择图片文件夹".to_string())?;
    let root_path = PathBuf::from(&root);
    if !root_path.is_dir() {
        return Err("图片路径不是文件夹".into());
    }

    let mut indexed = 0usize;
    db::with_conn(&state, |conn| {
        for entry in WalkDir::new(&root_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() || should_skip(path) {
                continue;
            }
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();
            let Some(kind) = media_kind(&ext) else {
                continue;
            };
            let meta = fs::metadata(path).map_err(|e| e.to_string())?;
            let file_name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            let mtime = meta
                .modified()
                .ok()
                .map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339());
            let id = Uuid::new_v4().to_string();
            let path_str = path.to_string_lossy().into_owned();
            upsert_media_asset(
                conn,
                &id,
                &path_str,
                &file_name,
                kind,
                meta.len() as i64,
                mtime.as_deref(),
            )?;
            indexed += 1;
        }

        let total: i64 = conn
            .query_row("SELECT COUNT(*) FROM media_assets", [], |row| row.get(0))
            .map_err(|e| e.to_string())?;

        Ok(ScanMediaResult {
            indexed_count: indexed,
            total_count: total as usize,
        })
    })
}

#[tauri::command]
fn list_media_assets_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
) -> Result<Vec<MediaAssetRow>, String> {
    db::with_conn(&state, |conn| {
        Ok(list_media_assets(conn)?
            .into_iter()
            .map(|(id, path, file_name, kind, size_bytes, indexed_at)| {
                build_media_row(
                    &app,
                    &id,
                    &path,
                    &file_name,
                    &kind,
                    size_bytes as u64,
                    &indexed_at,
                )
            })
            .collect())
    })
}

#[tauri::command]
fn list_content_media_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    content_item_id: String,
) -> Result<Vec<MediaAssetRow>, String> {
    db::with_conn(&state, |conn| {
        Ok(list_media_for_content(conn, &content_item_id)?
            .into_iter()
            .map(|(id, path, file_name, kind, size_bytes)| {
                build_media_row(&app, &id, &path, &file_name, &kind, size_bytes as u64, "")
            })
            .collect())
    })
}

#[tauri::command]
fn link_content_media_cmd(
    state: tauri::State<'_, DbState>,
    content_item_id: String,
    media_asset_id: String,
) -> Result<(), String> {
    db::with_conn(&state, |conn| link_content_media(conn, &content_item_id, &media_asset_id))
}

#[tauri::command]
fn unlink_content_media_cmd(
    state: tauri::State<'_, DbState>,
    content_item_id: String,
    media_asset_id: String,
) -> Result<(), String> {
    db::with_conn(&state, |conn| unlink_content_media(conn, &content_item_id, &media_asset_id))
}

#[tauri::command]
async fn generate_media_thumbnails_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    batch_size: Option<usize>,
) -> Result<ThumbBatchResult, String> {
    let batch_size = batch_size.unwrap_or(24).clamp(1, 50);
    let assets = db::with_conn(&state, |conn| {
        Ok(list_media_assets(conn)?
            .into_iter()
            .map(|(id, path, file_name, kind, _, _)| (id, path, kind))
            .collect::<Vec<_>>())
    })?;

    let app = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let (generated, remaining) = media::generate_thumbnail_batch(&app, &assets, batch_size);
        ThumbBatchResult {
            generated,
            remaining,
        }
    })
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
fn copy_markdown_rich_text_cmd(markdown: String) -> Result<(), String> {
    rich_text::copy_markdown_rich_text(&markdown)
}

#[tauri::command]
fn copy_media_image_cmd(path: String) -> Result<(), String> {
    media::copy_image_to_clipboard(&path)
}

#[tauri::command]
fn reveal_media_in_folder_cmd(app: tauri::AppHandle, path: String) -> Result<(), String> {
    app.opener()
        .reveal_item_in_dir(&path)
        .map_err(|e| e.to_string())
}

fn csv_cell(value: &str) -> String {
    if value.contains(['"', ',', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn unique_pack_dir(dest: &Path, base_name: &str) -> PathBuf {
    let mut candidate = dest.join(base_name);
    if !candidate.exists() {
        return candidate;
    }
    for i in 2..100 {
        candidate = dest.join(format!("{base_name}-{i}"));
        if !candidate.exists() {
            return candidate;
        }
    }
    dest.join(format!(
        "{base_name}-{}",
        chrono::Utc::now().format("%Y%m%d-%H%M%S")
    ))
}

fn unique_dest_path(dir: &Path, file_name: &str) -> PathBuf {
    let mut dest = dir.join(file_name);
    if !dest.exists() {
        return dest;
    }
    let path = Path::new(file_name);
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|e| format!(".{e}"))
        .unwrap_or_default();
    for i in 2..100 {
        dest = dir.join(format!("{stem}-{i}{ext}"));
        if !dest.exists() {
            return dest;
        }
    }
    dir.join(file_name)
}

#[tauri::command]
fn suggest_media_for_content_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    content_item_id: String,
    limit: Option<usize>,
) -> Result<Vec<MediaSuggestionRow>, String> {
    let limit = limit.unwrap_or(12).clamp(1, 50);
    let settings = load_settings(&app)?;
    let copy_root = settings.copy_root.as_deref();
    let media_root = settings.media_root.as_deref();
    db::with_conn(&state, |conn| {
        let (title, source_path, _) = get_content_item_by_id(conn, &content_item_id)?;
        let linked: std::collections::HashSet<String> =
            linked_media_ids(conn, &content_item_id)?.into_iter().collect();

        let mut scored: Vec<MediaSuggestionRow> = list_media_assets(conn)?
            .into_iter()
            .filter(|(id, _, _, _, _, _)| !linked.contains(id))
            .filter_map(|(id, path, file_name, kind, size_bytes, _)| {
                let (score, reason) = media_suggest::score_media_for_content(
                    &source_path,
                    &title,
                    &path,
                    &file_name,
                    copy_root,
                    media_root,
                );
                if score <= 0 {
                    return None;
                }
                Some(MediaSuggestionRow {
                    id: id.clone(),
                    path,
                    file_name,
                    kind: kind.clone(),
                    size_bytes: size_bytes as u64,
                    thumb_path: media::thumb_path_if_exists(&app, &id, &kind),
                    reason: reason.to_string(),
                    score,
                })
            })
            .collect();

        scored.sort_by(|a, b| {
            b.score
                .cmp(&a.score)
                .then_with(|| a.file_name.cmp(&b.file_name))
        });
        scored.truncate(limit);
        Ok(scored)
    })
}

#[tauri::command]
fn update_publish_task_scheduled_cmd(
    state: tauri::State<'_, DbState>,
    task_id: String,
    scheduled_date: String,
) -> Result<(), String> {
    let scheduled_at = if scheduled_date.trim().is_empty() {
        None
    } else {
        Some(parse_scheduled_date(&scheduled_date)?)
    };
    db::with_conn(&state, |conn| {
        update_task_scheduled_at(conn, &task_id, scheduled_at.as_deref())
    })
}

#[tauri::command]
fn export_content_pack_cmd(
    state: tauri::State<'_, DbState>,
    content_item_id: String,
    dest_folder: String,
) -> Result<ExportContentPackResult, String> {
    db::with_conn(&state, |conn| {
        let (title, source_path, fields_json) = get_content_item_by_id(conn, &content_item_id)?;
        let body = fields_body(&fields_json);
        let media = list_media_for_content(conn, &content_item_id)?;

        let dest = PathBuf::from(&dest_folder);
        if !dest.is_dir() {
            return Err("目标路径不是文件夹".into());
        }

        let pack_dir = unique_pack_dir(&dest, &sanitize_folder_name(&title));
        fs::create_dir_all(&pack_dir).map_err(|e| e.to_string())?;

        let mut md = format!("# {title}\n\n");
        if !source_path.starts_with("manual://") {
            md.push_str(&format!("> Source: {source_path}\n\n"));
        }
        md.push_str(&body);
        fs::write(pack_dir.join("content.md"), md).map_err(|e| e.to_string())?;

        let mut media_count = 0;
        if !media.is_empty() {
            let media_dir = pack_dir.join("media");
            fs::create_dir_all(&media_dir).map_err(|e| e.to_string())?;
            for (_, path, file_name, _, _) in media {
                let target = unique_dest_path(&media_dir, &file_name);
                fs::copy(&path, &target)
                    .map_err(|e| format!("复制素材失败 {path}: {e}"))?;
                media_count += 1;
            }
        }

        Ok(ExportContentPackResult {
            folder_path: pack_dir.to_string_lossy().to_string(),
            media_count,
        })
    })
}

#[tauri::command]
fn export_tasks_csv_cmd(
    state: tauri::State<'_, DbState>,
    dest_path: String,
) -> Result<ExportTasksCsvResult, String> {
    db::with_conn(&state, |conn| {
        let rows = list_publish_tasks(conn, None)?;
        let mut csv = String::from(
            "id,status,channel,content_title,language,scheduled_at,published_at,publish_url,updated_at\n",
        );

        for row in &rows {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                csv_cell(&row.0),
                csv_cell(&row.1),
                csv_cell(&row.8),
                csv_cell(&row.11),
                csv_cell(&row.12),
                csv_cell(&scheduled_date_input(&row.5)),
                csv_cell(&scheduled_date_input(&row.6)),
                csv_cell(&row.2),
                csv_cell(&row.4),
            ));
        }

        fs::write(&dest_path, csv.as_bytes()).map_err(|e| e.to_string())?;
        Ok(ExportTasksCsvResult {
            path: dest_path,
            row_count: rows.len(),
        })
    })
}

#[tauri::command]
fn stage_content_images_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    content_item_id: String,
) -> Result<StageImagesResult, String> {
    db::with_conn(&state, |conn| {
        let (title, _, _) = get_content_item_by_id(conn, &content_item_id)?;
        let images = content_images::list_content_image_paths(conn, &content_item_id)?;
        let (dir, copied_count) = content_images::stage_images(&app, &title, &images.paths)?;
        Ok(StageImagesResult {
            folder_path: dir.to_string_lossy().to_string(),
            copied_count,
            is_temporary: true,
        })
    })
}

#[tauri::command]
fn delete_content_item_cmd(
    state: tauri::State<'_, DbState>,
    content_item_id: String,
) -> Result<DeleteContentResult, String> {
    db::with_conn(&state, |conn| {
        let deleted = delete_content_items(conn, std::slice::from_ref(&content_item_id))?;
        if deleted == 0 {
            return Err("内容条目不存在".into());
        }
        Ok(DeleteContentResult { deleted_count: deleted })
    })
}

#[tauri::command]
fn delete_content_items_cmd(
    state: tauri::State<'_, DbState>,
    content_item_ids: Vec<String>,
) -> Result<DeleteContentResult, String> {
    db::with_conn(&state, |conn| {
        let deleted = delete_content_items(conn, &content_item_ids)?;
        if deleted == 0 {
            return Err("没有可删除的内容条目".into());
        }
        Ok(DeleteContentResult { deleted_count: deleted })
    })
}

#[tauri::command]
fn export_backup_cmd(
    app: tauri::AppHandle,
    dest_path: String,
    include_thumbs: Option<bool>,
) -> Result<ExportBackupResult, String> {
    let include_thumbs = include_thumbs.unwrap_or(false);
    let db_path = db::db_file_path(&app)?;
    let settings = settings_path(&app)?;
    let thumb_dir = media::thumb_cache_dir(&app)?;

    let summary = backup::export_backup_zip(
        &app,
        Path::new(&dest_path),
        &db_path,
        &settings,
        &thumb_dir,
        include_thumbs,
    )?;

    Ok(ExportBackupResult {
        path: dest_path,
        file_count: summary.file_count,
        includes_thumbs: include_thumbs,
    })
}

#[tauri::command]
fn import_backup_cmd(app: tauri::AppHandle, zip_path: String) -> Result<ImportBackupResult, String> {
    let summary = backup::stage_restore_from_zip(&app, Path::new(&zip_path))?;
    app.restart();
    #[allow(unreachable_code)]
    Ok(ImportBackupResult {
        file_count: summary.file_count,
        includes_thumbs: summary.includes_thumbs,
    })
}

#[tauri::command]
fn list_calendar_entries_cmd(
    state: tauri::State<'_, DbState>,
    year: i32,
    month: u32,
) -> Result<Vec<CalendarEntryRow>, String> {
    if !(1..=12).contains(&month) {
        return Err("月份无效".into());
    }
    db::with_conn(&state, |conn| {
        Ok(list_calendar_tasks(conn, year, month)?
            .into_iter()
            .filter_map(|(id, status, event_at, channel_name, channel_color, content_title, publish_url)| {
                event_date(&event_at).map(|date| CalendarEntryRow {
                    id,
                    status,
                    date,
                    channel_name,
                    channel_color,
                    content_title,
                    publish_url,
                })
            })
            .collect())
    })
}

fn ensure_api_settings(app: &tauri::AppHandle) -> Result<(u16, String), String> {
    let mut settings = load_settings(app)?;
    let mut changed = false;
    if settings.pairing_token.is_none() {
        settings.pairing_token = Some(api::generate_pairing_token());
        changed = true;
    }
    if settings.api_port.is_none() {
        settings.api_port = Some(api::pick_api_port()?);
        changed = true;
    }
    if changed {
        save_settings(app, &settings)?;
    }
    Ok((
        settings.api_port.unwrap_or(17345),
        settings
            .pairing_token
            .unwrap_or_else(api::generate_pairing_token),
    ))
}

#[tauri::command]
fn get_api_status_cmd(app: tauri::AppHandle) -> Result<ApiStatusRow, String> {
    let (port, token) = ensure_api_settings(&app)?;
    Ok(ApiStatusRow {
        port,
        pairing_token: token,
        base_url: format!("http://127.0.0.1:{port}"),
    })
}

#[tauri::command]
fn regenerate_pairing_token_cmd(app: tauri::AppHandle) -> Result<ApiStatusRow, String> {
    let mut settings = load_settings(&app)?;
    settings.pairing_token = Some(api::generate_pairing_token());
    save_settings(&app, &settings)?;
    get_api_status_cmd(app)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            backup::apply_pending_restore_if_any(app.handle())?;
            db::init(app.handle())?;
            staging::run_startup_staging_cleanup(app.handle());
            let (port, token) = ensure_api_settings(app.handle())?;
            api::start_server(app.handle().clone(), port, token);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_ui_locale,
            set_ui_locale,
            get_workspace_settings,
            set_copy_root,
            set_media_root,
            scan_markdown,
            preview_md_splits,
            import_md_splits,
            preview_table_import_cmd,
            import_table_rows_cmd,
            create_manual_content,
            list_content_items_cmd,
            list_channels_cmd,
            create_channel_cmd,
            delete_channel_cmd,
            create_publish_task_cmd,
            list_publish_tasks_cmd,
            list_today_tasks_cmd,
            update_publish_task_status_cmd,
            scan_media_cmd,
            list_media_assets_cmd,
            list_content_media_cmd,
            link_content_media_cmd,
            unlink_content_media_cmd,
            list_calendar_entries_cmd,
            generate_media_thumbnails_cmd,
            copy_media_image_cmd,
            copy_markdown_rich_text_cmd,
            reveal_media_in_folder_cmd,
            suggest_media_for_content_cmd,
            update_publish_task_scheduled_cmd,
            export_content_pack_cmd,
            export_tasks_csv_cmd,
            stage_content_images_cmd,
            delete_content_item_cmd,
            delete_content_items_cmd,
            export_backup_cmd,
            import_backup_cmd,
            get_api_status_cmd,
            regenerate_pairing_token_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
