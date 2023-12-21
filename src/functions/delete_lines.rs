use std::{fs, io::Write, path::Path};

pub fn delete_lines(arguments: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Arguments {
        path: String,
        start_line: u32,
        end_line: u32,
    }

    let Arguments {
        path,
        start_line,
        end_line,
    } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

    println!(
        "Deleting lines {} to {} from {}",
        start_line, end_line, path
    );
    let contents = std::fs::read_to_string(Path::new(&path)).map_err(|err| err.to_string())?;
    let lines = contents.lines();
    let new_lines = lines
        .clone()
        .take(start_line as usize - 1)
        .chain(lines.skip(end_line as usize))
        .collect::<Vec<_>>();
    fs::write(&path, new_lines.join("\n")).map_err(|err| err.to_string())?;
    println!("Lines deleted!");

    Ok("Lines deleted!".to_string())
}

