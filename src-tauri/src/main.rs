// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{async_runtime::block_on, State};
use database::{create_connection_pool, Memo};

mod database;

fn main() -> Result<(), sqlx::Error> {
  let db_pool = block_on(create_connection_pool())?;

  tauri::Builder::default()
    .manage(db_pool)
    .invoke_handler(tauri::generate_handler![
      cmd_create,
      cmd_get_all,
      cmd_update,
      cmd_delete
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}

#[tauri::command]
async fn cmd_create(
  db_pool: State<'_, sqlx::SqlitePool>,
  id: i64,
  thread: String,
  created_at: String,
  content: String
) -> Result<(), String> {
  let new_memo: Memo = Memo::new(id, thread, created_at, content);
  println!("create: {:?}", &new_memo);

  let _ = database::create_memo(&*db_pool, new_memo)
    .await
    .map_err(|e| e.to_string())?;

  Ok(())
}

#[tauri::command]
async fn cmd_get_all(db_pool: State<'_, sqlx::SqlitePool>) -> Result<Vec<Memo>, String> {
  println!("get: all");
  let query_result = database::get_all_memo(&*db_pool)
    .await
    .map_err(|e| e.to_string())?;

  Ok(query_result)
}

#[tauri::command]
async fn cmd_update(
  db_pool: State<'_, sqlx::SqlitePool>,
  id: i64,
  thread: String,
  created_at: String,
  content: String
) -> Result<(), String> {
  let new_memo: Memo = Memo::new(id, thread, created_at, content);
  println!("update: {:?}", &new_memo);

  let _ = database::update_memo(&*db_pool, new_memo)
    .await
    .map_err(|e| e.to_string())?;

  Ok(())
}

#[tauri::command]
async fn cmd_delete(db_pool: State<'_, sqlx::SqlitePool>, id: i64) -> Result<(), String> {
  println!("delete: id={:?}", &id);
  let _ = database::delete_memo(&*db_pool, id)
    .await
    .map_err(|e| e.to_string())?;

  Ok(())
}