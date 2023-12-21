use super::get_code_context;

pub fn read_well_context(arguments: &str) -> Result<String, String> {
    // Read the contents of the ".well/context" file
    let context_contents = std::fs::read_to_string(".well/context.md")
        .map_err(|err| err.to_string())
        .or::<String>(Ok("No context file".to_string()))?;

    let code_context = get_code_context::get_code_context()?;
    // Return the contents as a String
    Ok(format!(
        "Context:\n{}\n\nCode context:\n{}",
        context_contents, code_context
    ))
}
