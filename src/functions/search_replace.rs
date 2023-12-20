use std::process::Command;

pub fn search_replace(arguments: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Arguments {
        search: String,
        replace: String,
        path: String,
    }

    let Arguments { search, replace, path } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

    let output = Command::new("sed")
        .arg("-i")
        .arg(format!("s/{}/{}/g", search, replace))
        .arg(path)
        .output()
        .map_err(|err| err.to_string())?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("sed command failed: {}", error_message));
    }

    Ok("Search and replace completed successfully".to_string())
}