//! Functions available to the model by the function calling api.
use serde_json::json;

mod check_for_errors;
mod create_file;
mod delete_lines;
mod get_code_context;
mod grep;
mod list_commits;
mod list_files;
mod patch_file;
mod read_file;
mod read_file_range;
mod read_well_context;
mod search_replace;
mod show_commit;
mod review_changes;
mod reset_file;

use read_file_range::read_file_range;

use search_replace::search_replace;

use list_files::list_source_files;
use read_file::read_file;

use self::read_file_range::read_file_around;

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
        "update_file" => patch_file::patch_file(arguments),
        "insert_lines" => patch_file::insert_lines(arguments),
        "delete_lines" => delete_lines::delete_lines(arguments),
        "read_file" => read_file(arguments),
        "read_file_range" => read_file_range(arguments),
        "read_file_around" => read_file_around(arguments),
        "check_for_errors" => check_for_errors::check_for_errors(arguments),
        "create_file" => create_file::create_file(arguments),
        "search_and_replace" => search_replace(arguments),
        "get_context" => read_well_context::read_well_context(arguments),
"reset_file" => reset_file::reset_file(arguments),
        "review_changes" => review_changes::review_changes(arguments),
        "grep" => grep::grep(arguments),
        _ => Err(format!("no such function: `{name}`")),
    };
    if result.is_err() {
        eprintln!("function errored: {}", result.as_ref().unwrap_err());
    } else {
        println!("function returned: {}", result.as_ref().unwrap());
    }

    to_json(result).to_string()
}