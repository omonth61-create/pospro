#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use tauri::{Manager, generate_context};
use tauri_plugin_sqlite::SqlitePlugin;

fn main() {
  tauri::Builder::default()
    .plugin(SqlitePlugin::default())
    .run(generate_context!())
    .expect("error while running tauri application");
}
