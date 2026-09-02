CREATE VIRTUAL TABLE IF NOT EXISTS content_items_fts USING fts5(
  title,
  body,
  content_item_id UNINDEXED,
  tokenize = 'unicode61 remove_diacritics 0'
);
