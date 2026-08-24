CREATE TABLE IF NOT EXISTS media_assets (
  id TEXT PRIMARY KEY,
  path TEXT NOT NULL UNIQUE,
  file_name TEXT NOT NULL,
  kind TEXT NOT NULL DEFAULT 'image',
  size_bytes INTEGER NOT NULL DEFAULT 0,
  mtime TEXT,
  indexed_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_media_assets_file_name ON media_assets(file_name);

CREATE TABLE IF NOT EXISTS content_media (
  content_item_id TEXT NOT NULL,
  media_asset_id TEXT NOT NULL,
  sort_order INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY (content_item_id, media_asset_id),
  FOREIGN KEY (content_item_id) REFERENCES content_items(id) ON DELETE CASCADE,
  FOREIGN KEY (media_asset_id) REFERENCES media_assets(id) ON DELETE CASCADE
);
