use std::process::Command;

use itertools::Itertools;

pub fn check_for_errors(arguments: &str) -> Result<String, String> {
    // Read the contents of the ".well/compile" file
    let compile_command =
        std::fs::read_to_string(".well/compile").map_err(|err| err.to_string())?;

    // Run the compile command as a shell command
    let output = Command::new("sh")
        .arg("-c")
        .arg(&compile_command)
        .output()
        .map_err(|err| err.to_string())?;

    // Return the output as a String
    let stdout = String::from_utf8(output.stdout).map_err(|err| err.to_string())?;
    let stderr = String::from_utf8(output.stderr).map_err(|err| err.to_string())?;

    let output_str = if !stderr.is_empty() { stderr } else { stdout };
    if !output.status.success() {
        let final_output = if output_str.lines().collect_vec().len() > 50 {
            Ok(format!(
                "Output limited to 50 lines: {}...",
                output_str.lines().take(50).join("\n")
            ))
        } else {
            Ok(output_str)
        };
        println!("{:?}", final_output);
        return final_output;
    }

    Ok("No errors".to_string())
}
