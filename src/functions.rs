//! Functions available to the model by the function calling api.
use serde_json::json;

mod list_files;
mod patch_file;
use list_files::list_files;
use list_files::list_source_files;

mod read_file;
use read_file::read_file;

mod list_commits;
use list_commits::list_commits;

mod show_commit;
use show_commit::show_commit;

mod grep;
use grep::grep;
pub mod check_for_errors;

fn to_json<T, E>(result: Result<T, E>) -> serde_json::value::Value
where
    T: ToString,
    E: ToString,
{
    match result {
        Ok(value) => json!({ "result": value.to_string() }),
        Err(error) => json!({ "error": error.to_string() }),
    }
}

/// Apply a function call to the conversation.
pub fn apply(name: &str, arguments: &str) -> String {
    let result = match name {
        "list_source_files" => list_source_files(),
        "patch_file" => patch_file::patch_file(arguments),
        "insert_lines" => patch_file::insert_lines(arguments),
        "read_file" => read_file(arguments),
        "grep" => grep::grep(arguments),
        "check_for_errors" => check_for_errors::check_for_errors(arguments),
        "create_file" => {
            // Parse the arguments as JSON
            let args: serde_json::Value = serde_json::from_str(arguments).unwrap();
            // Extract the path
            let path = args["path"].as_str().unwrap();
            if path.contains("/") {
                std::fs::create_dir_all(std::path::Path::new(path).parent().unwrap()).unwrap();
            }
            std::fs::File::create(path).unwrap();
            Ok("blah".to_string())
        }
        _ => Err(format!("no such function: `{name}`")),
    };
    if result.is_err() {
        eprintln!("function errored: {}", result.as_ref().unwrap_err());
    }

    to_json(result).to_string()
}
