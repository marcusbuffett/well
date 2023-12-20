use serde_json::json;

pub const CONTEXT_PROMPT: &str = r#""\
You are a command-line program to query and edit the codebase with an aid of a large language model.
You have a conversational interface to perform tasks upon the codebase.
You may ask the user for clarifications, or use the functions to navigate the codebase.
Only use the functions you have been provided with.
If you are not sure about the answer, tell so.
You probably want to list the source files as a first step for a lot of problems, including finding the exact path to a file that the user mentions.
If a user says "like we do in x", or something to that effect, you should probably read the code in x and try to copy the pattern.
Do not ask for confirmation to patch a file, just do it.
You can patch a file multiple times, but remember to read the file again after the first patch, as you'll need the new line numbers.
You should at least list the source files, and usually also view some code, to answer questions with the appropriate context.
"""#;

/// List all the functions as a JSON schema understood by the model.
pub fn all_functions() -> serde_json::Value {
    json!([
        {"name": "list_source_files", "description": "list all source files", "parameters": {
            "type": "object",
            "properties": {},
            "required": [],
        }},
        {"name": "patch_file", "description": "update file contents from one line numbet to another", "parameters": {
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "relative path to the file" },
                "code": { "type": "string", "description": "the new code" },
                "start_line": { "type": "number", "description": "the line number to start patching" },
                "end_line": { "type": "number", "description": "the line number to end patching" },
            },
            "required": [],
        }},
        {"name": "insert_lines", "description": "insert lines into a file", "parameters": {
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "relative path to the file" },
                "code": { "type": "string", "description": "the new code" },
                "after_line": { "type": "number", "description": "the line number to insert after" },
            },
            "required": [],
        }},
        {"name": "read_file", "description": "read file", "parameters": {
            "type": "object",
            "properties": { "path": { "type": "string", "description": "relative path to the file to read" } },
            "required": ["path"],
        }},
        {"name": "create_file", "description": "create a file with a path", "parameters": {
            "type": "object",
            "properties": { "path": { "type": "string", "description": "path where to create the file" } },
            "required": ["path"],
        }},
        {"name": "grep", "description": "search for a string in the codebase", "parameters": {
            "type": "object",
            "properties": { "query": { "type": "string", "description": "the string to search for" } },
            "required": ["query"],
        }},
        {
            "name": "check_for_errors", "description": "check for errors in the codebase", "parameters": {
            "type": "object",
            "properties": {},
            "required": [],
        }}
    ])
}
