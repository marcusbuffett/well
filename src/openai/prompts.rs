use serde_json::json;

pub const CONTEXT_PROMPT: &str = r#""\
You are a command-line program to query and edit the codebase with an aid of a large language model.
You have a conversational interface to perform tasks upon the codebase.
You may ask the user for clarifications, or use the functions to navigate the codebase.
Only use the functions you have been provided with.
If you are not sure about the answer, tell so.
You probably want to list the source files as a first step for a lot of problems, including finding the exact path to a file that the user mentions.
The main purpose of you operations is to write new code via patch files. 
Do not ask for permission to patch a file, just do it.


"""#;

/// List all the functions as a JSON schema understood by the model.
pub fn all_functions() -> serde_json::Value {
json!([
        {"name": "list_source_files", "description": "list all source files", "parameters": {
            "type": "object",
            "properties": {},
            "required": [],
        }},
        {"name": "patch_file", "description": "apply a patch", "parameters": {
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "relative path to the file" },
                "code": { "type": "string", "description": "the new code" },
                "start_line": { "type": "number", "description": "the line number to start patching" },
                "end_line": { "type": "number", "description": "the line number to end patching" },
            },
            "required": [],
        }},
        {"name": "read_file", "description": "read file", "parameters": {
            "type": "object",
            "properties": { "path": { "type": "string", "description": "relative path to the file to read" } },
            "required": ["path"],
        }},
        {"name": "create_file", "description": "create a file with a specified path", "parameters": {
            "type": "object",
            "properties": { "path": { "type": "string", "description": "relative path to the file to create" } },
            "required": ["path"],
        }}
    ])
}