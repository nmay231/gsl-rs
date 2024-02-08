use anyhow::{anyhow, ensure, Context};
use once_cell::sync::{Lazy, OnceCell};
use std::fs;
use std::path::PathBuf;
use tauri::async_runtime::RwLock;

use crate::config::{Config, ConfigSerialized};

pub static PROJECT_DIRS: OnceCell<directories::ProjectDirs> = OnceCell::new();
pub static CONFIG: Lazy<RwLock<Option<Config>>> = Lazy::new(|| RwLock::new(None));

pub fn ensure_everything_exists() -> anyhow::Result<()> {
    let dirs = directories::ProjectDirs::from("com", "gsl", "gsl-client")
        .ok_or(anyhow!("Could not find home directory for current user"))?;
    ensure!(
        PROJECT_DIRS.set(dirs).is_ok(),
        "PROJECT_DIRS set multiple times"
    );

    ensure!(
        fs::create_dir_all(get_cache_dir()).is_ok(),
        "Could not create user cache directory"
    );
    ensure!(
        fs::create_dir_all(get_config_local_dir()).is_ok(),
        "Could not create user config directory"
    );

    let mut config_file = get_config_local_dir();
    config_file.push("config.json");

    let maybe_config: anyhow::Result<ConfigSerialized> = try {
        let config = fs::read_to_string(&config_file)?;
        let config = serde_json::from_str(&config)?;
        config
    };
    match maybe_config {
        Ok(valid) => {
            let mut lock = CONFIG
                .try_write()
                .with_context(|| "Nothing should be reading the config on startup")?;
            *lock = Some(valid.into());
        }
        Err(err) => {
            if config_file.exists() {
                println!("Invalid config: `{}`\nBacking up for posterity", err);
                // TODO: Yes, newer backups remove older backups. Sue me
                fs::rename(
                    &config_file,
                    &config_file.with_file_name("config.json.invalid"),
                )?;
            }
        }
    }

    Ok(())
}

// TODO: There should be a better way to implement global state than this, but
// testability is not a major concern right now.
pub fn get_cache_dir() -> PathBuf {
    PROJECT_DIRS
        .get()
        .expect("Expected project directories to be initialized")
        .cache_dir()
        .to_path_buf()
}

pub fn get_config_local_dir() -> PathBuf {
    PROJECT_DIRS
        .get()
        .expect("Expected project directories to be initialized")
        .config_local_dir()
        .to_path_buf()
}
