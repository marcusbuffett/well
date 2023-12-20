pub fn read_well_context(arguments: &str) -> Result<String, String> {
    // Read the contents of the ".well/context" file
    let context_contents =
        std::fs::read_to_string(".well/context").map_err(|err| err.to_string())?;

    // Return the contents as a String
    Ok(context_contents)
}