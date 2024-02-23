// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(try_blocks)]

mod config;
mod data;
use std::fs;

use config::{Config, ConfigSerialized};
use data::{ensure_everything_exists, CONFIG};
use serde::Serialize;

use crate::data::get_config_local_dir;

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

#[tauri::command]
fn remote_server_post(path: String, params: serde_json::Value) -> String {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url_of(&path))
        .query(params.as_object().unwrap())
        .send()
        .unwrap();
    response.text().unwrap()
}

#[tauri::command]
fn post_request() -> String {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url_of("/test"))
        .body("line1\nline2")
        .send()
        .unwrap();
    response.text().unwrap()
}

#[derive(Serialize)]
#[serde(tag = "status")]
enum InitInfo {
    FreshInstall,
    // The remote_url is only provided for the user's understanding. Only make
    // requests from the backend
    ConfigExists { remote_url: String },
}

#[tauri::command]
async fn startup() -> InitInfo {
    match CONFIG.read().await.to_owned() {
        Some(config) => InitInfo::ConfigExists {
            remote_url: config.remote_url.clone(),
        },
        None => InitInfo::FreshInstall,
    }
}

#[tauri::command]
async fn write_config(url: String) {
    {
        let mut lock = CONFIG.write().await;
        // println!("remote_url saved! {:?}", (url));
        *lock = Some(Config { remote_url: url });
    }

    let serializable_config: ConfigSerialized = CONFIG.read().await.to_owned().unwrap().into();
    // TODO: Error handling
    let serialized = serde_json::to_string_pretty(&serializable_config)
        .expect("Config should serialize without issue");

    let mut config_file = get_config_local_dir();
    config_file.push("config.json");
    fs::write(config_file, serialized).expect("file to be written without issue");
}

fn main() -> anyhow::Result<()> {
    ensure_everything_exists()?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            write_config,
            startup,
            testing,
            remote_server_get,
            remote_server_post,
            post_request,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
