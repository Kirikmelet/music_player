use std::path::{Path, PathBuf};

use anyhow::{Ok, Result};
use jammdb::{Data, Error, DB};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub mod audio_scanner;

pub const DB_BUCKET_AUDIO: &'static str = "audio";

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct AudioData {
    path: PathBuf,
    name: String,
    author: String,
    album: String,
}

#[derive(Clone)]
pub struct AppDB {
    db: DB,
}

impl AppDB {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            std::fs::DirBuilder::new().recursive(true).create(&path)?;
        }
        let db = DB::open(path.join("db.db"))?;
        Ok(Self { db })
    }
    pub fn get_db(&self) -> &DB {
        &self.db
    }
    pub fn open_db(&mut self, path: PathBuf) -> Result<()> {
        self.db = DB::open(path)?;
        Ok(())
    }
    pub fn refresh_audio_db(&self, audio_list: Vec<String>) -> Result<()> {
        let db = self.get_db();
        let tx = db.tx(true)?;
        let audio_bucket = tx.get_or_create_bucket(DB_BUCKET_AUDIO)?;
        let mut index: u8 = 0;
        for i in audio_list {
            let encoded_str = rmp_serde::encode::to_vec(&i)?;
            audio_bucket.put(index.to_ne_bytes(), encoded_str)?;
            index += 1;
        }
        Ok(())
    }
    pub fn get_audio_db(&self) -> Result<Vec<String>> {
        let db = self.get_db();
        let tx = db.tx(true)?;
        let mut return_value: Vec<String> = Vec::new();
        let audio_bucket = tx.get_or_create_bucket(DB_BUCKET_AUDIO)?;
        let mut audio_iter = audio_bucket.kv_pairs();
        while let Some(item) = audio_iter.next() {
            let stuff: String = rmp_serde::decode::from_slice(item.value()).unwrap_or_default();
            return_value.push(stuff);
        }
        Ok(return_value)
    }
}
