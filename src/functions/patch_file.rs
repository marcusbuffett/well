use std::{fs, io::Write, path::Path};

pub fn patch_file(arguments: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Arguments {
        path: String,
        start_line: u32,
        end_line: u32,
        code: String,
    }
    let Arguments {
        path,
        start_line,
        end_line,
        code,
    } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

// println!("Patching {} from {} to {}", path, start_line, end_line);
    let contents = std::fs::read_to_string(Path::new(&path)).map_err(|err| err.to_string())?;
    let lines = contents.lines();
    let new_lines = lines
        .clone()
        .take(start_line as usize - 1)
        .chain(code.lines().into_iter())
        .chain(lines.skip(end_line as usize))
        .collect::<Vec<_>>();
    fs::write(&path, new_lines.join("\n")).map_err(|err| err.to_string())?;
// println!("File patched!");

    Ok("File patched!".to_string())
}

pub fn insert_lines(arguments: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Arguments {
        path: String,
        after_line: u32,
        code: String,
    }
    let Arguments {
        path,
        after_line,
        code,
    } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

// println!("Inserting code into {}, after {}", path, after_line);
    let contents = std::fs::read_to_string(Path::new(&path)).map_err(|err| err.to_string())?;
    let lines = contents.lines();
    let new_lines = lines
        .clone()
        .take(after_line as usize)
        .chain(code.lines().into_iter())
        .chain(lines.clone().skip(after_line as usize))
        .collect::<Vec<_>>();
    fs::write(&path, new_lines.join("\n")).map_err(|err| err.to_string())?;
// println!("Code inserted!");

    Ok("File patched!".to_string())
}
