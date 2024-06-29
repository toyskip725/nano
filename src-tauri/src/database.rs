use sqlx::sqlite::{SqlitePool, SqliteQueryResult};

#[derive(Debug)]
pub struct Memo {
  id: i64,
  thread: String,
  created_at: String,
  content: String,
}

impl Memo {
  pub fn new(id: i64, thread: String, created_at: String, content: String) -> Self {
    Self {
      id,
      thread,
      created_at,
      content
    }
  }
}

pub async fn create_connection_pool() -> Result<SqlitePool, sqlx::Error> {
  let database_url = "./database.db";
  let pool = SqlitePool::connect(&database_url).await?;

  Ok(pool)
}

pub async fn create_memo(pool: &SqlitePool, memo: Memo) -> Result<SqliteQueryResult, sqlx::Error> {
  let result = sqlx::query(
    "INSERT INTO memo (id, thread, created_at, content) VALUES (?, ?, ?, ?)")
    .bind(memo.id)
    .bind(memo.thread)
    .bind(memo.created_at)
    .bind(memo.content)
    .execute(pool)
    .await?;

  Ok(result)
}