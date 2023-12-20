use std::path::Path;

pub fn read_file(arguments: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Arguments {
        path: String,
    }
    let Arguments { path } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

    let contents = std::fs::read_to_string(Path::new(&path)).map_err(|err| err.to_string())?;
    let contents_with_line_numbers =
        contents
            .lines()
            .enumerate()
            .fold(String::new(), |mut result, (i, line)| {
                result.push_str(&format!("[{}] {}\n", i + 1, line));
                result
            });
    Ok(contents_with_line_numbers)
}
