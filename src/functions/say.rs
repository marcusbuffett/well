use std::{path::Path, process::Command};

/// Use the `say` command to speak text out loud.
pub fn say(arguments: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Arguments {
        text: String,
    }

    let Arguments { text } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

    println!("Speaking out loud: {}", text);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&format!("say '{}'", text))
        .output()
        .map_err(|err| err.to_string())?;

    if !output.status.success() {
        return Err(format!("Failed to speak text: {}", text));
    }

    Ok("Text spoken out loud.".to_string())
}
