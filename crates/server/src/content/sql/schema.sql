PRAGMA user_version = 1;

CREATE TABLE post_metadata (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  author_id INTEGER NOT NULL,
  author_username TEXT NOT NULL,
  slug TEXT NOT NULL,
  filepath TEXT NOT NULL
);

CREATE INDEX ix_post_metadata_author_username ON post_metadata (author_username);
CREATE INDEX ix_post_metadata_url_safe_title ON post_metadata (url_safe_title);