// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{async_runtime::block_on, State};
use database::{create_connection_pool, Memo};

mod database;

fn main() -> Result<(), sqlx::Error> {
  let db_pool = block_on(create_connection_pool())?;

  tauri::Builder::default()
    .manage(db_pool)
    .invoke_handler(tauri::generate_handler![cmd_create])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}

#[tauri::command]
fn cmd_create(
  db_pool: State<'_, sqlx::SqlitePool>,
  id: i64,
  thread: String,
  created_at: String,
  content: String
) -> () {
  let new_memo: Memo = Memo::new(id, thread, created_at, content);
  println!("create: {:?}", &new_memo);

  let _ = block_on(database::create_memo(&*db_pool, new_memo));
}