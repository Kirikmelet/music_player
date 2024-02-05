use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod audio_scanner;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct AudioData {
    path: PathBuf,
    name: String,
    author: String,
    album: String,
}
