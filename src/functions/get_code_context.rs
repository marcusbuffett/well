use std::process::Command;

pub fn get_code_context() -> Result<String, String> {
    // Read the contents of the ".well/context" file
    let context_command = match std::fs::read_to_string(".well/context") {
        Ok(cmd) => cmd,
        Err(_) => return Ok("No code context file".to_string()),
    };

    // Run the context command as a shell command
    let output = Command::new("sh")
        .arg("-c")
        .arg(&context_command)
        .output()
        .map_err(|err| err.to_string())?;

    // Return the output as a String
    let stdout = String::from_utf8(output.stdout).map_err(|err| err.to_string())?;
    let stderr = String::from_utf8(output.stderr).map_err(|err| err.to_string())?;

    let output_str = if !stderr.is_empty() { stderr } else { stdout };
    return Ok(output_str);
}
