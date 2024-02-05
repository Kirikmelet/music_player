use async_stream::stream;
//use futures::Stream;
use std::path::{Path, PathBuf};
use tokio_stream::Stream;

use crate::config::AppConfig;

pub struct AudioScanner {
    config: AppConfig,
}

impl AudioScanner {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
    pub fn scan_dir<P: AsRef<Path>>(music_dir: P) -> impl Stream<Item = PathBuf> {
        stream! {
            let mut path = music_dir.as_ref().to_path_buf();
            // The recursive stack
            let mut recursive_paths: Vec<PathBuf> = Vec::new();
            loop {
                // If this directory is fake, then exit early :)
                let Ok(mut directory) = tokio::fs::read_dir(path).await else {
                    break;
                };
                while let Ok(Some(item)) = directory.next_entry().await {
                        match item.file_type().await {
                            // We want to push this path to the "recursive" stack
                            Ok(x) if x.is_dir() => recursive_paths.push(item.path()),
                            // If anything *but* a directory
                            Ok(x) => {
                                let mut item = item.path();
                                if x.is_symlink() {
                                    // We already checked if it is a symlink
                                    item = std::fs::read_link(item).unwrap();
                                }
                                if Self::check_file_extension(&item) {
                                    // TODO: Get and yield song metadata instead of a string
                                    //yield item.file_name()
                                    //.unwrap_or_default()
                                    //    .to_str()
                                    //    .map(String::from)
                                    //    .unwrap_or_default();
                                    yield item;
                                }
                        },
                            _ => continue,
                        }
                }
                if !recursive_paths.is_empty() {
                    // We already know this element exists
                    path = recursive_paths.pop().unwrap();
                    continue;
                }
                break;
            }
        }
    }
    fn check_file_extension<P: AsRef<Path>>(file: P) -> bool {
        let file = file.as_ref();
        match file.extension() {
            Some(x) => match x.to_str().unwrap_or_default() {
                // TODO: Add more formats
                "opus" => true,
                _ => false,
            },
            _ => false,
        }
    }
    pub fn set_config(&mut self, config: AppConfig) {
        self.config = config;
    }
}
