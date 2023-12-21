use std::process::Command;

pub fn review_changes(arguments: &str) -> Result<String, String> {
    // Run the compile command as a shell command
    let output = Command::new("git")
        .arg("diff")
        .output()
        .map_err(|err| err.to_string())?;

    // Return the output as a String
    // let stdout = String::from_utf8(output.stdout).map_err(|err| err.to_string())?;
    let stderr = String::from_utf8(output.stderr).map_err(|err| err.to_string())?;

    let results = String::from_utf8_lossy(&output.stdout);
    Ok(results.to_string())
}
