use std::path::Path;

pub fn read_file_range(arguments: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Arguments {
        path: String,
        start_line: usize,
        end_line: usize,
    }

    let Arguments {
        path,
        start_line,
        end_line,
    } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

    let contents = std::fs::read_to_string(Path::new(&path)).map_err(|err| err.to_string())?;
    let lines: Vec<&str> = contents.lines().collect();
    let start_line = std::cmp::max(1, start_line);
    let end_line = std::cmp::min(lines.len(), end_line);

    let range = start_line - 1..end_line;
    let contents_with_line_numbers = range
        .map(|i| format!("[{}] {}", i + 1, lines[i]))
        .collect::<Vec<String>>()
        .join("\n");

    Ok(contents_with_line_numbers)
}

/// Read a specified range of lines around a given line number.
///
/// # Arguments
///
/// * `arguments` - A JSON string containing `path`, `line_number`, and `context_lines`.
///
/// # Returns
///
/// A result containing the specified range of lines or an error message.
pub fn read_file_around(arguments: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Arguments {
        path: String,
        line_number: usize,
        context_lines: Option<usize>,
    }

    let Arguments {
        path,
        line_number,
        context_lines,
    } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

    let contents = std::fs::read_to_string(Path::new(&path)).map_err(|err| err.to_string())?;
    let lines: Vec<&str> = contents.lines().collect();
    let context_lines = context_lines.unwrap_or(10);
    let start_line = line_number.saturating_sub(context_lines);
    let end_line = std::cmp::min(lines.len(), line_number + context_lines);

    let start_line = std::cmp::max(1, start_line);
    let end_line = std::cmp::min(lines.len(), end_line);

    let range = start_line - 1..end_line;
    dbg!(&range, &start_line, &end_line);
    let contents_with_line_numbers = range
        .map(|i| format!("[{}] {}", i + 1, lines[i]))
        .collect::<Vec<String>>()
        .join("\n");

    Ok(contents_with_line_numbers)
}
