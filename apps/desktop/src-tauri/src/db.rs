use rusqlite::{Connection, OptionalExtension};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::{AppHandle, Manager};

const MIGRATION_001: &str = include_str!("../migrations/001_init.sql");
const MIGRATION_002: &str = include_str!("../migrations/002_tasks.sql");
const MIGRATION_003: &str = include_str!("../migrations/003_channels.sql");
const MIGRATION_004: &str = include_str!("../migrations/004_media.sql");
const MIGRATION_005: &str = include_str!("../migrations/005_fts.sql");
const MIGRATION_006: &str = include_str!("../migrations/006_task_blocked_reason.sql");
const MIGRATION_007: &str = include_str!("../migrations/007_task_checklist.sql");
const MIGRATION_008: &str = include_str!("../migrations/008_projects.sql");

const PROJECT_COLORS: &[&str] = &[
    "#c4a574", "#d4654a", "#3d7a6a", "#4a6fa5", "#8a6aa8", "#b08968",
];

const DEFAULT_CHANNELS: &[(&str, &str, &str, &str, i32)] = &[
    // 国内
    ("xhs", "小红书", "domestic", "#ff2442", 1),
    ("douyin", "抖音", "domestic", "#111111", 2),
    ("channels", "视频号", "domestic", "#fa9d3b", 3),
    ("kuaishou", "快手", "domestic", "#ff4906", 4),
    ("zhihu", "知乎", "domestic", "#0084ff", 5),
    ("weibo", "微博", "domestic", "#e6162d", 6),
    ("bilibili", "哔哩哔哩", "domestic", "#00a1d6", 7),
    ("wechat_mp", "微信公众号", "domestic", "#07c160", 8),
    ("toutiao", "今日头条", "domestic", "#ff0000", 9),
    // 海外
    ("instagram", "Instagram", "overseas", "#e1306c", 20),
    ("tiktok", "TikTok", "overseas", "#010101", 21),
    ("youtube", "YouTube", "overseas", "#ff0000", 22),
    ("pinterest", "Pinterest", "overseas", "#e60023", 23),
    ("reddit", "Reddit", "overseas", "#ff4500", 24),
    ("facebook", "Facebook", "overseas", "#1877f2", 25),
    ("twitter", "X (Twitter)", "overseas", "#000000", 26),
    ("linkedin", "LinkedIn", "overseas", "#0a66c2", 27),
    ("threads", "Threads", "overseas", "#000000", 28),
    ("producthunt", "Product Hunt", "overseas", "#da552f", 29),
];

pub struct DbState {
    pub conn: Mutex<Connection>,
}

pub fn init(app: &AppHandle) -> Result<(), String> {
    let db_path = db_path(app)?;
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    migrate(&conn)?;
    app.manage(DbState {
        conn: Mutex::new(conn),
    });
    Ok(())
}

fn db_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(dir.join("publishkit.db"))
}

pub fn db_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    db_path(app)
}

fn migration_applied(conn: &Connection, version: i64) -> Result<bool, String> {
    let applied: Option<i64> = conn
        .query_row(
            "SELECT version FROM schema_migrations WHERE version = ?1",
            [version],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    Ok(applied.is_some())
}

fn mark_migration(conn: &Connection, version: i64) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
        rusqlite::params![version, now],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn seed_channels(conn: &Connection) -> Result<(), String> {
    for (id, name, market, color, sort_order) in DEFAULT_CHANNELS {
        conn.execute(
            "INSERT OR IGNORE INTO channels (id, name, market, color, sort_order)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![id, name, market, color, sort_order],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn channels_has_is_custom(conn: &Connection) -> Result<bool, String> {
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('channels') WHERE name = 'is_custom'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    Ok(count > 0)
}

fn migrate(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(MIGRATION_001)
        .map_err(|e| e.to_string())?;
    if !migration_applied(conn, 1)? {
        mark_migration(conn, 1)?;
    }

    conn.execute_batch(MIGRATION_002)
        .map_err(|e| e.to_string())?;
    if !migration_applied(conn, 2)? {
        seed_channels(conn)?;
        mark_migration(conn, 2)?;
    }

    if !migration_applied(conn, 3)? {
        if !channels_has_is_custom(conn)? {
            conn.execute_batch(MIGRATION_003)
                .map_err(|e| e.to_string())?;
        }
        mark_migration(conn, 3)?;
    }
    seed_channels(conn)?;

    if !migration_applied(conn, 4)? {
        conn.execute_batch(MIGRATION_004)
            .map_err(|e| e.to_string())?;
        mark_migration(conn, 4)?;
    }

    if !migration_applied(conn, 5)? {
        conn.execute_batch(MIGRATION_005)
            .map_err(|e| e.to_string())?;
        rebuild_content_fts(conn)?;
        mark_migration(conn, 5)?;
    }

    if !migration_applied(conn, 6)? {
        conn.execute_batch(MIGRATION_006)
            .map_err(|e| e.to_string())?;
        if !publish_tasks_has_blocked_reason(conn)? {
            conn.execute("ALTER TABLE publish_tasks ADD COLUMN blocked_reason TEXT", [])
                .map_err(|e| e.to_string())?;
        }
        mark_migration(conn, 6)?;
    }

    if !migration_applied(conn, 7)? {
        conn.execute_batch(MIGRATION_007)
            .map_err(|e| e.to_string())?;
        if !publish_tasks_has_checklist_json(conn)? {
            conn.execute("ALTER TABLE publish_tasks ADD COLUMN checklist_json TEXT", [])
                .map_err(|e| e.to_string())?;
        }
        mark_migration(conn, 7)?;
    }

    if !migration_applied(conn, 8)? {
        conn.execute_batch(MIGRATION_008)
            .map_err(|e| e.to_string())?;
        add_column_if_missing(conn, "content_items", "project_id", "TEXT")?;
        add_column_if_missing(conn, "content_items", "pair_id", "TEXT")?;
        add_column_if_missing(conn, "media_assets", "project_id", "TEXT")?;
        add_column_if_missing(conn, "media_assets", "width", "INTEGER")?;
        add_column_if_missing(conn, "media_assets", "height", "INTEGER")?;
        add_column_if_missing(conn, "media_assets", "thumb_status", "TEXT")?;
        add_column_if_missing(conn, "media_assets", "thumb_error", "TEXT")?;
        rebuild_media_assets_unique(conn)?;
        mark_migration(conn, 8)?;
    }

    if !migration_applied(conn, 9)? {
        add_column_if_missing(conn, "channels", "keyword_hash", "INTEGER NOT NULL DEFAULT 0")?;
        for channel_id in [
            "xhs", "douyin", "kuaishou", "channels", "bilibili", "instagram", "tiktok", "threads", "weibo",
            "toutiao",
        ] {
            conn.execute(
                "UPDATE channels SET keyword_hash = 1 WHERE id = ?1",
                [channel_id],
            )
            .map_err(|e| e.to_string())?;
        }
        mark_migration(conn, 9)?;
    }

    let _ = ensure_default_project(conn, "默认项目")?;
    assign_orphan_rows(conn)?;

    Ok(())
}

fn column_exists(conn: &Connection, table: &str, column: &str) -> Result<bool, String> {
    let sql = format!("SELECT COUNT(*) FROM pragma_table_info('{table}') WHERE name = ?1");
    let count: i64 = conn
        .query_row(&sql, [column], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    Ok(count > 0)
}

fn add_column_if_missing(
    conn: &Connection,
    table: &str,
    column: &str,
    decl: &str,
) -> Result<(), String> {
    if column_exists(conn, table, column)? {
        return Ok(());
    }
    conn.execute(&format!("ALTER TABLE {table} ADD COLUMN {column} {decl}"), [])
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn media_has_project_path_unique(conn: &Connection) -> Result<bool, String> {
    let sql: String = conn
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type = 'table' AND name = 'media_assets'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    let compact: String = sql.chars().filter(|ch| !ch.is_whitespace()).collect();
    Ok(compact.to_ascii_lowercase().contains("unique(project_id,path)"))
}

fn rebuild_media_assets_unique(conn: &Connection) -> Result<(), String> {
    if media_has_project_path_unique(conn)? {
        return Ok(());
    }
    conn.execute_batch(
        "PRAGMA foreign_keys = OFF;
         CREATE TABLE media_assets_v8 (
           id TEXT PRIMARY KEY,
           path TEXT NOT NULL,
           file_name TEXT NOT NULL,
           kind TEXT NOT NULL DEFAULT 'image',
           size_bytes INTEGER NOT NULL DEFAULT 0,
           mtime TEXT,
           indexed_at TEXT NOT NULL,
           project_id TEXT,
           width INTEGER,
           height INTEGER,
           thumb_status TEXT,
           thumb_error TEXT,
           UNIQUE(project_id, path)
         );
         INSERT INTO media_assets_v8 (
           id, path, file_name, kind, size_bytes, mtime, indexed_at,
           project_id, width, height, thumb_status, thumb_error
         )
         SELECT
           id, path, file_name, kind, size_bytes, mtime, indexed_at,
           project_id, width, height, thumb_status, thumb_error
         FROM media_assets;
         DROP TABLE media_assets;
         ALTER TABLE media_assets_v8 RENAME TO media_assets;
         CREATE INDEX IF NOT EXISTS idx_media_assets_file_name ON media_assets(file_name);
         CREATE INDEX IF NOT EXISTS idx_media_assets_project ON media_assets(project_id);
         PRAGMA foreign_keys = ON;",
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn publish_tasks_has_checklist_json(conn: &Connection) -> Result<bool, String> {
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('publish_tasks') WHERE name = 'checklist_json'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    Ok(count > 0)
}

fn publish_tasks_has_blocked_reason(conn: &Connection) -> Result<bool, String> {
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('publish_tasks') WHERE name = 'blocked_reason'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    Ok(count > 0)
}

pub fn with_conn<T, F>(state: &DbState, f: F) -> Result<T, String>
where
    F: FnOnce(&Connection) -> Result<T, String>,
{
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    f(&conn)
}

pub fn upsert_source_document(
    conn: &Connection,
    id: &str,
    path: &str,
    title: &str,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO source_documents (id, path, title, hash, last_scanned_at)
         VALUES (?1, ?2, ?3, NULL, ?4)
         ON CONFLICT(path) DO UPDATE SET
           title = excluded.title,
           last_scanned_at = excluded.last_scanned_at",
        rusqlite::params![id, path, title, now],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn delete_content_items_for_source(conn: &Connection, source_path: &str) -> Result<(), String> {
    conn.execute(
        "DELETE FROM content_items_fts
         WHERE content_item_id IN (SELECT id FROM content_items WHERE source_path = ?1)",
        [source_path],
    )
    .map_err(|e| e.to_string())?;
    conn.execute(
        "DELETE FROM content_items WHERE source_path = ?1",
        [source_path],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct ContentListRow {
    pub id: String,
    pub title: String,
    pub source_path: String,
    pub language: String,
    pub fields_json: String,
    pub created_at: String,
    pub project_id: String,
    pub project_name: String,
    pub project_color: String,
    pub pair_id: String,
}

pub fn insert_content_item(
    conn: &Connection,
    id: &str,
    source_document_id: &str,
    source_path: &str,
    source_anchor: &str,
    title: &str,
    language: &str,
    fields_json: &str,
    project_id: Option<&str>,
    pair_id: Option<&str>,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    let project_id = match project_id.map(str::trim).filter(|value| !value.is_empty()) {
        Some(id) => id.to_string(),
        None => ensure_default_project(conn, "默认项目")?,
    };
    let pair_id = pair_id.map(str::trim).filter(|value| !value.is_empty());
    // Empty string is not a source document. Foreign keys are on after migration 8.
    let source_document_id = source_document_id.trim();
    let source_document_id = if source_document_id.is_empty() {
        None
    } else {
        Some(source_document_id)
    };
    conn.execute(
        "INSERT INTO content_items (
            id, source_document_id, source_path, source_anchor, title,
            language, fields_json, created_at, updated_at, project_id, pair_id
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        rusqlite::params![
            id,
            source_document_id,
            source_path,
            source_anchor,
            title,
            language,
            fields_json,
            now,
            now,
            project_id,
            pair_id,
        ],
    )
    .map_err(|e| e.to_string())?;
    sync_content_fts(conn, id, title, &fields_body(fields_json))?;
    Ok(())
}

fn map_content_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ContentListRow> {
    Ok(ContentListRow {
        id: row.get(0)?,
        title: row.get(1)?,
        source_path: row.get(2)?,
        language: row.get(3)?,
        fields_json: row.get(4)?,
        created_at: row.get(5)?,
        project_id: row.get::<_, Option<String>>(6)?.unwrap_or_default(),
        project_name: row.get::<_, Option<String>>(7)?.unwrap_or_default(),
        project_color: row.get::<_, Option<String>>(8)?.unwrap_or_else(|| "#c4a574".into()),
        pair_id: row.get::<_, Option<String>>(9)?.unwrap_or_default(),
    })
}

const CONTENT_LIST_SQL: &str = "SELECT ci.id, ci.title, ci.source_path, ci.language, ci.fields_json, ci.created_at,
        COALESCE(ci.project_id, ''), COALESCE(p.name, ''), COALESCE(p.color, '#c4a574'), COALESCE(ci.pair_id, '')
     FROM content_items ci
     LEFT JOIN projects p ON p.id = ci.project_id";

pub fn list_content_items(
    conn: &Connection,
    project_id: Option<&str>,
) -> Result<Vec<ContentListRow>, String> {
    let (sql, use_filter) = if project_id.map(str::trim).filter(|value| !value.is_empty()).is_some() {
        (
            format!("{CONTENT_LIST_SQL} WHERE ci.project_id = ?1 ORDER BY ci.created_at DESC"),
            true,
        )
    } else {
        (format!("{CONTENT_LIST_SQL} ORDER BY ci.created_at DESC"), false)
    };
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let rows = if use_filter {
        stmt.query_map([project_id.unwrap_or_default()], map_content_row)
    } else {
        stmt.query_map([], map_content_row)
    }
    .map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    Ok(rows)
}

fn build_fts_query(raw: &str) -> Option<String> {
    let terms: Vec<String> = raw
        .split_whitespace()
        .map(str::trim)
        .filter(|term| !term.is_empty())
        .map(|term| {
            let escaped = term.replace('"', "\"\"");
            format!("\"{escaped}\"*")
        })
        .collect();
    if terms.is_empty() {
        None
    } else {
        Some(terms.join(" AND "))
    }
}

pub fn sync_content_fts(
    conn: &Connection,
    content_item_id: &str,
    title: &str,
    body: &str,
) -> Result<(), String> {
    conn.execute(
        "DELETE FROM content_items_fts WHERE content_item_id = ?1",
        [content_item_id],
    )
    .map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO content_items_fts (title, body, content_item_id) VALUES (?1, ?2, ?3)",
        rusqlite::params![title, body, content_item_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn remove_content_fts_for_ids(conn: &Connection, ids: &[String]) -> Result<(), String> {
    for id in ids {
        conn.execute(
            "DELETE FROM content_items_fts WHERE content_item_id = ?1",
            [id],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn rebuild_content_fts(conn: &Connection) -> Result<(), String> {
    conn.execute("DELETE FROM content_items_fts", [])
        .map_err(|e| e.to_string())?;
    // Read the base table directly. list_content_items joins projects, which
    // does not exist until migration 8, and this rebuild runs in migration 5.
    let mut stmt = conn
        .prepare("SELECT id, title, fields_json FROM content_items")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    drop(stmt);
    for (id, title, fields_json) in rows {
        sync_content_fts(conn, &id, &title, &fields_body(&fields_json))?;
    }
    Ok(())
}

fn search_content_items_fts(
    conn: &Connection,
    fts_query: &str,
    limit: i64,
    project_id: Option<&str>,
) -> Result<Vec<ContentListRow>, String> {
    let filter = project_id.map(str::trim).filter(|value| !value.is_empty());
    let sql = if filter.is_some() {
        format!(
            "{CONTENT_LIST_SQL}
             JOIN content_items_fts fts ON ci.id = fts.content_item_id
             WHERE fts MATCH ?1 AND ci.project_id = ?2
             ORDER BY bm25(fts)
             LIMIT ?3"
        )
    } else {
        format!(
            "{CONTENT_LIST_SQL}
             JOIN content_items_fts fts ON ci.id = fts.content_item_id
             WHERE fts MATCH ?1
             ORDER BY bm25(fts)
             LIMIT ?2"
        )
    };
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let rows = if let Some(project) = filter {
        stmt.query_map(rusqlite::params![fts_query, project, limit], map_content_row)
    } else {
        stmt.query_map(rusqlite::params![fts_query, limit], map_content_row)
    }
    .map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    Ok(rows)
}

fn search_content_items_like(
    conn: &Connection,
    query: &str,
    limit: i64,
    project_id: Option<&str>,
) -> Result<Vec<ContentListRow>, String> {
    let pattern = format!("%{query}%");
    let filter = project_id.map(str::trim).filter(|value| !value.is_empty());
    let sql = if filter.is_some() {
        format!(
            "{CONTENT_LIST_SQL}
             WHERE (ci.title LIKE ?1 OR ci.fields_json LIKE ?1) AND ci.project_id = ?2
             ORDER BY ci.created_at DESC
             LIMIT ?3"
        )
    } else {
        format!(
            "{CONTENT_LIST_SQL}
             WHERE ci.title LIKE ?1 OR ci.fields_json LIKE ?1
             ORDER BY ci.created_at DESC
             LIMIT ?2"
        )
    };
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let rows = if let Some(project) = filter {
        stmt.query_map(rusqlite::params![pattern, project, limit], map_content_row)
    } else {
        stmt.query_map(rusqlite::params![pattern, limit], map_content_row)
    }
    .map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    Ok(rows)
}

pub fn search_content_items(
    conn: &Connection,
    query: &str,
    limit: i64,
    project_id: Option<&str>,
) -> Result<Vec<ContentListRow>, String> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return list_content_items(conn, project_id);
    }
    if let Some(fts_query) = build_fts_query(trimmed) {
        match search_content_items_fts(conn, &fts_query, limit, project_id) {
            Ok(rows) => return Ok(rows),
            Err(_) => {}
        }
    }
    search_content_items_like(conn, trimmed, limit, project_id)
}

pub fn delete_content_items(conn: &Connection, ids: &[String]) -> Result<usize, String> {
    if ids.is_empty() {
        return Ok(0);
    }
    remove_content_fts_for_ids(conn, ids)?;
    let placeholders = (1..=ids.len())
        .map(|index| format!("?{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!("DELETE FROM content_items WHERE id IN ({placeholders})");
    conn.execute(&sql, rusqlite::params_from_iter(ids.iter()))
        .map_err(|e| e.to_string())?;
    Ok(conn.changes() as usize)
}

pub fn list_channels(conn: &Connection) -> Result<Vec<(String, String, String, String, i64, i64)>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, name, market, COALESCE(color, '#888888'), is_custom, COALESCE(keyword_hash, 0)
             FROM channels
             ORDER BY is_custom ASC, sort_order ASC, name ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, i64>(5)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

pub fn channel_keyword_hash(conn: &Connection, channel_id: &str) -> Result<bool, String> {
    let value: Option<i64> = conn
        .query_row(
            "SELECT COALESCE(keyword_hash, 0) FROM channels WHERE id = ?1",
            [channel_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    Ok(match value {
        Some(flag) => flag != 0,
        None => crate::content_fields::default_keyword_hash(Some(channel_id)),
    })
}

pub fn set_channel_keyword_hash(conn: &Connection, channel_id: &str, enabled: bool) -> Result<(), String> {
    let updated = conn
        .execute(
            "UPDATE channels SET keyword_hash = ?1 WHERE id = ?2",
            rusqlite::params![if enabled { 1 } else { 0 }, channel_id],
        )
        .map_err(|e| e.to_string())?;
    if updated == 0 {
        return Err("渠道不存在".into());
    }
    Ok(())
}

pub fn create_custom_channel(
    conn: &Connection,
    id: &str,
    name: &str,
    market: &str,
    color: &str,
) -> Result<(), String> {
    let sort: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(sort_order), 999) + 1 FROM channels WHERE is_custom = 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(1000);

    conn.execute(
        "INSERT INTO channels (id, name, market, color, sort_order, is_custom)
         VALUES (?1, ?2, ?3, ?4, ?5, 1)",
        rusqlite::params![id, name, market, color, sort],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn delete_custom_channel(conn: &Connection, channel_id: &str) -> Result<(), String> {
    let is_custom: i64 = conn
        .query_row(
            "SELECT is_custom FROM channels WHERE id = ?1",
            [channel_id],
            |row| row.get(0),
        )
        .map_err(|_| "渠道不存在".to_string())?;

    if is_custom == 0 {
        return Err("内置渠道不可删除".into());
    }

    let in_use: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM publish_tasks WHERE channel_id = ?1",
            [channel_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    if in_use > 0 {
        return Err("该渠道下仍有任务，无法删除".into());
    }

    conn.execute("DELETE FROM channels WHERE id = ?1 AND is_custom = 1", [channel_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn create_publish_task(
    conn: &Connection,
    id: &str,
    content_item_id: &str,
    channel_id: &str,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO publish_tasks (
            id, content_item_id, channel_id, status, created_at, updated_at
         ) VALUES (?1, ?2, ?3, 'draft', ?4, ?5)",
        rusqlite::params![id, content_item_id, channel_id, now, now],
    )
    .map_err(|e| {
        if e.to_string().contains("UNIQUE") {
            "该内容与渠道的任务已存在".into()
        } else {
            e.to_string()
        }
    })?;
    Ok(())
}

pub fn count_content_items(conn: &Connection) -> Result<usize, String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM content_items", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    Ok(count as usize)
}

pub fn update_task_checklist(
    conn: &Connection,
    task_id: &str,
    checklist_json: &str,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE publish_tasks SET checklist_json = ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![checklist_json, now, task_id],
    )
    .map_err(|e| e.to_string())?;
    if conn.changes() == 0 {
        return Err("任务不存在".into());
    }
    Ok(())
}

pub fn update_publish_task_status(
    conn: &Connection,
    task_id: &str,
    status: &str,
    publish_url: Option<&str>,
    note: Option<&str>,
    blocked_reason: Option<&str>,
    published_at: Option<&str>,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    let default_scheduled = format!(
        "{}T12:00:00Z",
        chrono::Local::now().format("%Y-%m-%d")
    );
    let published_timestamp = if status == "published" {
        Some(resolve_task_timestamp(published_at, &now)?)
    } else {
        None
    };

    conn.execute(
        "UPDATE publish_tasks SET
            status = ?1,
            publish_url = COALESCE(?2, publish_url),
            note = COALESCE(?3, note),
            scheduled_at = CASE
                WHEN ?1 = 'ready' AND scheduled_at IS NULL THEN ?7
                ELSE scheduled_at
            END,
            published_at = CASE
                WHEN ?1 = 'published' THEN COALESCE(?8, ?4)
                ELSE published_at
            END,
            blocked_reason = CASE
                WHEN ?1 = 'blocked' THEN COALESCE(?6, blocked_reason)
                ELSE NULL
            END,
            updated_at = ?4
         WHERE id = ?5",
        rusqlite::params![
            status,
            publish_url,
            note,
            now,
            task_id,
            blocked_reason,
            default_scheduled,
            published_timestamp
        ],
    )
    .map_err(|e| e.to_string())?;

    if conn.changes() == 0 {
        return Err("任务不存在".into());
    }
    Ok(())
}

fn resolve_task_timestamp(value: Option<&str>, fallback: &str) -> Result<String, String> {
    let Some(raw) = value.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(fallback.to_string());
    };
    if raw.len() == 10 && raw.chars().all(|c| c.is_ascii_digit() || c == '-') {
        return Ok(format!("{raw}T12:00:00Z"));
    }
    chrono::DateTime::parse_from_rfc3339(raw)
        .map(|dt| dt.to_rfc3339())
        .map_err(|_| "日期格式无效".into())
}

pub fn update_task_note(conn: &Connection, task_id: &str, note: &str) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE publish_tasks SET note = ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![note, now, task_id],
    )
    .map_err(|e| e.to_string())?;
    if conn.changes() == 0 {
        return Err("任务不存在".into());
    }
    Ok(())
}

pub fn get_content_item_by_id(
    conn: &Connection,
    content_item_id: &str,
) -> Result<(String, String, String), String> {
    conn.query_row(
        "SELECT title, source_path, fields_json FROM content_items WHERE id = ?1",
        [content_item_id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )
    .map_err(|_| "内容条目不存在".into())
}

pub fn linked_media_ids(conn: &Connection, content_item_id: &str) -> Result<Vec<String>, String> {
    let mut stmt = conn
        .prepare("SELECT media_asset_id FROM content_media WHERE content_item_id = ?1")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([content_item_id], |row| row.get::<_, String>(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

pub fn update_task_scheduled_at(
    conn: &Connection,
    task_id: &str,
    scheduled_at: Option<&str>,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE publish_tasks SET scheduled_at = ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![scheduled_at, now, task_id],
    )
    .map_err(|e| e.to_string())?;
    if conn.changes() == 0 {
        return Err("任务不存在".into());
    }
    Ok(())
}

pub fn list_publish_tasks(
    conn: &Connection,
    status_filter: Option<&str>,
) -> Result<Vec<(String, String, String, String, String, String, String, String, String, String, String, String, String, String, String, String)>, String> {
    let sql = if status_filter.is_some() {
        "SELECT
            t.id, t.status, t.publish_url, t.note, COALESCE(t.blocked_reason, ''), t.updated_at,
            COALESCE(t.scheduled_at, ''), COALESCE(t.published_at, ''),
            c.id, c.name, COALESCE(c.color, '#888888'),
            i.id, i.title, i.language, i.fields_json, COALESCE(t.checklist_json, '')
         FROM publish_tasks t
         JOIN channels c ON c.id = t.channel_id
         JOIN content_items i ON i.id = t.content_item_id
         WHERE t.status = ?1
         ORDER BY t.updated_at DESC"
    } else {
        "SELECT
            t.id, t.status, t.publish_url, t.note, COALESCE(t.blocked_reason, ''), t.updated_at,
            COALESCE(t.scheduled_at, ''), COALESCE(t.published_at, ''),
            c.id, c.name, COALESCE(c.color, '#888888'),
            i.id, i.title, i.language, i.fields_json, COALESCE(t.checklist_json, '')
         FROM publish_tasks t
         JOIN channels c ON c.id = t.channel_id
         JOIN content_items i ON i.id = t.content_item_id
         ORDER BY t.updated_at DESC"
    };

    let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;

    let map_row = |row: &rusqlite::Row<'_>| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<String>>(2)?.unwrap_or_default(),
            row.get::<_, Option<String>>(3)?.unwrap_or_default(),
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
            row.get::<_, String>(7)?,
            row.get::<_, String>(8)?,
            row.get::<_, String>(9)?,
            row.get::<_, String>(10)?,
            row.get::<_, String>(11)?,
            row.get::<_, String>(12)?,
            row.get::<_, String>(13)?,
            row.get::<_, String>(14)?,
            row.get::<_, String>(15)?,
        ))
    };

    let rows = if let Some(status) = status_filter {
        stmt.query_map([status], map_row)
    } else {
        stmt.query_map([], map_row)
    }
    .map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;

    Ok(rows)
}

pub fn title_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .to_string()
}

pub fn fields_body(fields_json: &str) -> String {
    serde_json::from_str::<serde_json::Value>(fields_json)
        .ok()
        .and_then(|v| v.get("body").and_then(|b| b.as_str()).map(str::to_string))
        .unwrap_or_default()
}

pub fn fields_keywords(fields_json: &str) -> Vec<String> {
    serde_json::from_str::<serde_json::Value>(fields_json)
        .ok()
        .and_then(|value| value.get("keywords").cloned())
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or_default()
}

pub fn content_fields_json(title: &str, body: &str, keywords: &[String]) -> String {
    serde_json::json!({
        "title": title,
        "body": body,
        "keywords": keywords,
    })
    .to_string()
}

pub fn upsert_media_asset(
    conn: &Connection,
    id: &str,
    path: &str,
    file_name: &str,
    kind: &str,
    size_bytes: i64,
    mtime: Option<&str>,
    project_id: &str,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO media_assets (
            id, path, file_name, kind, size_bytes, mtime, indexed_at, project_id, thumb_status
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'pending')
         ON CONFLICT(project_id, path) DO UPDATE SET
           file_name = excluded.file_name,
           kind = excluded.kind,
           size_bytes = excluded.size_bytes,
           mtime = excluded.mtime,
           indexed_at = excluded.indexed_at",
        rusqlite::params![id, path, file_name, kind, size_bytes, mtime, now, project_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct MediaListRow {
    pub id: String,
    pub path: String,
    pub file_name: String,
    pub kind: String,
    pub size_bytes: i64,
    pub indexed_at: String,
    pub project_id: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub thumb_status: String,
    pub thumb_error: String,
}

fn map_media_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<MediaListRow> {
    Ok(MediaListRow {
        id: row.get(0)?,
        path: row.get(1)?,
        file_name: row.get(2)?,
        kind: row.get(3)?,
        size_bytes: row.get(4)?,
        indexed_at: row.get(5)?,
        project_id: row.get::<_, Option<String>>(6)?.unwrap_or_default(),
        width: row.get(7)?,
        height: row.get(8)?,
        thumb_status: row.get::<_, Option<String>>(9)?.unwrap_or_default(),
        thumb_error: row.get::<_, Option<String>>(10)?.unwrap_or_default(),
    })
}

const MEDIA_LIST_SQL: &str = "SELECT id, path, file_name, kind, size_bytes, indexed_at,
        COALESCE(project_id, ''), width, height, COALESCE(thumb_status, ''), COALESCE(thumb_error, '')
     FROM media_assets";

pub fn media_asset_id_by_path(conn: &Connection, path: &str) -> Result<String, String> {
    conn.query_row(
        "SELECT id FROM media_assets WHERE path = ?1",
        [path],
        |row| row.get(0),
    )
    .map_err(|_| "素材不存在".into())
}

pub fn list_media_assets(
    conn: &Connection,
    project_id: Option<&str>,
) -> Result<Vec<MediaListRow>, String> {
    let filter = project_id.map(str::trim).filter(|value| !value.is_empty());
    let sql = if filter.is_some() {
        format!("{MEDIA_LIST_SQL} WHERE project_id = ?1 ORDER BY file_name ASC")
    } else {
        format!("{MEDIA_LIST_SQL} ORDER BY file_name ASC")
    };
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let rows = if let Some(project) = filter {
        stmt.query_map([project], map_media_row)
    } else {
        stmt.query_map([], map_media_row)
    }
    .map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    Ok(rows)
}

pub fn list_media_for_content(
    conn: &Connection,
    content_item_id: &str,
) -> Result<Vec<MediaListRow>, String> {
    let sql = format!(
        "SELECT m.id, m.path, m.file_name, m.kind, m.size_bytes, m.indexed_at,
                COALESCE(m.project_id, ''), m.width, m.height,
                COALESCE(m.thumb_status, ''), COALESCE(m.thumb_error, '')
         FROM content_media cm
         JOIN media_assets m ON m.id = cm.media_asset_id
         WHERE cm.content_item_id = ?1
         ORDER BY cm.sort_order ASC, m.file_name ASC"
    );
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([content_item_id], map_media_row)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

pub fn save_media_thumb_state(
    conn: &Connection,
    id: &str,
    width: Option<i64>,
    height: Option<i64>,
    status: &str,
    error: &str,
) -> Result<(), String> {
    conn.execute(
        "UPDATE media_assets
         SET width = COALESCE(?1, width),
             height = COALESCE(?2, height),
             thumb_status = ?3,
             thumb_error = ?4
         WHERE id = ?5",
        rusqlite::params![width, height, status, error, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn reset_failed_thumbs(conn: &Connection, project_id: Option<&str>) -> Result<usize, String> {
    let changed = if let Some(project) = project_id.map(str::trim).filter(|value| !value.is_empty()) {
        conn.execute(
            "UPDATE media_assets SET thumb_status = 'pending', thumb_error = ''
             WHERE thumb_status = 'failed' AND project_id = ?1",
            [project],
        )
    } else {
        conn.execute(
            "UPDATE media_assets SET thumb_status = 'pending', thumb_error = ''
             WHERE thumb_status = 'failed'",
            [],
        )
    }
    .map_err(|e| e.to_string())?;
    Ok(changed)
}

pub fn list_media_content_usages(
    conn: &Connection,
) -> Result<Vec<(String, String, String)>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT cm.media_asset_id, ci.id, ci.title
             FROM content_media cm
             JOIN content_items ci ON ci.id = cm.content_item_id
             ORDER BY ci.title ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

pub fn project_image_asset(conn: &Connection, media_id: &str, project_id: &str) -> Result<bool, String> {
    let found = conn
        .query_row(
            "SELECT 1 FROM media_assets WHERE id = ?1 AND project_id = ?2 AND kind = 'image'",
            rusqlite::params![media_id, project_id],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    Ok(found.is_some())
}

pub fn link_content_media(
    conn: &Connection,
    content_item_id: &str,
    media_asset_id: &str,
) -> Result<(), String> {
    let sort: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(sort_order), -1) + 1 FROM content_media WHERE content_item_id = ?1",
            [content_item_id],
            |row| row.get(0),
        )
        .unwrap_or(0);

    conn.execute(
        "INSERT OR IGNORE INTO content_media (content_item_id, media_asset_id, sort_order)
         VALUES (?1, ?2, ?3)",
        rusqlite::params![content_item_id, media_asset_id, sort],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn unlink_content_media(
    conn: &Connection,
    content_item_id: &str,
    media_asset_id: &str,
) -> Result<(), String> {
    conn.execute(
        "DELETE FROM content_media WHERE content_item_id = ?1 AND media_asset_id = ?2",
        rusqlite::params![content_item_id, media_asset_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn list_calendar_tasks(
    conn: &Connection,
    year: i32,
    month: u32,
) -> Result<Vec<(String, String, String, String, String, String, String, String)>, String> {
    let start = format!("{:04}-{:02}-01T00:00:00", year, month);
    let next_month = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let end = format!("{:04}-{:02}-01T00:00:00", next_month.0, next_month.1);

    let mut stmt = conn
        .prepare(
            "SELECT
                t.id, t.status,
                COALESCE(t.published_at, t.scheduled_at, t.updated_at) AS event_at,
                c.name, COALESCE(c.color, '#888888'), i.title, t.publish_url,
                COALESCE(p.name, '')
             FROM publish_tasks t
             JOIN channels c ON c.id = t.channel_id
             JOIN content_items i ON i.id = t.content_item_id
             LEFT JOIN projects p ON p.id = i.project_id
             WHERE t.status IN ('published', 'scheduled', 'ready')
               AND COALESCE(t.published_at, t.scheduled_at, t.updated_at) >= ?1
               AND COALESCE(t.published_at, t.scheduled_at, t.updated_at) < ?2
             ORDER BY event_at ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![start, end], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, Option<String>>(6)?.unwrap_or_default(),
                row.get::<_, String>(7)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

pub fn list_calendar_week_tasks(
    conn: &Connection,
    anchor_date: &str,
) -> Result<Vec<(String, String, String, String, String, String, String, String)>, String> {
    use chrono::{Datelike, Duration, NaiveDate};

    let date = NaiveDate::parse_from_str(anchor_date, "%Y-%m-%d")
        .map_err(|_| "日期格式无效".to_string())?;
    let weekday = date.weekday().num_days_from_monday();
    let week_start = date - Duration::days(weekday as i64);
    let week_end = week_start + Duration::days(7);
    let start = format!("{}T00:00:00", week_start);
    let end = format!("{}T00:00:00", week_end);

    let mut stmt = conn
        .prepare(
            "SELECT
                t.id, t.status,
                COALESCE(t.published_at, t.scheduled_at, t.updated_at) AS event_at,
                c.name, COALESCE(c.color, '#888888'), i.title, t.publish_url,
                COALESCE(p.name, '')
             FROM publish_tasks t
             JOIN channels c ON c.id = t.channel_id
             JOIN content_items i ON i.id = t.content_item_id
             LEFT JOIN projects p ON p.id = i.project_id
             WHERE t.status IN ('published', 'scheduled', 'ready', 'blocked')
               AND COALESCE(t.published_at, t.scheduled_at, t.updated_at) >= ?1
               AND COALESCE(t.published_at, t.scheduled_at, t.updated_at) < ?2
             ORDER BY event_at ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![start, end], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, Option<String>>(6)?.unwrap_or_default(),
                row.get::<_, String>(7)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

const LEDGER_STATUSES: &[&str] = &["draft", "ready", "scheduled", "published", "archived", "blocked"];
const LEDGER_CELL_LIMIT: usize = 32000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerRow {
    pub date: String,
    pub project_id: String,
    pub project_name: String,
    pub channel_name: String,
    pub status: String,
    pub language: String,
    pub title: String,
    pub body: String,
    pub publish_url: String,
    pub note: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LedgerLabels {
    pub sheet: String,
    pub date: String,
    pub project: String,
    pub channel: String,
    pub status: String,
    pub language: String,
    pub title: String,
    pub body: String,
    pub url: String,
    pub note: String,
    pub status_draft: String,
    pub status_ready: String,
    pub status_scheduled: String,
    pub status_published: String,
    pub status_archived: String,
    pub status_blocked: String,
    pub lang_zh: String,
    pub lang_en: String,
    pub lang_bilingual: String,
}

pub fn list_ledger_rows(
    conn: &Connection,
    start_date: &str,
    end_date: &str,
    project_ids: &[String],
    statuses: &[String],
) -> Result<Vec<LedgerRow>, String> {
    let start = parse_ledger_date(start_date)?;
    let end = parse_ledger_date(end_date)?;
    if start > end {
        return Err("起始日要早于或等于结束日".into());
    }
    let statuses: Vec<String> = statuses
        .iter()
        .map(|status| status.trim().to_string())
        .filter(|status| LEDGER_STATUSES.contains(&status.as_str()))
        .collect();
    if statuses.is_empty() {
        return Err("请至少选一种状态".into());
    }
    let project_ids: Vec<String> = project_ids
        .iter()
        .map(|id| id.trim().to_string())
        .filter(|id| !id.is_empty())
        .collect();

    let mut sql = String::from(
        "SELECT
            COALESCE(t.published_at, t.scheduled_at, t.updated_at),
            COALESCE(i.project_id, ''),
            COALESCE(p.name, ''),
            c.name,
            t.status,
            COALESCE(i.language, ''),
            i.title,
            i.fields_json,
            COALESCE(t.publish_url, ''),
            COALESCE(t.note, '')
         FROM publish_tasks t
         JOIN channels c ON c.id = t.channel_id
         JOIN content_items i ON i.id = t.content_item_id
         LEFT JOIN projects p ON p.id = i.project_id
         WHERE t.status IN (",
    );
    let mut params: Vec<String> = Vec::new();
    for (index, status) in statuses.iter().enumerate() {
        if index > 0 {
            sql.push(',');
        }
        sql.push('?');
        params.push(status.clone());
    }
    sql.push(')');
    if !project_ids.is_empty() {
        sql.push_str(" AND i.project_id IN (");
        for (index, id) in project_ids.iter().enumerate() {
            if index > 0 {
                sql.push(',');
            }
            sql.push('?');
            params.push(id.clone());
        }
        sql.push(')');
    }

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let queried = stmt
        .query_map(rusqlite::params_from_iter(params.iter()), |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, String>(8)?,
                row.get::<_, String>(9)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let start_key = start.format("%Y-%m-%d").to_string();
    let end_key = end.format("%Y-%m-%d").to_string();
    let mut rows = Vec::new();
    for (event_at, project_id, project_name, channel_name, status, language, title, fields_json, publish_url, note) in
        queried
    {
        let Some(date) = ledger_event_date(&event_at) else {
            continue;
        };
        if date < start_key || date > end_key {
            continue;
        }
        rows.push(LedgerRow {
            date,
            project_id,
            project_name,
            channel_name,
            status,
            language,
            title,
            body: fields_body(&fields_json),
            publish_url,
            note,
        });
    }
    rows.sort_by(|left, right| {
        left.date
            .cmp(&right.date)
            .then(left.project_name.cmp(&right.project_name))
            .then(left.channel_name.cmp(&right.channel_name))
            .then(left.title.cmp(&right.title))
    });
    Ok(rows)
}

pub fn ledger_table(rows: &[LedgerRow], labels: &LedgerLabels) -> Vec<Vec<String>> {
    let mut table = vec![vec![
        labels.date.clone(),
        labels.project.clone(),
        labels.channel.clone(),
        labels.status.clone(),
        labels.language.clone(),
        labels.title.clone(),
        labels.body.clone(),
        labels.url.clone(),
        labels.note.clone(),
    ]];
    for row in rows {
        table.push(vec![
            row.date.clone(),
            row.project_name.clone(),
            row.channel_name.clone(),
            ledger_status_label(&row.status, labels),
            ledger_language_label(&row.language, labels),
            row.title.clone(),
            clip_ledger_cell(&row.body),
            row.publish_url.clone(),
            clip_ledger_cell(&row.note),
        ]);
    }
    table
}

fn parse_ledger_date(raw: &str) -> Result<chrono::NaiveDate, String> {
    chrono::NaiveDate::parse_from_str(raw.trim(), "%Y-%m-%d").map_err(|_| "日期格式无效".to_string())
}

fn ledger_event_date(iso: &str) -> Option<String> {
    chrono::DateTime::parse_from_rfc3339(iso.trim())
        .ok()
        .map(|dt| dt.date_naive().format("%Y-%m-%d").to_string())
}

fn ledger_status_label(status: &str, labels: &LedgerLabels) -> String {
    match status {
        "draft" => labels.status_draft.clone(),
        "ready" => labels.status_ready.clone(),
        "scheduled" => labels.status_scheduled.clone(),
        "published" => labels.status_published.clone(),
        "archived" => labels.status_archived.clone(),
        "blocked" => labels.status_blocked.clone(),
        other => other.to_string(),
    }
}

fn ledger_language_label(language: &str, labels: &LedgerLabels) -> String {
    match language.trim().to_ascii_lowercase().as_str() {
        "" | "zh" | "cn" | "zh-cn" => labels.lang_zh.clone(),
        "en" | "en-us" => labels.lang_en.clone(),
        "bilingual" => labels.lang_bilingual.clone(),
        other => other.to_string(),
    }
}

fn clip_ledger_cell(value: &str) -> String {
    if value.chars().count() <= LEDGER_CELL_LIMIT {
        return value.to_string();
    }
    let mut clipped: String = value.chars().take(LEDGER_CELL_LIMIT - 1).collect();
    clipped.push('…');
    clipped
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DuplicatePublishWarning {
    pub previous_published_at: String,
    pub previous_publish_url: String,
    pub days_since: i64,
}

pub fn duplicate_publish_warning(
    conn: &Connection,
    content_item_id: &str,
    channel_id: &str,
    _task_id: &str,
    within_days: i64,
) -> Result<Option<DuplicatePublishWarning>, String> {
    let cutoff = chrono::Utc::now() - chrono::Duration::days(within_days);
    // content_item_id + channel_id is unique — the only publish row is the current task.
    // Look at published_at history, not other task ids.
    let row: Option<(String, String)> = conn
        .query_row(
            "SELECT published_at, COALESCE(publish_url, '')
             FROM publish_tasks
             WHERE content_item_id = ?1 AND channel_id = ?2
               AND published_at IS NOT NULL AND published_at != ''
             ORDER BY published_at DESC LIMIT 1",
            rusqlite::params![content_item_id, channel_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|e| e.to_string())?;

    let Some((published_at, publish_url)) = row else {
        return Ok(None);
    };
    let Some(dt) = chrono::DateTime::parse_from_rfc3339(&published_at)
        .ok()
        .map(|value| value.with_timezone(&chrono::Utc))
    else {
        return Ok(None);
    };
    if dt < cutoff {
        return Ok(None);
    }
    let days_since = (chrono::Utc::now() - dt).num_days().max(0);
    Ok(Some(DuplicatePublishWarning {
        previous_published_at: published_at,
        previous_publish_url: publish_url,
        days_since,
    }))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRow {
    pub id: String,
    pub name: String,
    pub color: String,
    pub brand_domestic: String,
    pub brand_overseas: String,
    pub archived_at: String,
    pub content_count: i64,
    pub media_count: i64,
    pub copy_root: String,
    pub media_root: String,
    pub video_root: String,
}

pub fn ensure_default_project(conn: &Connection, preferred_name: &str) -> Result<String, String> {
    let existing: Option<String> = conn
        .query_row(
            "SELECT id FROM projects WHERE archived_at IS NULL ORDER BY created_at ASC LIMIT 1",
            [],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    if let Some(id) = existing {
        return Ok(id);
    }
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let name = {
        let trimmed = preferred_name.trim();
        if trimmed.is_empty() {
            "默认项目"
        } else {
            trimmed
        }
    };
    conn.execute(
        "INSERT INTO projects (id, name, color, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4)",
        rusqlite::params![id, name, PROJECT_COLORS[0], now],
    )
    .map_err(|e| e.to_string())?;
    Ok(id)
}

pub fn assign_orphan_rows(conn: &Connection) -> Result<(), String> {
    let project_id = ensure_default_project(conn, "默认项目")?;
    conn.execute(
        "UPDATE content_items SET project_id = ?1 WHERE project_id IS NULL OR project_id = ''",
        [&project_id],
    )
    .map_err(|e| e.to_string())?;

    let dupes: Vec<(String, String)> = {
        let mut stmt = conn
            .prepare(
                "SELECT id, path FROM media_assets
                 WHERE (project_id IS NULL OR project_id = '')
                   AND path IN (SELECT path FROM media_assets WHERE project_id = ?1)",
            )
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([&project_id], |row| Ok((row.get(0)?, row.get(1)?)))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        rows
    };
    for (old_id, path) in dupes {
        let keeper: String = conn
            .query_row(
                "SELECT id FROM media_assets WHERE project_id = ?1 AND path = ?2 LIMIT 1",
                rusqlite::params![project_id, path],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;
        conn.execute(
            "UPDATE OR IGNORE content_media SET media_asset_id = ?1 WHERE media_asset_id = ?2",
            rusqlite::params![keeper, old_id],
        )
        .map_err(|e| e.to_string())?;
        conn.execute(
            "DELETE FROM content_media WHERE media_asset_id = ?1",
            [&old_id],
        )
        .map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM media_assets WHERE id = ?1", [&old_id])
            .map_err(|e| e.to_string())?;
    }

    conn.execute(
        "UPDATE media_assets SET project_id = ?1 WHERE project_id IS NULL OR project_id = ''",
        [&project_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn list_projects(conn: &Connection, include_archived: bool) -> Result<Vec<ProjectRow>, String> {
    let sql = if include_archived {
        "SELECT id, name, color, COALESCE(brand_domestic, ''), COALESCE(brand_overseas, ''), COALESCE(archived_at, '')
         FROM projects ORDER BY created_at ASC"
    } else {
        "SELECT id, name, color, COALESCE(brand_domestic, ''), COALESCE(brand_overseas, ''), COALESCE(archived_at, '')
         FROM projects WHERE archived_at IS NULL ORDER BY created_at ASC"
    };
    let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;
    let base = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let mut rows = Vec::new();
    for (id, name, color, brand_domestic, brand_overseas, archived_at) in base {
        let content_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM content_items WHERE project_id = ?1",
                [&id],
                |row| row.get(0),
            )
            .unwrap_or(0);
        let media_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM media_assets WHERE project_id = ?1",
                [&id],
                |row| row.get(0),
            )
            .unwrap_or(0);
        let copy_root = project_root(conn, &id, "copy")?.unwrap_or_default();
        let media_root = project_root(conn, &id, "media")?.unwrap_or_default();
        let video_root = project_root(conn, &id, "video")?.unwrap_or_default();
        rows.push(ProjectRow {
            id,
            name,
            color,
            brand_domestic,
            brand_overseas,
            archived_at,
            content_count,
            media_count,
            copy_root,
            media_root,
            video_root,
        });
    }
    Ok(rows)
}

pub fn project_root(conn: &Connection, project_id: &str, kind: &str) -> Result<Option<String>, String> {
    conn.query_row(
        "SELECT path FROM project_roots WHERE project_id = ?1 AND kind = ?2",
        rusqlite::params![project_id, kind],
        |row| row.get(0),
    )
    .optional()
    .map_err(|e| e.to_string())
}

pub fn create_project(
    conn: &Connection,
    name: &str,
    brand_domestic: Option<&str>,
    brand_overseas: Option<&str>,
) -> Result<ProjectRow, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("项目名称不能为空".into());
    }
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    let color = PROJECT_COLORS[(count as usize) % PROJECT_COLORS.len()];
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO projects (id, name, color, brand_domestic, brand_overseas, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)",
        rusqlite::params![id, name, color, brand_domestic, brand_overseas, now],
    )
    .map_err(|e| e.to_string())?;
    Ok(list_projects(conn, true)?
        .into_iter()
        .find(|row| row.id == id)
        .ok_or_else(|| "项目创建失败".to_string())?)
}

pub fn update_project(
    conn: &Connection,
    id: &str,
    name: &str,
    brand_domestic: Option<&str>,
    brand_overseas: Option<&str>,
) -> Result<(), String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("项目名称不能为空".into());
    }
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE projects
         SET name = ?1, brand_domestic = ?2, brand_overseas = ?3, updated_at = ?4
         WHERE id = ?5",
        rusqlite::params![name, brand_domestic, brand_overseas, now, id],
    )
    .map_err(|e| e.to_string())?;
    if conn.changes() == 0 {
        return Err("项目不存在".into());
    }
    Ok(())
}

pub fn archive_project(conn: &Connection, id: &str, archived: bool) -> Result<(), String> {
    let active: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM projects WHERE archived_at IS NULL",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    if archived && active <= 1 {
        return Err("至少保留一个项目".into());
    }
    let now = chrono::Utc::now().to_rfc3339();
    let archived_at: Option<String> = if archived { Some(now.clone()) } else { None };
    conn.execute(
        "UPDATE projects SET archived_at = ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![archived_at, now, id],
    )
    .map_err(|e| e.to_string())?;
    if conn.changes() == 0 {
        return Err("项目不存在".into());
    }
    Ok(())
}

pub fn upsert_project_root(
    conn: &Connection,
    project_id: &str,
    kind: &str,
    path: &str,
    force: bool,
) -> Result<(), String> {
    let path = path.trim();
    if path.is_empty() {
        conn.execute(
            "DELETE FROM project_roots WHERE project_id = ?1 AND kind = ?2",
            rusqlite::params![project_id, kind],
        )
        .map_err(|e| e.to_string())?;
        return Ok(());
    }
    let other: Option<(String, String)> = conn
        .query_row(
            "SELECT r.project_id, p.name
             FROM project_roots r
             JOIN projects p ON p.id = r.project_id
             WHERE r.kind = ?1 AND r.path = ?2 AND r.project_id != ?3",
            rusqlite::params![kind, path, project_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    if let Some((_, other_name)) = &other {
        if !force {
            return Err(format!("ROOT_BOUND:{other_name}"));
        }
        conn.execute(
            "DELETE FROM project_roots WHERE kind = ?1 AND path = ?2 AND project_id != ?3",
            rusqlite::params![kind, path, project_id],
        )
        .map_err(|e| e.to_string())?;
    }
    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO project_roots (id, project_id, path, kind) VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(project_id, kind) DO UPDATE SET path = excluded.path",
        rusqlite::params![id, project_id, path, kind],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn project_label_for_content(
    conn: &Connection,
    content_id: &str,
) -> Result<(String, String, String), String> {
    conn.query_row(
        "SELECT COALESCE(ci.project_id, ''), COALESCE(p.name, ''), COALESCE(p.color, '#c4a574')
         FROM content_items ci
         LEFT JOIN projects p ON p.id = ci.project_id
         WHERE ci.id = ?1",
        [content_id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )
    .optional()
    .map_err(|e| e.to_string())?
    .ok_or_else(|| "内容不存在".to_string())
}

pub fn update_content_fields(
    conn: &Connection,
    id: &str,
    title: &str,
    body: &str,
    keywords: &[String],
    language: Option<&str>,
) -> Result<(), String> {
    let title = title.trim();
    if title.is_empty() {
        return Err("标题不能为空".into());
    }
    let fields = content_fields_json(title, body, keywords);
    let now = chrono::Utc::now().to_rfc3339();
    if let Some(language) = language.map(str::trim).filter(|value| !value.is_empty()) {
        conn.execute(
            "UPDATE content_items SET title = ?1, fields_json = ?2, language = ?3, updated_at = ?4 WHERE id = ?5",
            rusqlite::params![title, fields, language, now, id],
        )
    } else {
        conn.execute(
            "UPDATE content_items SET title = ?1, fields_json = ?2, updated_at = ?3 WHERE id = ?4",
            rusqlite::params![title, fields, now, id],
        )
    }
    .map_err(|e| e.to_string())?;
    if conn.changes() == 0 {
        return Err("内容条目不存在".into());
    }
    sync_content_fts(conn, id, title, body)?;
    Ok(())
}

pub fn move_content_to_project(
    conn: &Connection,
    content_ids: &[String],
    dest_project_id: &str,
) -> Result<usize, String> {
    let exists: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM projects WHERE id = ?1 AND archived_at IS NULL",
            [dest_project_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    if exists == 0 {
        return Err("目标项目不存在".into());
    }
    let mut moved = 0usize;
    for content_id in content_ids {
        let media = list_media_for_content(conn, content_id)?;
        conn.execute(
            "UPDATE content_items SET project_id = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![dest_project_id, chrono::Utc::now().to_rfc3339(), content_id],
        )
        .map_err(|e| e.to_string())?;
        if conn.changes() == 0 {
            continue;
        }
        moved += 1;
        for asset in media {
            let still_shared: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM content_media cm
                     JOIN content_items ci ON ci.id = cm.content_item_id
                     WHERE cm.media_asset_id = ?1 AND ci.project_id != ?2",
                    rusqlite::params![asset.id, dest_project_id],
                    |row| row.get(0),
                )
                .unwrap_or(0);
            if still_shared == 0 {
                let taken: i64 = conn
                    .query_row(
                        "SELECT COUNT(*) FROM media_assets WHERE project_id = ?1 AND path = ?2 AND id != ?3",
                        rusqlite::params![dest_project_id, asset.path, asset.id],
                        |row| row.get(0),
                    )
                    .unwrap_or(0);
                if taken == 0 {
                    conn.execute(
                        "UPDATE media_assets SET project_id = ?1 WHERE id = ?2",
                        rusqlite::params![dest_project_id, asset.id],
                    )
                    .map_err(|e| e.to_string())?;
                } else {
                    retarget_shared_media(conn, content_id, &asset, dest_project_id)?;
                }
            } else {
                retarget_shared_media(conn, content_id, &asset, dest_project_id)?;
            }
        }
    }
    Ok(moved)
}

fn retarget_shared_media(
    conn: &Connection,
    content_id: &str,
    asset: &MediaListRow,
    dest_project_id: &str,
) -> Result<(), String> {
    let existing: Option<String> = conn
        .query_row(
            "SELECT id FROM media_assets WHERE project_id = ?1 AND path = ?2",
            rusqlite::params![dest_project_id, asset.path],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let dest_media_id = if let Some(id) = existing {
        id
    } else {
        let new_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO media_assets (
                id, path, file_name, kind, size_bytes, indexed_at, project_id, width, height, thumb_status, thumb_error
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![
                new_id,
                asset.path,
                asset.file_name,
                asset.kind,
                asset.size_bytes,
                now,
                dest_project_id,
                asset.width,
                asset.height,
                if asset.thumb_status.is_empty() { "pending" } else { asset.thumb_status.as_str() },
                asset.thumb_error,
            ],
        )
        .map_err(|e| e.to_string())?;
        new_id
    };
    conn.execute(
        "UPDATE content_media SET media_asset_id = ?1 WHERE content_item_id = ?2 AND media_asset_id = ?3",
        rusqlite::params![dest_media_id, content_id, asset.id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn rename_default_project_if_untouched(conn: &Connection, preferred_name: &str) -> Result<(), String> {
    let preferred = preferred_name.trim();
    if preferred.is_empty() || preferred == "默认项目" {
        return Ok(());
    }
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))
        .unwrap_or(0);
    if count != 1 {
        return Ok(());
    }
    conn.execute(
        "UPDATE projects SET name = ?1, updated_at = ?2 WHERE name = '默认项目'",
        rusqlite::params![preferred, chrono::Utc::now().to_rfc3339()],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod fts_tests {
    use super::build_fts_query;

    #[test]
    fn builds_prefix_and_query() {
        let query = build_fts_query("春季 上新").expect("query");
        assert!(query.contains("春季"));
        assert!(query.contains("AND"));
        assert!(query.contains('*'));
    }

    #[test]
    fn empty_query_returns_none() {
        assert!(build_fts_query("   ").is_none());
    }
}

#[cfg(test)]
mod duplicate_publish_tests {
    use super::*;

    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("open in-memory db");
        migrate(&conn).expect("migrate");
        conn
    }

    fn insert_test_task(conn: &Connection, task_id: &str, content_id: &str, channel_id: &str) {
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO content_items (id, source_path, source_anchor, title, fields_json, created_at, updated_at)
             VALUES (?1, '/test.md', 'a', 'Test', '{}', ?2, ?2)",
            rusqlite::params![content_id, now],
        )
        .expect("insert content");
        conn.execute(
            "INSERT INTO publish_tasks (id, content_item_id, channel_id, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, 'ready', ?4, ?4)",
            rusqlite::params![task_id, content_id, channel_id, now],
        )
        .expect("insert task");
    }

    #[test]
    fn warns_when_same_task_already_published() {
        let conn = test_conn();
        insert_test_task(&conn, "task-1", "content-1", "xhs");
        update_publish_task_status(&conn, "task-1", "published", Some("https://example.com"), None, None, None)
            .expect("publish");

        let warning =
            duplicate_publish_warning(&conn, "content-1", "xhs", "task-1", 30).expect("query");
        assert!(
            warning.is_some(),
            "same task already published should trigger warning"
        );
    }

    #[test]
    fn no_warning_for_never_published() {
        let conn = test_conn();
        insert_test_task(&conn, "task-1", "content-1", "xhs");

        let warning =
            duplicate_publish_warning(&conn, "content-1", "xhs", "task-1", 30).expect("query");
        assert!(warning.is_none());
    }

    #[test]
    fn warns_after_undo_when_published_at_retained() {
        let conn = test_conn();
        insert_test_task(&conn, "task-1", "content-1", "xhs");
        update_publish_task_status(&conn, "task-1", "published", None, None, None, None).expect("publish");
        update_publish_task_status(&conn, "task-1", "ready", None, None, None, None).expect("undo");

        let warning =
            duplicate_publish_warning(&conn, "content-1", "xhs", "task-1", 30).expect("query");
        assert!(
            warning.is_some(),
            "re-publish after undo within 30 days should warn"
        );
    }
}

#[cfg(test)]
mod ledger_tests {
    use super::*;

    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("open in-memory db");
        migrate(&conn).expect("migrate");
        conn
    }

    fn labels() -> LedgerLabels {
        LedgerLabels {
            sheet: "发布记录".into(),
            date: "日期".into(),
            project: "项目".into(),
            channel: "平台".into(),
            status: "状态".into(),
            language: "语言".into(),
            title: "标题".into(),
            body: "正文".into(),
            url: "发布链接".into(),
            note: "备注".into(),
            status_draft: "草稿".into(),
            status_ready: "待发".into(),
            status_scheduled: "已排期".into(),
            status_published: "已发布".into(),
            status_archived: "已归档".into(),
            status_blocked: "阻塞".into(),
            lang_zh: "中文".into(),
            lang_en: "English".into(),
            lang_bilingual: "中英".into(),
        }
    }

    fn insert_piece(
        conn: &Connection,
        id: &str,
        project_id: &str,
        title: &str,
        language: &str,
        body: &str,
    ) {
        insert_content_item(
            conn,
            id,
            "",
            "manual://ledger",
            "{}",
            title,
            language,
            &content_fields_json(title, body, &[]),
            Some(project_id),
            None,
        )
        .expect("insert content");
    }

    fn insert_task(
        conn: &Connection,
        id: &str,
        content_id: &str,
        channel_id: &str,
        status: &str,
        scheduled_at: Option<&str>,
        published_at: Option<&str>,
        url: &str,
        note: &str,
    ) {
        let now = "2026-09-01T12:00:00Z";
        conn.execute(
            "INSERT INTO publish_tasks (
                id, content_item_id, channel_id, status, scheduled_at, published_at,
                publish_url, note, created_at, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)",
            rusqlite::params![id, content_id, channel_id, status, scheduled_at, published_at, url, note, now],
        )
        .expect("insert task");
    }

    #[test]
    fn ledger_rows_follow_the_calendar_day() {
        let conn = test_conn();
        let alpha = create_project(&conn, "甲项目", None, None).expect("project");
        let beta = create_project(&conn, "乙项目", None, None).expect("project");
        insert_piece(&conn, "c1", &alpha.id, "中文稿", "zh", "正文甲");
        insert_piece(&conn, "c2", &beta.id, "English draft", "en", "Body B");
        insert_task(
            &conn,
            "t1",
            "c1",
            "xhs",
            "ready",
            Some("2026-10-02T12:00:00Z"),
            None,
            "",
            "先发小红书",
        );
        insert_task(
            &conn,
            "t2",
            "c1",
            "pinterest",
            "published",
            Some("2026-10-01T12:00:00Z"),
            Some("2026-10-03T12:00:00Z"),
            "https://pin.example/1",
            "",
        );
        insert_task(
            &conn,
            "t3",
            "c2",
            "pinterest",
            "ready",
            Some("2026-10-02T12:00:00Z"),
            None,
            "",
            "",
        );
        insert_task(&conn, "t4", "c2", "xhs", "draft", Some("2026-10-02T12:00:00Z"), None, "", "");

        let open = ["ready", "scheduled", "published"];
        let statuses: Vec<String> = open.iter().map(|status| (*status).to_string()).collect();
        let rows = list_ledger_rows(&conn, "2026-10-01", "2026-10-07", &[], &statuses).expect("rows");
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].date, "2026-10-02");
        assert_eq!(rows[0].project_name, "乙项目");
        assert_eq!(rows[0].channel_name, "Pinterest");
        assert_eq!(rows[0].title, "English draft");
        assert_eq!(rows[1].date, "2026-10-02");
        assert_eq!(rows[1].project_name, "甲项目");
        assert_eq!(rows[1].channel_name, "小红书");
        assert_eq!(rows[1].title, "中文稿");
        assert_eq!(rows[1].body, "正文甲");
        assert_eq!(rows[1].note, "先发小红书");
        assert_eq!(rows[2].date, "2026-10-03");
        assert_eq!(rows[2].status, "published");
        assert_eq!(rows[2].publish_url, "https://pin.example/1");
        assert!(rows.iter().all(|row| row.status != "draft"));

        let october_first = list_ledger_rows(&conn, "2026-10-01", "2026-10-02", &[], &statuses).expect("narrow");
        assert!(october_first.iter().all(|row| row.date != "2026-10-03"));

        let only_beta = list_ledger_rows(&conn, "2026-10-01", "2026-10-07", &[beta.id.clone()], &statuses).expect("beta");
        assert_eq!(only_beta.len(), 1);
        assert_eq!(only_beta[0].title, "English draft");

        let table = ledger_table(&rows, &labels());
        assert_eq!(
            table[0],
            vec!["日期", "项目", "平台", "状态", "语言", "标题", "正文", "发布链接", "备注"]
        );
        assert_eq!(table[1][3], "待发");
        assert_eq!(table[1][4], "English");
        assert_eq!(table[2][4], "中文");
        assert_eq!(table[3][3], "已发布");
    }
}
