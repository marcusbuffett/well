use std::{process::Command, path::Path};

/// Use the `say` command to speak text out loud.
pub fn say(arguments: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Arguments {
        text: String,
    }

    let Arguments { text } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

    println!("Speaking out loud: {}", text);

    let status = Command::new("say")
        .arg(&text)
        .status()
        .map_err(|err| err.to_string())?;

    if !status.success() {
        return Err(format!("Failed to speak text: {}", text));
    }

    Ok("Text spoken out loud.".to_string())
}