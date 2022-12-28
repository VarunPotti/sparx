use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub current_dir: PathBuf,
    pub config_dir: PathBuf,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let current_dir = std::env::current_dir()?;
        let mut config_dir = current_dir.join(".sparx");

        // if user is already in the config directory, then both the current_dir and the config_dir are the same
        if current_dir.ends_with(".sparx") {
            config_dir = current_dir.clone();
        }

        if !config_dir.exists() {
            std::fs::create_dir(&config_dir)?;
        }

        Ok(Self {
            current_dir,
            config_dir,
        })
    }
}
