use std::path::PathBuf;

use walkdir::WalkDir;

// This function is **TEMPORARY**!
pub fn temp_list_dir() -> Vec<String> {
    WalkDir::new(
        PathBuf::from("C:\\Users\\troyd\\Music\\")
            .canonicalize()
            .unwrap_or(PathBuf::from(".")),
    )
    .into_iter()
    .filter_map(|f| f.ok())
    .filter(|f| f.path().to_str().unwrap_or("").ends_with("opus"))
    .filter_map(|f| f.file_name().to_str().map(|f| f.to_string()))
    .collect::<Vec<String>>()
}
