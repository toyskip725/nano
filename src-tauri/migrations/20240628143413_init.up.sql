-- Add up migration script here
CREATE TABLE IF NOT EXISTS memo (
  id INTEGER PRIMARY KEY,
  thread TEXT NOT NULL,
  created_at TEXT NOT NULL,
  content TEXT NOT NULL
);