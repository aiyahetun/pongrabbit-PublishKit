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

    Ok(())
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
        "DELETE FROM content_items WHERE source_path = ?1",
        [source_path],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
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
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO content_items (
            id, source_document_id, source_path, source_anchor, title,
            language, fields_json, created_at, updated_at
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            id,
            source_document_id,
            source_path,
            source_anchor,
            title,
            language,
            fields_json,
            now,
            now
        ],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn list_content_items(
    conn: &Connection,
) -> Result<Vec<(String, String, String, String, String, String)>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, title, source_path, language, fields_json, created_at
             FROM content_items
             ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
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

    Ok(rows)
}

pub fn delete_content_items(conn: &Connection, ids: &[String]) -> Result<usize, String> {
    if ids.is_empty() {
        return Ok(0);
    }
    let placeholders = (1..=ids.len())
        .map(|index| format!("?{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!("DELETE FROM content_items WHERE id IN ({placeholders})");
    conn.execute(&sql, rusqlite::params_from_iter(ids.iter()))
        .map_err(|e| e.to_string())?;
    Ok(conn.changes() as usize)
}

pub fn list_channels(conn: &Connection) -> Result<Vec<(String, String, String, String, i64)>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, name, market, COALESCE(color, '#888888'), is_custom
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
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(rows)
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

pub fn update_publish_task_status(
    conn: &Connection,
    task_id: &str,
    status: &str,
    publish_url: Option<&str>,
    note: Option<&str>,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "UPDATE publish_tasks SET
            status = ?1,
            publish_url = COALESCE(?2, publish_url),
            note = COALESCE(?3, note),
            scheduled_at = CASE
                WHEN ?1 = 'ready' AND scheduled_at IS NULL THEN ?4
                ELSE scheduled_at
            END,
            published_at = CASE WHEN ?1 = 'published' THEN ?4 ELSE published_at END,
            updated_at = ?4
         WHERE id = ?5",
        rusqlite::params![status, publish_url, note, now, task_id],
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
) -> Result<Vec<(String, String, String, String, String, String, String, String, String, String, String, String, String, String)>, String> {
    let sql = if status_filter.is_some() {
        "SELECT
            t.id, t.status, t.publish_url, t.note, t.updated_at,
            COALESCE(t.scheduled_at, ''), COALESCE(t.published_at, ''),
            c.id, c.name, COALESCE(c.color, '#888888'),
            i.id, i.title, i.language, i.fields_json
         FROM publish_tasks t
         JOIN channels c ON c.id = t.channel_id
         JOIN content_items i ON i.id = t.content_item_id
         WHERE t.status = ?1
         ORDER BY t.updated_at DESC"
    } else {
        "SELECT
            t.id, t.status, t.publish_url, t.note, t.updated_at,
            COALESCE(t.scheduled_at, ''), COALESCE(t.published_at, ''),
            c.id, c.name, COALESCE(c.color, '#888888'),
            i.id, i.title, i.language, i.fields_json
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

pub fn upsert_media_asset(
    conn: &Connection,
    id: &str,
    path: &str,
    file_name: &str,
    kind: &str,
    size_bytes: i64,
    mtime: Option<&str>,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO media_assets (id, path, file_name, kind, size_bytes, mtime, indexed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(path) DO UPDATE SET
           file_name = excluded.file_name,
           kind = excluded.kind,
           size_bytes = excluded.size_bytes,
           mtime = excluded.mtime,
           indexed_at = excluded.indexed_at",
        rusqlite::params![id, path, file_name, kind, size_bytes, mtime, now],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

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
) -> Result<Vec<(String, String, String, String, i64, String)>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, path, file_name, kind, size_bytes, indexed_at
             FROM media_assets
             ORDER BY file_name ASC",
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
                row.get::<_, String>(5)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

pub fn list_media_for_content(
    conn: &Connection,
    content_item_id: &str,
) -> Result<Vec<(String, String, String, String, i64)>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT m.id, m.path, m.file_name, m.kind, m.size_bytes
             FROM content_media cm
             JOIN media_assets m ON m.id = cm.media_asset_id
             WHERE cm.content_item_id = ?1
             ORDER BY cm.sort_order ASC, m.file_name ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([content_item_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(rows)
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
) -> Result<Vec<(String, String, String, String, String, String, String)>, String> {
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
                c.name, COALESCE(c.color, '#888888'), i.title, t.publish_url
             FROM publish_tasks t
             JOIN channels c ON c.id = t.channel_id
             JOIN content_items i ON i.id = t.content_item_id
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
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(rows)
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
        update_publish_task_status(&conn, "task-1", "published", Some("https://example.com"), None)
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
        update_publish_task_status(&conn, "task-1", "published", None, None).expect("publish");
        update_publish_task_status(&conn, "task-1", "ready", None, None).expect("undo");

        let warning =
            duplicate_publish_warning(&conn, "content-1", "xhs", "task-1", 30).expect("query");
        assert!(
            warning.is_some(),
            "re-publish after undo within 30 days should warn"
        );
    }
}
