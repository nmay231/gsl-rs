// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn testing() -> &'static str {
    return "hello from tauri testing command";
}

const REMOTE_SERVER_URL: &'static str = "http://localhost:4040";

fn url_of(path: &str) -> String {
    assert!(path.starts_with('/'));
    format!("{}{}", REMOTE_SERVER_URL, path)
}

#[tauri::command]
fn remote_server_get(path: String) -> String {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url_of(&path)).send().unwrap();
    response.text().unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![testing, remote_server_get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
