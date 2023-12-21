use std::{process::Command, path::Path};

pub fn reset_file(arguments: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Arguments {
        path: String,
    }

    let Arguments { path } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

    println!("Resetting file {}", path);

    let status = Command::new("git")
        .args([&"checkout", "--", &path])
        .status()
        .map_err(|err| err.to_string())?;

    if !status.success() {
        return Err(format!("Failed to reset file: {}", path));
    }

    // println!("File reset!");

    Ok("File reset!".to_string())
}
