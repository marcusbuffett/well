//! Module to handle file creation.

use std::fs::{self, File};
use std::path::Path;

pub fn create_file(arguments: &str) -> Result<String, String> {
    let args: serde_json::Value = serde_json::from_str(arguments).unwrap();
    let path = args["path"].as_str().unwrap();
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    File::create(path).map_err(|e| e.to_string())?;
    Ok(path.to_string())
}
