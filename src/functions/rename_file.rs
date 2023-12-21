use std::{fs, path::Path};

pub fn rename_file(arguments: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Arguments {
        source: String,
        destination: String,
    }

    let Arguments { source, destination } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

    // println!("Renaming file from {} to {}", source, destination);

    fs::rename(&source, &destination).map_err(|err| err.to_string())?;

    // println!("File renamed!");

    Ok("File renamed!".to_string())
}