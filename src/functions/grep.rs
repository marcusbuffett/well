use std::process::Command;

pub fn grep(arguments: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Arguments {
        query: String,
    }

    let Arguments { query } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

    let output = Command::new("rg")
        .arg(&query)
        .arg("--color=never")
        .output()
        .map_err(|err| err.to_string())?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("rg command failed: {}", error_message));
    }

    let results = String::from_utf8_lossy(&output.stdout);
    Ok(results.to_string())
}