pub mod doc_import;
pub mod md_split;
pub mod table_import;
mod batch_import;
mod import_template;
mod content_fields;
mod license;
mod api;
mod backup;
mod channel_pack;
mod content_images;
mod db;
mod media;
mod media_suggest;
mod rich_text;
mod staging;

use db::{
    channel_keyword_hash, content_fields_json, count_content_items, create_custom_channel, create_publish_task,
    delete_content_items, delete_content_items_for_source, delete_custom_channel,
    duplicate_publish_warning, fields_body, fields_keywords, get_content_item_by_id,
    insert_content_item, link_content_media, linked_media_ids, list_calendar_tasks,
    list_calendar_week_tasks, list_channels, list_content_items, list_media_assets,
    list_media_content_usages, list_media_for_content, list_publish_tasks, project_label_for_content,
    reset_failed_thumbs, save_media_thumb_state, search_content_items, unlink_content_media,
    set_channel_keyword_hash, update_publish_task_status, update_task_checklist, update_task_note, update_task_scheduled_at,
    upsert_media_asset, upsert_source_document, DbState, DuplicatePublishWarning,
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
    pub video_root: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_port: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pairing_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_project_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand_domestic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand_overseas: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scan_ignore_dirs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_key: Option<String>,
    #[serde(default)]
    pub onboarding_done: bool,
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
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub project_id: String,
    #[serde(default)]
    pub project_name: String,
    #[serde(default)]
    pub project_color: String,
    #[serde(default)]
    pub pair_id: String,
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
    #[serde(default)]
    pub keyword_hash: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskContentRef {
    pub id: String,
    pub title: String,
    pub language: String,
    pub body: String,
    #[serde(default)]
    pub keywords: Vec<String>,
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
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub blocked_reason: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checklist: Vec<String>,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub scheduled_at: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub published_at: String,
    pub channel: TaskChannelRef,
    pub content: TaskContentRef,
    #[serde(default)]
    pub project_id: String,
    #[serde(default)]
    pub project_name: String,
    #[serde(default)]
    pub project_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaContentUsageRow {
    pub media_asset_id: String,
    pub content_id: String,
    pub content_title: String,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub thumb_status: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub thumb_error: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub project_id: String,
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
    pub mode: String,
    pub file_count: usize,
    pub includes_thumbs: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged: Option<backup::MergeBackupSummary>,
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
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub project_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseStatusRow {
    pub tier: String,
    pub content_count: usize,
    pub content_limit: usize,
    pub is_pro: bool,
}

fn parse_checklist(raw: &str) -> Vec<String> {
    serde_json::from_str(raw).unwrap_or_default()
}

fn ensure_can_add_content(app: &tauri::AppHandle, state: &DbState, add_count: usize) -> Result<(), String> {
    let settings = load_settings(app)?;
    if license::is_pro(settings.license_key.as_deref()) {
        return Ok(());
    }
    let count = db::with_conn(state, |conn| count_content_items(conn))?;
    if count + add_count > license::FREE_CONTENT_LIMIT {
        return Err(format!(
            "免费版最多 {} 条内容，请在设置中激活 Pro 许可证",
            license::FREE_CONTENT_LIMIT
        ));
    }
    Ok(())
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
            blocked_reason: row.4.clone(),
            checklist: parse_checklist(&row.15),
            updated_at: row.5.clone(),
            scheduled_at: row.6.clone(),
            published_at: row.7.clone(),
            channel: TaskChannelRef {
                id: row.8.clone(),
                name: row.9.clone(),
                color: row.10.clone(),
            },
            content: TaskContentRef {
                id: row.11.clone(),
                title: row.12.clone(),
                language: row.13.clone(),
                body: fields_body(&row.14),
                keywords: fields_keywords(&row.14),
            },
            project_id: String::new(),
            project_name: String::new(),
            project_color: String::new(),
        })
        .collect()
}

fn attach_project_labels(conn: &rusqlite::Connection, tasks: &mut [PublishTaskRow]) {
    for task in tasks {
        if let Ok((project_id, project_name, project_color)) =
            project_label_for_content(conn, &task.content.id)
        {
            task.project_id = project_id;
            task.project_name = project_name;
            task.project_color = project_color;
        }
    }
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

fn build_media_row(app: &tauri::AppHandle, row: &db::MediaListRow) -> MediaAssetRow {
    let thumb_path = media::thumb_path_if_exists(app, &row.id, &row.kind);
    let mut status = row.thumb_status.clone();
    if thumb_path.is_some() && status != "failed" {
        status = "ready".into();
    } else if status.is_empty() {
        status = if row.kind == "image" { "pending".into() } else { String::new() };
    }
    MediaAssetRow {
        id: row.id.clone(),
        path: row.path.clone(),
        file_name: row.file_name.clone(),
        kind: row.kind.clone(),
        size_bytes: row.size_bytes as u64,
        indexed_at: row.indexed_at.clone(),
        thumb_path,
        width: row.width,
        height: row.height,
        thumb_status: status,
        thumb_error: row.thumb_error.clone(),
        project_id: row.project_id.clone(),
    }
}

fn active_project_id(app: &tauri::AppHandle, conn: &rusqlite::Connection) -> Result<String, String> {
    let settings = load_settings(app)?;
    if let Some(id) = settings.current_project_id.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
        let active: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM projects WHERE id = ?1 AND archived_at IS NULL",
                [id],
                |row| row.get(0),
            )
            .unwrap_or(0);
        if active > 0 {
            return Ok(id.to_string());
        }
    }
    db::ensure_default_project(conn, settings.project_name.as_deref().unwrap_or("默认项目"))
}

fn remember_project_roots(
    conn: &rusqlite::Connection,
    project_id: &str,
    kind: &str,
    path: &str,
    force: bool,
) -> Result<(), String> {
    db::upsert_project_root(conn, project_id, kind, path, force)
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
            video_root: None,
            api_port: None,
            pairing_token: None,
            project_name: Some("PublishKit".into()),
            current_project_id: None,
            brand_domestic: None,
            brand_overseas: None,
            scan_ignore_dirs: Vec::new(),
            license_key: None,
            onboarding_done: false,
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

fn should_skip(path: &Path, extra_ignore: &[String]) -> bool {
    path.components().any(|c| {
        if let Some(name) = c.as_os_str().to_str() {
            if IGNORE_DIRS.contains(&name) {
                return true;
            }
            extra_ignore.iter().any(|item| item == name)
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

/// Event date for ready tasks: scheduled date if set, otherwise last updated.
fn ready_task_event_date(scheduled_at: &str, updated_at: &str) -> Option<String> {
    let iso = if scheduled_at.is_empty() {
        updated_at
    } else {
        scheduled_at
    };
    event_date(iso)
}

fn today_local_key() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

/// Ready tasks that should appear in today's queue: due today or overdue, not future.
pub(crate) fn filter_today_queue_tasks(mut tasks: Vec<PublishTaskRow>) -> Vec<PublishTaskRow> {
    let today = today_local_key();
    tasks.retain(|task| {
        ready_task_event_date(&task.scheduled_at, &task.updated_at)
            .map(|date| date <= today)
            .unwrap_or(false)
    });
    tasks.sort_by(|a, b| {
        let da = ready_task_event_date(&a.scheduled_at, &a.updated_at).unwrap_or_default();
        let db = ready_task_event_date(&b.scheduled_at, &b.updated_at).unwrap_or_default();
        da.cmp(&db)
    });
    tasks
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
fn set_copy_root(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    path: String,
    force: Option<bool>,
) -> Result<WorkspaceSettings, String> {
    save_project_root(&app, &state, "copy", &path, force.unwrap_or(false))
}

#[tauri::command]
fn set_media_root(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    path: String,
    force: Option<bool>,
) -> Result<WorkspaceSettings, String> {
    save_project_root(&app, &state, "media", &path, force.unwrap_or(false))
}

#[tauri::command]
fn set_video_root(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    path: String,
    force: Option<bool>,
) -> Result<WorkspaceSettings, String> {
    save_project_root(&app, &state, "video", &path, force.unwrap_or(false))
}

fn save_project_root(
    app: &tauri::AppHandle,
    state: &DbState,
    kind: &str,
    path: &str,
    force: bool,
) -> Result<WorkspaceSettings, String> {
    let mut settings = load_settings(app)?;
    let project_id = db::with_conn(state, |conn| {
        let project_id = active_project_id(app, conn)?;
        remember_project_roots(conn, &project_id, kind, path, force)?;
        Ok(project_id)
    })?;
    settings.current_project_id = Some(project_id);
    match kind {
        "copy" => settings.copy_root = Some(path.to_string()).filter(|value| !value.is_empty()),
        "media" => settings.media_root = Some(path.to_string()).filter(|value| !value.is_empty()),
        "video" => settings.video_root = Some(path.to_string()).filter(|value| !value.is_empty()),
        _ => {}
    }
    save_settings(app, &settings)?;
    Ok(settings)
}

#[tauri::command]
fn set_workspace_profile_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    project_name: String,
    brand_domestic: Option<String>,
    brand_overseas: Option<String>,
) -> Result<WorkspaceSettings, String> {
    let mut settings = load_settings(&app)?;
    let name = project_name.trim().to_string();
    let domestic = brand_domestic.map(|v| v.trim().to_string()).filter(|v| !v.is_empty());
    let overseas = brand_overseas.map(|v| v.trim().to_string()).filter(|v| !v.is_empty());
    let project_id = db::with_conn(&state, |conn| {
        let project_id = active_project_id(&app, conn)?;
        db::update_project(
            conn,
            &project_id,
            &name,
            domestic.as_deref(),
            overseas.as_deref(),
        )?;
        Ok(project_id)
    })?;
    settings.current_project_id = Some(project_id);
    settings.project_name = Some(name);
    settings.brand_domestic = domestic;
    settings.brand_overseas = overseas;
    save_settings(&app, &settings)?;
    Ok(settings)
}

#[tauri::command]
fn set_scan_ignore_dirs_cmd(
    app: tauri::AppHandle,
    dirs: Vec<String>,
) -> Result<WorkspaceSettings, String> {
    let mut settings = load_settings(&app)?;
    settings.scan_ignore_dirs = dirs
        .into_iter()
        .map(|d| d.trim().to_string())
        .filter(|d| !d.is_empty())
        .collect();
    save_settings(&app, &settings)?;
    Ok(settings)
}

#[tauri::command]
fn complete_onboarding_cmd(app: tauri::AppHandle) -> Result<WorkspaceSettings, String> {
    let mut settings = load_settings(&app)?;
    settings.onboarding_done = true;
    save_settings(&app, &settings)?;
    Ok(settings)
}

#[tauri::command]
fn get_license_status_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
) -> Result<LicenseStatusRow, String> {
    let settings = load_settings(&app)?;
    let count = db::with_conn(&state, |conn| count_content_items(conn))?;
    let is_pro = license::is_pro(settings.license_key.as_deref());
    Ok(LicenseStatusRow {
        tier: license::tier_label(settings.license_key.as_deref()).into(),
        content_count: count,
        content_limit: license::FREE_CONTENT_LIMIT,
        is_pro,
    })
}

#[tauri::command]
fn activate_license_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    license_key: String,
) -> Result<LicenseStatusRow, String> {
    if !license::validate_activation_key(&license_key) {
        return Err("无效的激活码，格式应为 PKPRO-xxxxxxxx".into());
    }
    let mut settings = load_settings(&app)?;
    settings.license_key = Some(license_key.trim().to_string());
    save_settings(&app, &settings)?;
    get_license_status_cmd(app, state)
}

#[tauri::command]
fn scan_markdown(app: tauri::AppHandle, root: String) -> Result<Vec<MarkdownScanItem>, String> {
    let settings = load_settings(&app)?;
    let extra_ignore = settings.scan_ignore_dirs;
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
        if !path.is_file() || should_skip(path, &extra_ignore) {
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
    let previews = preview_splits(&path, strategy)?;
    Ok(expand_labeled_previews(previews))
}

fn expand_labeled_previews(previews: Vec<SplitPreview>) -> Vec<SplitPreview> {
    let mut expanded = Vec::new();
    for preview in previews {
        let pieces = content_fields::parse_block(&preview.title, &preview.body);
        let pair_key = if pieces.len() > 1 {
            Some(Uuid::new_v4().to_string())
        } else {
            None
        };
        for piece in pieces {
            let body = piece.body;
            let language = if piece.language.is_empty() {
                preview.language.clone()
            } else {
                piece.language
            };
            let clipped: String = body.chars().take(180).collect();
            let body_preview = if body.chars().count() > 180 {
                format!("{clipped}…")
            } else {
                clipped
            };
            expanded.push(SplitPreview {
                index: expanded.len(),
                title: piece.title,
                heading: preview.heading.clone(),
                start_line: preview.start_line,
                end_line: preview.end_line,
                body_preview,
                body,
                language,
                section_kind: preview.section_kind,
                recommended: preview.recommended,
                keywords: piece.keywords,
                pair_key: pair_key.clone(),
            });
        }
    }
    expanded
}

#[tauri::command]
fn import_md_splits(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    path: String,
    strategy: String,
    selected_indexes: Vec<usize>,
    replace_existing: bool,
) -> Result<ImportSplitsResult, String> {
    let strategy = SplitStrategy::parse(&strategy)?;
    let previews = expand_labeled_previews(preview_splits(&path, strategy)?);
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

    ensure_can_add_content(&app, &state, selected.len())?;

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

        let project_id = active_project_id(&app, conn)?;
        let mut content_ids = Vec::new();
        for preview in selected {
            let anchor = SourceAnchor {
                start_line: preview.start_line,
                end_line: preview.end_line,
                heading: preview.heading.clone(),
            };
            let fields_json = content_fields_json(&preview.title, &preview.body, &preview.keywords);
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
                Some(&project_id),
                preview.pair_key.as_deref(),
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
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    path: String,
    selected_indexes: Vec<usize>,
    replace_existing: bool,
) -> Result<ImportSplitsResult, String> {
    let rows = table_import::selected_table_rows(&path, &selected_indexes)?;
    if rows.is_empty() {
        return Err("请至少选择一行".into());
    }

    ensure_can_add_content(&app, &state, rows.len())?;

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

        let project_id = active_project_id(&app, conn)?;
        let mut content_ids = Vec::new();
        for row in rows {
            let anchor = SourceAnchor {
                start_line: row.index as u32,
                end_line: row.index as u32,
                heading: Some(format!("row:{}", row.index + 1)),
            };
            let fields_json = content_fields_json(&row.title, &row.body, &row.keywords);
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
                Some(&project_id),
                row.pair_key.as_deref(),
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
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    title: String,
    body: String,
    language: Option<String>,
    keywords: Option<Vec<String>>,
) -> Result<ContentItemRow, String> {
    let title = title.trim().to_string();
    let body = body.trim().to_string();
    if title.is_empty() || body.is_empty() {
        return Err("标题和正文不能为空".into());
    }
    let keywords = keywords
        .unwrap_or_default()
        .into_iter()
        .map(|word| word.trim().trim_start_matches('#').trim().to_string())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();

    ensure_can_add_content(&app, &state, 1)?;

    let language = language.unwrap_or_else(|| md_split::detect_language(&body));
    let item_id = Uuid::new_v4().to_string();
    let doc_id = Uuid::new_v4().to_string();
    let anchor = SourceAnchor {
        start_line: 0,
        end_line: 0,
        heading: Some("manual".into()),
    };
    let fields_json = content_fields_json(&title, &body, &keywords);

    db::with_conn(&state, |conn| {
        upsert_source_document(conn, &doc_id, MANUAL_SOURCE_PATH, "Manual entries")?;
        let existing_doc_id: String = conn
            .query_row(
                "SELECT id FROM source_documents WHERE path = ?1",
                [MANUAL_SOURCE_PATH],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;
        let project_id = active_project_id(&app, conn)?;
        let (project_name, project_color) = {
            let row = db::list_projects(conn, false)?
                .into_iter()
                .find(|item| item.id == project_id);
            (
                row.as_ref().map(|item| item.name.clone()).unwrap_or_default(),
                row.as_ref().map(|item| item.color.clone()).unwrap_or_else(|| "#c4a574".into()),
            )
        };

        insert_content_item(
            conn,
            &item_id,
            &existing_doc_id,
            MANUAL_SOURCE_PATH,
            &anchor_json(&anchor)?,
            &title,
            &language,
            &fields_json,
            Some(&project_id),
            None,
        )?;

        Ok(ContentItemRow {
            id: item_id.clone(),
            title,
            source_path: MANUAL_SOURCE_PATH.to_string(),
            language,
            body,
            created_at: chrono::Utc::now().to_rfc3339(),
            keywords,
            project_id,
            project_name,
            project_color,
            pair_id: String::new(),
        })
    })
}

fn fetch_publish_tasks(state: &DbState, status: Option<String>) -> Result<Vec<PublishTaskRow>, String> {
    db::with_conn(state, |conn| {
        let rows = list_publish_tasks(conn, status.as_deref())?;
        let mut tasks = map_task_rows(rows);
        attach_project_labels(conn, &mut tasks);
        Ok(tasks)
    })
}

pub(crate) fn fetch_today_ready_tasks(state: &DbState) -> Result<Vec<PublishTaskRow>, String> {
    let tasks = fetch_publish_tasks(state, Some("ready".into()))?;
    Ok(filter_today_queue_tasks(tasks))
}

#[tauri::command]
fn list_channels_cmd(state: tauri::State<'_, DbState>) -> Result<Vec<ChannelRow>, String> {
    db::with_conn(&state, |conn| {
        Ok(list_channels(conn)?
            .into_iter()
            .map(|(id, name, market, color, is_custom, keyword_hash)| ChannelRow {
                id,
                name,
                market,
                color,
                is_custom: is_custom != 0,
                keyword_hash: keyword_hash != 0,
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
            keyword_hash: false,
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
    fetch_today_ready_tasks(&state)
}

#[tauri::command]
fn update_publish_task_status_cmd(
    state: tauri::State<'_, DbState>,
    task_id: String,
    status: String,
    publish_url: Option<String>,
    note: Option<String>,
    blocked_reason: Option<String>,
    published_date: Option<String>,
) -> Result<PublishTaskRow, String> {
    db::with_conn(&state, |conn| {
        update_publish_task_status(
            conn,
            &task_id,
            &status,
            publish_url.as_deref(),
            note.as_deref(),
            blocked_reason.as_deref(),
            published_date.as_deref(),
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
fn update_task_note_cmd(
    state: tauri::State<'_, DbState>,
    task_id: String,
    note: String,
) -> Result<PublishTaskRow, String> {
    db::with_conn(&state, |conn| update_task_note(conn, &task_id, &note))?;
    fetch_publish_tasks(&state, None).and_then(|tasks| {
        tasks
            .into_iter()
            .find(|t| t.id == task_id)
            .ok_or_else(|| "任务不存在".into())
    })
}

#[tauri::command]
fn update_task_checklist_cmd(
    state: tauri::State<'_, DbState>,
    task_id: String,
    checklist: Vec<String>,
) -> Result<PublishTaskRow, String> {
    let raw = serde_json::to_string(&checklist).map_err(|e| e.to_string())?;
    db::with_conn(&state, |conn| update_task_checklist(conn, &task_id, &raw))?;
    fetch_publish_tasks(&state, None).and_then(|tasks| {
        tasks
            .into_iter()
            .find(|t| t.id == task_id)
            .ok_or_else(|| "任务不存在".into())
    })
}

#[tauri::command]
fn list_content_items_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    query: Option<String>,
) -> Result<Vec<ContentItemRow>, String> {
    db::with_conn(&state, |conn| {
        let project_id = active_project_id(&app, conn)?;
        let rows = if let Some(q) = query.as_ref().filter(|value| !value.trim().is_empty()) {
            search_content_items(conn, q, 200, Some(&project_id))?
        } else {
            list_content_items(conn, Some(&project_id))?
        };
        let mut items = Vec::new();
        for row in rows {
            items.push(ContentItemRow {
                id: row.id,
                title: row.title,
                source_path: row.source_path,
                language: row.language,
                body: fields_body(&row.fields_json),
                created_at: row.created_at,
                keywords: fields_keywords(&row.fields_json),
                project_id: row.project_id,
                project_name: row.project_name,
                project_color: row.project_color,
                pair_id: row.pair_id,
            });
        }
        Ok(items)
    })
}

#[tauri::command]
fn scan_media_cmd(app: tauri::AppHandle, state: tauri::State<'_, DbState>) -> Result<ScanMediaResult, String> {
    let settings = load_settings(&app)?;
    let extra_ignore = settings.scan_ignore_dirs;
    let root = settings
        .media_root
        .ok_or_else(|| "请先在「来源文件」选择图片文件夹".to_string())?;
    let roots: Vec<String> = std::iter::once(root)
        .chain(settings.video_root)
        .collect();
    let mut indexed = 0usize;
    db::with_conn(&state, |conn| {
        for root in roots {
            let root_path = PathBuf::from(&root);
            if !root_path.is_dir() {
                continue;
            }
            for entry in WalkDir::new(&root_path)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if !path.is_file() || should_skip(path, &extra_ignore) {
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
            let project_id = active_project_id(&app, conn)?;
            upsert_media_asset(
                conn,
                &id,
                &path_str,
                &file_name,
                kind,
                meta.len() as i64,
                mtime.as_deref(),
                &project_id,
            )?;
            indexed += 1;
            }
        }

        let project_id = active_project_id(&app, conn)?;
        let total: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM media_assets WHERE project_id = ?1",
                [&project_id],
                |row| row.get(0),
            )
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
        let project_id = active_project_id(&app, conn)?;
        Ok(list_media_assets(conn, Some(&project_id))?
            .iter()
            .map(|row| build_media_row(&app, row))
            .collect())
    })
}

#[tauri::command]
fn list_media_content_usages_cmd(
    state: tauri::State<'_, DbState>,
) -> Result<Vec<MediaContentUsageRow>, String> {
    db::with_conn(&state, |conn| {
        Ok(list_media_content_usages(conn)?
            .into_iter()
            .map(|(media_asset_id, content_id, content_title)| MediaContentUsageRow {
                media_asset_id,
                content_id,
                content_title,
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
            .iter()
            .map(|row| build_media_row(&app, row))
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
        let project_id = active_project_id(&app, conn)?;
        Ok(list_media_assets(conn, Some(&project_id))?
            .into_iter()
            .map(|row| (row.id, row.path, row.kind, row.thumb_status))
            .collect::<Vec<_>>())
    })?;

    let app_for_batch = app.clone();
    let (updates, remaining) = tauri::async_runtime::spawn_blocking(move || {
        media::generate_thumbnail_batch(&app_for_batch, &assets, batch_size)
    })
    .await
    .map_err(|e| e.to_string())?;
    let generated = updates.iter().filter(|item| item.status == "ready").count();
    db::with_conn(&state, |conn| {
        for update in &updates {
            save_media_thumb_state(
                conn,
                &update.id,
                update.width,
                update.height,
                &update.status,
                &update.error,
            )?;
        }
        Ok(())
    })?;
    Ok(ThumbBatchResult { generated, remaining })
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

        let project_id = active_project_id(&app, conn)?;
        let mut scored: Vec<MediaSuggestionRow> = list_media_assets(conn, Some(&project_id))?
            .into_iter()
            .filter(|row| !linked.contains(&row.id))
            .filter_map(|row| {
                let (score, reason) = media_suggest::score_media_for_content(
                    &source_path,
                    &title,
                    &row.path,
                    &row.file_name,
                    copy_root,
                    media_root,
                );
                if score <= 0 {
                    return None;
                }
                Some(MediaSuggestionRow {
                    id: row.id.clone(),
                    path: row.path,
                    file_name: row.file_name,
                    kind: row.kind.clone(),
                    size_bytes: row.size_bytes as u64,
                    thumb_path: media::thumb_path_if_exists(&app, &row.id, &row.kind),
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
            for asset in media {
                let target = unique_dest_path(&media_dir, &asset.file_name);
                fs::copy(&asset.path, &target)
                    .map_err(|e| format!("复制素材失败 {}: {e}", asset.path))?;
                media_count += 1;
            }
        }

        Ok(ExportContentPackResult {
            folder_path: pack_dir.to_string_lossy().to_string(),
            media_count,
        })
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportTaskPackResult {
    pub folder_path: String,
    pub media_count: usize,
    pub channel_id: String,
    pub file_count: usize,
}

#[tauri::command]
fn export_task_pack_cmd(
    state: tauri::State<'_, DbState>,
    task_id: String,
    dest_folder: String,
) -> Result<ExportTaskPackResult, String> {
    db::with_conn(&state, |conn| {
        let rows = list_publish_tasks(conn, None)?;
        let row = rows
            .into_iter()
            .find(|item| item.0 == task_id)
            .ok_or_else(|| "任务不存在".to_string())?;
        let channel_id = row.8.clone();
        let channel_name = row.9.clone();
        let content_item_id = row.11.clone();
        let title = row.12.clone();
        let body = fields_body(&row.14);
        let keywords = fields_keywords(&row.14);
        let media = list_media_for_content(conn, &content_item_id)?;

        let dest = PathBuf::from(&dest_folder);
        if !dest.is_dir() {
            return Err("目标路径不是文件夹".into());
        }

        let hash = channel_keyword_hash(conn, &channel_id)?;
        let pack = channel_pack::build_channel_pack(&channel_id, &channel_name, &title, &body, &keywords, hash);
        let base = format!(
            "{}-{}",
            sanitize_folder_name(&title),
            pack.folder_tag
        );
        let pack_dir = unique_pack_dir(&dest, &base);
        fs::create_dir_all(&pack_dir).map_err(|e| e.to_string())?;

        for file in &pack.files {
            fs::write(pack_dir.join(file.file_name), &file.content).map_err(|e| e.to_string())?;
        }
        fs::write(pack_dir.join("README.txt"), &pack.readme).map_err(|e| e.to_string())?;

        let mut media_count = 0;
        if !media.is_empty() {
            let media_dir = pack_dir.join("media");
            fs::create_dir_all(&media_dir).map_err(|e| e.to_string())?;
            for asset in media {
                let target = unique_dest_path(&media_dir, &asset.file_name);
                fs::copy(&asset.path, &target)
                    .map_err(|e| format!("复制素材失败 {}: {e}", asset.path))?;
                media_count += 1;
            }
        }

        Ok(ExportTaskPackResult {
            folder_path: pack_dir.to_string_lossy().to_string(),
            media_count,
            channel_id,
            file_count: pack.files.len(),
        })
    })
}

#[tauri::command]
fn copy_task_body_cmd(state: tauri::State<'_, DbState>, task_id: String) -> Result<(), String> {
    db::with_conn(&state, |conn| {
        let rows = list_publish_tasks(conn, None)?;
        let row = rows
            .into_iter()
            .find(|item| item.0 == task_id)
            .ok_or_else(|| "任务不存在".to_string())?;
        channel_pack::copy_task_body(&row.8, &row.12, &fields_body(&row.14))
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
            "id,status,channel,content_title,language,scheduled_at,published_at,publish_url,note,blocked_reason,updated_at\n",
        );

        for row in &rows {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                csv_cell(&row.0),
                csv_cell(&row.1),
                csv_cell(&row.9),
                csv_cell(&row.12),
                csv_cell(&row.13),
                csv_cell(&scheduled_date_input(&row.6)),
                csv_cell(&scheduled_date_input(&row.7)),
                csv_cell(&row.2),
                csv_cell(&row.3),
                csv_cell(&row.4),
                csv_cell(&row.5),
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
fn export_tasks_markdown_cmd(
    state: tauri::State<'_, DbState>,
    dest_path: String,
) -> Result<ExportTasksCsvResult, String> {
    db::with_conn(&state, |conn| {
        let rows = list_publish_tasks(conn, None)?;
        let mut md = String::from("# PublishKit Task Report\n\n");
        md.push_str(&format!(
            "Generated: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M UTC")
        ));
        md.push_str("| Status | Channel | Title | Language | Scheduled | Published | URL |\n");
        md.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &rows {
            md.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                row.1,
                row.9,
                row.12.replace('|', "\\|"),
                row.13,
                scheduled_date_input(&row.6),
                scheduled_date_input(&row.7),
                row.2.replace('|', "\\|"),
            ));
        }
        fs::write(&dest_path, md.as_bytes()).map_err(|e| e.to_string())?;
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
fn check_duplicate_publish_cmd(
    state: tauri::State<'_, DbState>,
    task_id: String,
    content_item_id: String,
    channel_id: String,
) -> Result<Option<DuplicatePublishWarning>, String> {
    db::with_conn(&state, |conn| {
        duplicate_publish_warning(conn, &content_item_id, &channel_id, &task_id, 30)
    })
}

#[tauri::command]
fn import_backup_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    zip_path: String,
    mode: Option<String>,
) -> Result<ImportBackupResult, String> {
    let mode = mode.unwrap_or_else(|| "replace".into());
    match mode.as_str() {
        "merge" => {
            let (merged, includes_thumbs) = db::with_conn(&state, |conn| {
                backup::merge_backup_from_zip(&app, conn, Path::new(&zip_path))
            })?;
            Ok(ImportBackupResult {
                mode,
                file_count: merged.content_items_added
                    + merged.publish_tasks_added
                    + merged.media_assets_added,
                includes_thumbs,
                merged: Some(merged),
            })
        }
        "replace" => {
            let summary = backup::stage_restore_from_zip(&app, Path::new(&zip_path))?;
            app.restart();
            #[allow(unreachable_code)]
            Ok(ImportBackupResult {
                mode,
                file_count: summary.file_count,
                includes_thumbs: summary.includes_thumbs,
                merged: None,
            })
        }
        _ => Err(format!("未知恢复模式: {mode}")),
    }
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
            .filter_map(|(id, status, event_at, channel_name, channel_color, content_title, publish_url, project_name)| {
                event_date(&event_at).map(|date| CalendarEntryRow {
                    id,
                    status,
                    date,
                    channel_name,
                    channel_color,
                    content_title,
                    publish_url,
                    project_name,
                })
            })
            .collect())
    })
}

#[tauri::command]
fn list_calendar_week_entries_cmd(
    state: tauri::State<'_, DbState>,
    anchor_date: String,
) -> Result<Vec<CalendarEntryRow>, String> {
    db::with_conn(&state, |conn| {
        Ok(list_calendar_week_tasks(conn, &anchor_date)?
            .into_iter()
            .filter_map(|(id, status, event_at, channel_name, channel_color, content_title, publish_url, project_name)| {
                event_date(&event_at).map(|date| CalendarEntryRow {
                    id,
                    status,
                    date,
                    channel_name,
                    channel_color,
                    content_title,
                    publish_url,
                    project_name,
                })
            })
            .collect())
    })
}

#[tauri::command]
fn list_projects_cmd(state: tauri::State<'_, DbState>) -> Result<Vec<db::ProjectRow>, String> {
    db::with_conn(&state, |conn| db::list_projects(conn, false))
}

#[tauri::command]
fn create_project_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    name: String,
) -> Result<WorkspaceSettings, String> {
    let project_id = db::with_conn(&state, |conn| {
        let project = db::create_project(conn, &name, None, None)?;
        Ok(project.id)
    })?;
    set_current_project_cmd(app, state, project_id)
}

#[tauri::command]
fn update_project_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    project_id: String,
    name: String,
) -> Result<WorkspaceSettings, String> {
    db::with_conn(&state, |conn| {
        let existing = db::list_projects(conn, true)?
            .into_iter()
            .find(|item| item.id == project_id)
            .ok_or_else(|| "项目不存在".to_string())?;
        let domestic = Some(existing.brand_domestic).filter(|value| !value.is_empty());
        let overseas = Some(existing.brand_overseas).filter(|value| !value.is_empty());
        db::update_project(
            conn,
            &project_id,
            &name,
            domestic.as_deref(),
            overseas.as_deref(),
        )
    })?;
    let settings = load_settings(&app)?;
    if settings.current_project_id.as_deref() == Some(project_id.as_str()) {
        return set_current_project_cmd(app, state, project_id);
    }
    Ok(settings)
}

#[tauri::command]
fn archive_project_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    project_id: String,
) -> Result<WorkspaceSettings, String> {
    let settings = load_settings(&app)?;
    db::with_conn(&state, |conn| db::archive_project(conn, &project_id, true))?;
    if settings.current_project_id.as_deref() == Some(project_id.as_str()) {
        let next = db::with_conn(&state, |conn| db::ensure_default_project(conn, "默认项目"))?;
        return set_current_project_cmd(app, state, next);
    }
    Ok(settings)
}

#[tauri::command]
fn set_current_project_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    project_id: String,
) -> Result<WorkspaceSettings, String> {
    let mut settings = load_settings(&app)?;
    let project = db::with_conn(&state, |conn| {
        db::list_projects(conn, false)?
            .into_iter()
            .find(|item| item.id == project_id)
            .ok_or_else(|| "项目不存在或已归档".to_string())
    })?;
    settings.current_project_id = Some(project.id);
    settings.project_name = Some(project.name);
    settings.brand_domestic = Some(project.brand_domestic).filter(|value| !value.is_empty());
    settings.brand_overseas = Some(project.brand_overseas).filter(|value| !value.is_empty());
    settings.copy_root = Some(project.copy_root).filter(|value| !value.is_empty());
    settings.media_root = Some(project.media_root).filter(|value| !value.is_empty());
    settings.video_root = Some(project.video_root).filter(|value| !value.is_empty());
    save_settings(&app, &settings)?;
    Ok(settings)
}

#[tauri::command]
fn move_content_to_project_cmd(
    state: tauri::State<'_, DbState>,
    content_ids: Vec<String>,
    project_id: String,
) -> Result<usize, String> {
    db::with_conn(&state, |conn| {
        db::move_content_to_project(conn, &content_ids, &project_id)
    })
}

#[tauri::command]
fn update_content_fields_cmd(
    state: tauri::State<'_, DbState>,
    content_item_id: String,
    title: String,
    body: String,
    keywords: Vec<String>,
    language: Option<String>,
) -> Result<(), String> {
    let keywords = keywords
        .into_iter()
        .map(|word| word.trim().trim_start_matches('#').trim().to_string())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();
    db::with_conn(&state, |conn| {
        db::update_content_fields(
            conn,
            &content_item_id,
            &title,
            &body,
            &keywords,
            language.as_deref(),
        )
    })
}

#[tauri::command]
fn recognize_content_keywords_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    content_item_id: String,
) -> Result<usize, String> {
    db::with_conn(&state, |conn| {
        let (title, source_path, fields_json) = get_content_item_by_id(conn, &content_item_id)?;
        let body = fields_body(&fields_json);
        let pieces = content_fields::parse_block(&title, &body);
        if pieces.is_empty() {
            return Ok(0);
        }
        let first = &pieces[0];
        let language = if first.language.is_empty() {
            None
        } else {
            Some(first.language.as_str())
        };
        db::update_content_fields(
            conn,
            &content_item_id,
            &first.title,
            &first.body,
            &first.keywords,
            language,
        )?;
        let mut created = 1usize;
        if pieces.len() > 1 {
            let pair_id = Uuid::new_v4().to_string();
            conn.execute(
                "UPDATE content_items SET pair_id = ?1 WHERE id = ?2 AND (pair_id IS NULL OR pair_id = '')",
                rusqlite::params![pair_id, content_item_id],
            )
            .map_err(|e| e.to_string())?;
            let project_id = active_project_id(&app, conn)?;
            let doc_id: String = conn
                .query_row(
                    "SELECT COALESCE(source_document_id, '') FROM content_items WHERE id = ?1",
                    [&content_item_id],
                    |row| row.get(0),
                )
                .unwrap_or_default();
            for piece in pieces.iter().skip(1) {
                let item_id = Uuid::new_v4().to_string();
                let fields = content_fields_json(&piece.title, &piece.body, &piece.keywords);
                let anchor = SourceAnchor {
                    start_line: 0,
                    end_line: 0,
                    heading: Some("recognized".into()),
                };
                insert_content_item(
                    conn,
                    &item_id,
                    &doc_id,
                    &source_path,
                    &anchor_json(&anchor)?,
                    &piece.title,
                    if piece.language.is_empty() { "en" } else { &piece.language },
                    &fields,
                    Some(&project_id),
                    Some(&pair_id),
                )?;
                created += 1;
            }
        }
        Ok(created)
    })
}

#[tauri::command]
fn copy_publish_field_cmd(
    state: tauri::State<'_, DbState>,
    field: String,
    title: String,
    body: String,
    keywords: Option<Vec<String>>,
    channel_id: Option<String>,
    hash: Option<bool>,
) -> Result<(), String> {
    let keywords = keywords.unwrap_or_default();
    let channel = channel_id.as_deref().map(str::trim).filter(|value| !value.is_empty());
    let hash = match hash {
        Some(value) => value,
        None => match channel {
            Some(id) => db::with_conn(&state, |conn| channel_keyword_hash(conn, id)).unwrap_or(false),
            None => false,
        },
    };
    match field.as_str() {
        "title" => rich_text::copy_plain_text(title.trim()),
        "keywords" => {
            let text = content_fields::format_keywords_with_hash(channel, &keywords, hash);
            if text.is_empty() {
                return Err("这条内容还没有关联词".into());
            }
            rich_text::copy_plain_text(&text)
        }
        "body" => {
            let text = content_fields::body_for_channel_copy_with_hash(channel, &body, &keywords, hash);
            let use_rich = channel
                .map(|id| !channel_pack::uses_plain_copy(id))
                .unwrap_or(true);
            if use_rich {
                rich_text::copy_markdown_rich_text(&text)
            } else {
                rich_text::copy_plain_text(&rich_text::markdown_to_plain(&text))
            }
        }
        "full" => {
            let text = content_fields::full_text_for_copy_with_hash(channel, &title, &body, &keywords, hash);
            if text.is_empty() {
                return Err("这条内容是空的".into());
            }
            rich_text::copy_plain_text(&text)
        }
        _ => Err("未知复制字段".into()),
    }
}

#[tauri::command]
fn open_media_file_cmd(app: tauri::AppHandle, path: String) -> Result<(), String> {
    app.opener()
        .open_path(path, None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn retry_failed_thumbnails_cmd(
    state: tauri::State<'_, DbState>,
    app: tauri::AppHandle,
) -> Result<usize, String> {
    db::with_conn(&state, |conn| {
        let project_id = active_project_id(&app, conn)?;
        reset_failed_thumbs(conn, Some(&project_id))
    })
}

fn bootstrap_workspace(app: &tauri::AppHandle) -> Result<(), String> {
    let state = app.state::<DbState>();
    let mut settings = load_settings(app)?;
    let preferred = settings.project_name.clone().unwrap_or_default();
    let snapshot = db::with_conn(&state, |conn| {
        db::rename_default_project_if_untouched(conn, &preferred)?;
        let project_id = active_project_id(app, conn)?;
        for (kind, path) in [
            ("copy", settings.copy_root.clone()),
            ("media", settings.media_root.clone()),
            ("video", settings.video_root.clone()),
        ] {
            if db::project_root(conn, &project_id, kind)?.is_none() {
                if let Some(path) = path.as_deref() {
                    let _ = db::upsert_project_root(conn, &project_id, kind, path, false);
                }
            }
        }
        db::list_projects(conn, false)?
            .into_iter()
            .find(|item| item.id == project_id)
            .ok_or_else(|| "项目不存在".to_string())
    })?;
    settings.current_project_id = Some(snapshot.id);
    settings.project_name = Some(snapshot.name);
    settings.brand_domestic = Some(snapshot.brand_domestic).filter(|value| !value.is_empty());
    settings.brand_overseas = Some(snapshot.brand_overseas).filter(|value| !value.is_empty());
    settings.copy_root = Some(snapshot.copy_root).filter(|value| !value.is_empty());
    settings.media_root = Some(snapshot.media_root).filter(|value| !value.is_empty());
    settings.video_root = Some(snapshot.video_root).filter(|value| !value.is_empty());
    save_settings(app, &settings)?;
    Ok(())
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
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BatchCommitItem {
    source_path: String,
    title: String,
    body: String,
    keywords: Vec<String>,
    language: String,
    pair_key: Option<String>,
    scheduled_date: String,
    channel_ids: Vec<String>,
    #[serde(default)]
    media_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct BatchCommitResult {
    content_count: usize,
    task_count: usize,
    skipped_tasks: usize,
}

#[tauri::command]
fn preview_batch_import_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    paths: Vec<String>,
) -> Result<batch_import::BatchImportPreview, String> {
    if paths.is_empty() {
        return Err("请选择文件".into());
    }
    let mut preview = batch_import::preview_paths(&paths);
    let (assets, media_root) = db::with_conn(&state, |conn| {
        let project_id = active_project_id(&app, conn)?;
        let assets = list_media_assets(conn, Some(&project_id))?;
        let media_root = db::project_root(conn, &project_id, "media")?;
        Ok((assets, media_root))
    })?;
    let candidates = assets
        .into_iter()
        .filter(|row| row.kind == "image")
        .map(|row| batch_import::ImageCandidate {
            id: row.id,
            file_name: row.file_name,
            path: row.path,
            kind: row.kind,
        })
        .collect::<Vec<_>>();
    batch_import::attach_images(&mut preview.items, &candidates, media_root.as_deref());
    for item in &mut preview.items {
        for image in &mut item.images {
            image.thumb_path = media::thumb_path_if_exists(&app, &image.id, "image");
        }
    }
    Ok(preview)
}

#[tauri::command]
fn commit_batch_import_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, DbState>,
    items: Vec<BatchCommitItem>,
) -> Result<BatchCommitResult, String> {
    let items: Vec<BatchCommitItem> = items
        .into_iter()
        .filter(|item| !item.channel_ids.is_empty() && !item.scheduled_date.trim().is_empty())
        .collect();
    if items.is_empty() {
        return Err("没有可创建的任务。请选择平台，并填写发布日期。".into());
    }
    for item in &items {
        let _ = parse_scheduled_date(&item.scheduled_date)?;
        if item.title.trim().is_empty() && item.body.trim().is_empty() {
            return Err("有一条内容没有标题，也没有正文".into());
        }
    }
    ensure_can_add_content(&app, &state, items.len())?;

    db::with_conn(&state, |conn| {
        let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
        let project_id = active_project_id(&app, &tx)?;
        let mut docs: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        let mut content_count = 0usize;
        let mut task_count = 0usize;
        let mut skipped_tasks = 0usize;
        let now = chrono::Utc::now().to_rfc3339();

        for item in &items {
            let doc_id = if let Some(id) = docs.get(&item.source_path) {
                id.clone()
            } else {
                let new_id = Uuid::new_v4().to_string();
                let doc_title = Path::new(&item.source_path)
                    .file_stem()
                    .and_then(|value| value.to_str())
                    .unwrap_or("导入")
                    .to_string();
                upsert_source_document(&tx, &new_id, &item.source_path, &doc_title)?;
                let stored: String = tx
                    .query_row(
                        "SELECT id FROM source_documents WHERE path = ?1",
                        [&item.source_path],
                        |row| row.get(0),
                    )
                    .map_err(|e| e.to_string())?;
                docs.insert(item.source_path.clone(), stored.clone());
                stored
            };
            let anchor = anchor_json(&SourceAnchor {
                start_line: 1,
                end_line: 1,
                heading: None,
            })?;
            let keywords: Vec<String> = item
                .keywords
                .iter()
                .map(|word| word.trim().trim_start_matches('#').trim().to_string())
                .filter(|word| !word.is_empty())
                .collect();
            let fields = content_fields_json(&item.title, &item.body, &keywords);
            let item_id = Uuid::new_v4().to_string();
            let language = if item.language.trim().is_empty() {
                "zh"
            } else {
                item.language.trim()
            };
            insert_content_item(
                &tx,
                &item_id,
                &doc_id,
                &item.source_path,
                &anchor,
                &item.title,
                language,
                &fields,
                Some(&project_id),
                item.pair_key.as_deref(),
            )?;
            content_count += 1;
            let scheduled = parse_scheduled_date(&item.scheduled_date)?;
            for channel_id in &item.channel_ids {
                let task_id = Uuid::new_v4().to_string();
                match tx.execute(
                    "INSERT INTO publish_tasks (
                        id, content_item_id, channel_id, status, scheduled_at, created_at, updated_at
                     ) VALUES (?1, ?2, ?3, 'ready', ?4, ?5, ?6)",
                    rusqlite::params![task_id, item_id, channel_id, scheduled, now, now],
                ) {
                    Ok(_) => task_count += 1,
                    Err(error) if error.to_string().contains("UNIQUE") => skipped_tasks += 1,
                    Err(error) => return Err(error.to_string()),
                }
            }
            for media_id in &item.media_ids {
                let media_id = media_id.trim();
                if media_id.is_empty() {
                    continue;
                }
                if db::project_image_asset(&tx, media_id, &project_id)? {
                    link_content_media(&tx, &item_id, media_id)?;
                }
            }
        }
        tx.commit().map_err(|e| e.to_string())?;
        Ok(BatchCommitResult {
            content_count,
            task_count,
            skipped_tasks,
        })
    })
}

#[tauri::command]
fn export_calendar_ledger_cmd(
    state: tauri::State<'_, DbState>,
    dest_path: String,
    start_date: String,
    end_date: String,
    project_ids: Vec<String>,
    statuses: Vec<String>,
    labels: db::LedgerLabels,
) -> Result<u32, String> {
    let dest_path = dest_path.trim();
    if dest_path.is_empty() {
        return Err("请选择保存位置".into());
    }
    let rows = db::with_conn(&state, |conn| {
        db::list_ledger_rows(conn, &start_date, &end_date, &project_ids, &statuses)
    })?;
    let table = db::ledger_table(&rows, &labels);
    let bytes = import_template::xlsx_bytes(&labels.sheet, &table)?;
    fs::write(dest_path, bytes).map_err(|e| e.to_string())?;
    Ok(rows.len() as u32)
}

#[tauri::command]
fn list_ledger_projects_cmd(state: tauri::State<'_, DbState>) -> Result<Vec<db::ProjectRow>, String> {
    db::with_conn(&state, |conn| db::list_projects(conn, true))
}

#[tauri::command]
fn save_import_template_cmd(kind: String, dest_path: String) -> Result<(), String> {
    let dest_path = dest_path.trim();
    if dest_path.is_empty() {
        return Err("请选择保存位置".into());
    }
    match kind.as_str() {
        "markdown" => {
            std::fs::write(dest_path, import_template::article_template_markdown()).map_err(|e| e.to_string())?;
        }
        "sheet" => {
            std::fs::write(dest_path, import_template::schedule_template_xlsx()?).map_err(|e| e.to_string())?;
        }
        _ => return Err("未知模板".into()),
    }
    Ok(())
}

#[tauri::command]
fn channel_keyword_hash_cmd(state: tauri::State<'_, DbState>, channel_id: String) -> Result<bool, String> {
    db::with_conn(&state, |conn| channel_keyword_hash(conn, &channel_id))
}

#[tauri::command]
fn set_channel_keyword_hash_cmd(
    state: tauri::State<'_, DbState>,
    channel_id: String,
    enabled: bool,
) -> Result<(), String> {
    db::with_conn(&state, |conn| set_channel_keyword_hash(conn, &channel_id, enabled))
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .on_page_load(|webview, payload| {
            use tauri::webview::PageLoadEvent;
            if payload.event() == PageLoadEvent::Finished {
                let window = webview.window();
                let _ = window.show();
                let _ = window.set_focus();
            }
        })
        .setup(|app| {
            backup::apply_pending_restore_if_any(app.handle())?;
            db::init(app.handle())?;
            bootstrap_workspace(app.handle())?;
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
            set_video_root,
            set_workspace_profile_cmd,
            set_scan_ignore_dirs_cmd,
            complete_onboarding_cmd,
            get_license_status_cmd,
            activate_license_cmd,
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
            update_task_note_cmd,
            update_task_checklist_cmd,
            scan_media_cmd,
            list_media_assets_cmd,
            list_media_content_usages_cmd,
            list_content_media_cmd,
            link_content_media_cmd,
            unlink_content_media_cmd,
            list_calendar_entries_cmd,
            list_calendar_week_entries_cmd,
            generate_media_thumbnails_cmd,
            copy_media_image_cmd,
            copy_markdown_rich_text_cmd,
            copy_task_body_cmd,
            reveal_media_in_folder_cmd,
            suggest_media_for_content_cmd,
            update_publish_task_scheduled_cmd,
            export_content_pack_cmd,
            export_task_pack_cmd,
            export_tasks_csv_cmd,
            export_tasks_markdown_cmd,
            stage_content_images_cmd,
            delete_content_item_cmd,
            delete_content_items_cmd,
            export_backup_cmd,
            check_duplicate_publish_cmd,
            import_backup_cmd,
            get_api_status_cmd,
            regenerate_pairing_token_cmd,
            list_projects_cmd,
            create_project_cmd,
            update_project_cmd,
            archive_project_cmd,
            set_current_project_cmd,
            move_content_to_project_cmd,
            update_content_fields_cmd,
            recognize_content_keywords_cmd,
            copy_publish_field_cmd,
            open_media_file_cmd,
            retry_failed_thumbnails_cmd,
            preview_batch_import_cmd,
            commit_batch_import_cmd,
            save_import_template_cmd,
            channel_keyword_hash_cmd,
            set_channel_keyword_hash_cmd,
            list_ledger_projects_cmd,
            export_calendar_ledger_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
