use std::{any, path::PathBuf};
use tokio::fs;

use crate::config::AppConfig;

pub struct AudioScanner {
    config: AppConfig,
}

impl AudioScanner {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
    pub async fn scan_dir(&mut self) -> anyhow::Result<Vec<String>> {
        let path = PathBuf::from(&self.config.dir.music_dir);
        let mut return_value: Vec<String> = Vec::new();
        let mut path = fs::read_dir(path).await?;
        while let Ok(Some(item)) = path.next_entry().await {
            return_value.push(
                item.file_name()
                    .to_str()
                    .map(String::from)
                    .unwrap_or_default(),
            )
        }
        anyhow::Ok(return_value)
    }
}
