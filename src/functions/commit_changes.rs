use std::{process::Command, path::Path};

/// Stage and commit all changes in the repository.
pub fn commit_changes(arguments: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Arguments {
        message: String,
    }

    let Arguments { message } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

// println!("Staging and committing changes with message: {}", message);

    let status = Command::new("git")
        .args([&"add", "."])
        .status()
        .map_err(|err| err.to_string())?;

    if !status.success() {
        return Err("Failed to stage changes".to_string());
    }

    let status = Command::new("git")
        .args([&"commit", "-m", &message])
        .status()
        .map_err(|err| err.to_string())?;

    if !status.success() {
        return Err("Failed to commit changes".to_string());
    }

// println!("Changes committed!");

    Ok("Changes committed!".to_string())
}