use std::{fs::File, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub music_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            music_dir: dirs::audio_dir().unwrap(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let Some(config_dir) = Self::get_config_dir() else {
            return Self::default();
        };
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir.clone()).unwrap();
        }
        let config_file = Self::get_config_file(config_dir.clone());
        Self::read_config_file(config_file)
    }
    fn get_config_dir() -> Option<PathBuf> {
        match dirs::config_local_dir() {
            Some(mut x) => {
                x.push("music_player_test");
                Some(x)
            }
            None => None,
        }
    }
    fn get_config_file(mut config_dir: PathBuf) -> PathBuf {
        config_dir.push("config.toml");
        config_dir
    }
    fn read_config_file(config_file: PathBuf) -> Self {
        if !config_file.exists() {
            let mut file =
                File::create(config_file.clone()).expect("Already guarded for config_dir!");
            file.write(
                toml_edit::ser::to_string(&Self::default())
                    .unwrap()
                    .into_bytes()
                    .as_slice(),
            )
            .unwrap();
        }
        let config_file_contents = std::fs::read_to_string(config_file.clone()).unwrap();
        toml_edit::de::from_str(&config_file_contents).unwrap_or(Default::default())
    }
}
