#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use g_email::api::api;

#[tauri::command]
fn zero() -> String {
    return String::from("hello world");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![zero,api::init_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
