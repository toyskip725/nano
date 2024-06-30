-- Add up migration script here
CREATE TABLE IF NOT EXISTS memo (
  id INTEGER PRIMARY KEY,
  thread TEXT NOT NULL,
  created_at TEXT NOT NULL,
  content TEXT NOT NULL
);

INSERT INTO memo (id, thread, created_at, content) VALUES (1, "index", "2024/06/30 12:00", "Hello World!");