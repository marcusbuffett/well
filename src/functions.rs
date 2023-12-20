//! Functions available to the model by the function calling api.
use serde_json::json;

mod check_for_errors;
mod create_file;
mod grep;
mod list_commits;
mod list_files;
mod patch_file;
mod read_file;
mod show_commit;

use list_files::list_source_files;
use read_file::read_file;

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
        "check_for_errors" => check_for_errors::check_for_errors(arguments),
        "create_file" => create_file::create_file(arguments),
        _ => Err(format!("no such function: `{name}`")),
    };
    if result.is_err() {
        eprintln!("function errored: {}", result.as_ref().unwrap_err());
    }

    to_json(result).to_string()
}
