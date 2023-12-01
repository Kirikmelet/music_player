use anyhow::{Ok, Result};
use std::{
    fs::{DirBuilder, File},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppConfig {
    pub dir: AppConfigDir,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfigDir {
    #[serde(default = "AppConfigDir::default_music_dir")]
    pub music_dir: String,
}

impl AppConfigDir {
    fn default_music_dir() -> String {
        directories::UserDirs::new()
            .and_then(|f| f.audio_dir().map(|f| f.to_path_buf()))
            .map_or_else(|| String::from("no"), |f| f.display().to_string())
    }
}

impl Default for AppConfigDir {
    fn default() -> Self {
        Self {
            music_dir: Self::default_music_dir(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AppConfigHandler {
    config: AppConfig,
    config_path: PathBuf,
}

impl AppConfigHandler {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            DirBuilder::new().recursive(true).create(&path)?;
        }
        let config_path = path.join("config.toml");
        if !config_path.exists() {
            Self::write_default(&config_path)?;
        }
        let config = Self::read_config_file(&config_path).unwrap_or_default();
        Ok(Self {
            config,
            config_path,
        })
    }
    pub fn write_default<P: AsRef<Path>>(file_path: P) -> Result<()> {
        let file = File::create(file_path.as_ref())?;
        let mut writer = BufWriter::new(file);
        writer.write_all(toml_edit::ser::to_vec(&AppConfig::default())?.as_slice())?;
        Ok(())
    }
    pub fn read_config_file<P: AsRef<Path>>(config_path: P) -> Result<AppConfig> {
        let file = File::open(config_path.as_ref())?;
        let mut reader = BufReader::new(file);
        let mut file_buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut file_buffer)?;
        let config: AppConfig = toml_edit::de::from_slice(file_buffer.as_slice())?;
        Ok(config)
    }
    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }
}
