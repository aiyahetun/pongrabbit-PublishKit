mod db;
mod md_split;

use db::{
    create_custom_channel, create_publish_task, delete_content_items_for_source,
    delete_custom_channel, fields_body, insert_content_item, list_channels, list_content_items,
    list_publish_tasks, update_publish_task_status, upsert_source_document, DbState,
};
use md_split::{anchor_json, preview_splits, SourceAnchor, SplitPreview, SplitStrategy};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tauri::Manager;
use uuid::Uuid;
use walkdir::WalkDir;

const IGNORE_DIRS: &[&str] = &["node_modules", ".git", ".cursor", "target", "dist"];
const MANUAL_SOURCE_PATH: &str = "manual://publishkit";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSettings {
    pub ui_locale: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_root: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_root: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownScanItem {
    pub path: String,
    pub title: String,
    pub size_bytes: u64,
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
    pub channel: TaskChannelRef,
    pub content: TaskContentRef,
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
    )>,
) -> Vec<PublishTaskRow> {
    rows.into_iter()
        .map(|row| PublishTaskRow {
            id: row.0.clone(),
            status: row.1.clone(),
            publish_url: row.2.clone(),
            note: row.3.clone(),
            updated_at: row.4.clone(),
            channel: TaskChannelRef {
                id: row.5.clone(),
                name: row.6.clone(),
                color: row.7.clone(),
            },
            content: TaskContentRef {
                id: row.8.clone(),
                title: row.9.clone(),
                language: row.10.clone(),
                body: fields_body(&row.11),
            },
        })
        .collect()
}

fn settings_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("settings.json"))
}

fn load_settings(app: &tauri::AppHandle) -> Result<WorkspaceSettings, String> {
    let path = settings_path(app)?;
    if !path.exists() {
        return Ok(WorkspaceSettings {
            ui_locale: detect_ui_locale(),
            copy_root: None,
            media_root: None,
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
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if ext != "md" && ext != "markdown" && ext != "txt" {
            continue;
        }
        let meta = fs::metadata(path).map_err(|e| e.to_string())?;
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        items.push(MarkdownScanItem {
            path: path.to_string_lossy().into_owned(),
            title: title_from_markdown(path, &content),
            size_bytes: meta.len(),
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
    let doc_title = fs::read_to_string(&path)
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            db::init(app.handle())?;
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
            create_manual_content,
            list_content_items_cmd,
            list_channels_cmd,
            create_channel_cmd,
            delete_channel_cmd,
            create_publish_task_cmd,
            list_publish_tasks_cmd,
            list_today_tasks_cmd,
            update_publish_task_status_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
