// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn zero() -> String {
    println!("hello world");
    String::from("hello world")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![zero])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
