use serde_json::json;

pub const CONTEXT_PROMPT: &str = r#""\
You query and edit the codebase with an aid of a large language model.
You can read and perform tasks upon the local codebase.
You may ask the user for clarifications at any time.
Only use the functions you have been provided with.
Always get the context first using get_context.
If you are not sure about the answer, ask the user for help.
If a user says "like we do in x", or something to that effect, you should read relevant code and try to copy the pattern.
Try to abide by the conventions of the code. You can read other files in order to determine what conventions are appropriate.
Inserting or deleting lines will mean that the line numbers change. Re-read the relevant section before making any further edits.
Make sure to check for errors before considering the task done, and you can ask for help if you need it.
If a file has a lot of lines, you'll need to use the more specific read functionality.
After changing a file, make sure to check the result with read_file_range or read_file_around, to ensure that the change was applied correctly.
Please stop and ask for help if it seems you have gone down the wrong path.
When investigating an error, you should use read_file_around to see the broader context.
You can review_changes at any point, to see what you've done.
If in doubt, feel free to delete your changes and start over.
"""#;

/// List all the functions as a JSON schema understood by the model.
pub fn all_functions() -> serde_json::Value {
    json!([
        {"name": "list_source_files", "description": "list all source files", "parameters": {
            "type": "object",
            "properties": {},
            "required": [],
        }},
        {"name": "update_file", "description": "update file contents", "parameters": {
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "relative path to the file" },
                "code": { "type": "string", "description": "the code to insert" },
                "start_line": { "type": "number", "description": "the line number to start changing code from" },
                "end_line": { "type": "number", "description": "the line number to stop changing code at" },
            },
            "required": ["path", "code", "start_line", "end_line"],
        }},
        {"name": "insert_lines", "description": "insert lines into a file, after a given line number", "parameters": {
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "relative path to the file" },
                "code": { "type": "string", "description": "the code to insert" },
                "after_line": { "type": "number", "description": "the line number to insert after" },
            },
            "required": [],
        }},
        {"name": "delete_lines", "description": "delete lines from a file", "parameters": {
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "the relative path to the file" },
                "start_line": { "type": "number", "description": "the first line number to delete" },
                "end_line": { "type": "number", "description": "the last line number to delete" },
            },
            "required": ["path", "start_line", "end_line"],
        }},
        {"name": "read_file", "description": "read file", "parameters": {
            "type": "object",
            "properties": { "path": { "type": "string", "description": "relative path to the file to read" } },
            "required": ["path"],
        }},
        {
            "name": "read_file_range", "description": "read a range of lines from a file", "parameters": {
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "relative path to the file" },
                    "start_line": { "type": "number", "description": "the start line number" },
                    "end_line": { "type": "number", "description": "the end line number" }
                },
                "required": ["path", "line_number", "number_of_lines"]
            }
        },
        {
            "name": "read_file_around", "description": "read a file around a specific line number", "parameters": {
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "relative path to the file" },
                    "line_number": { "type": "number", "description": "the line number" },
                    "context_lines": { "type": "number", "description": "the number of lines up and down to read" }
                },
                "required": ["path", "line_number"]
            }
        },
        {"name": "create_file", "description": "create a file with a path", "parameters": {
            "type": "object",
            "properties": { "path": { "type": "string", "description": "relative path to the file" } },
            "required": ["path"],
        }},
        {"name": "grep", "description": "search for a string in the codebase or file", "parameters": {
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "the string to search for" },
                "path": { "type": "string", "description": "optional path to a file to search" }
            },
            "required": ["query"],
        }},
        {
            "name": "check_for_errors", "description": "check for errors in the codebase", "parameters": {
            "type": "object",
            "properties": {},
            "required": [],
        }},
        {"name": "review_changes", "description": "Review the changes made to the codebase", "parameters": {
            "type": "object",
            "properties": {},
            "required": [],
        }},
        {"name": "get_context", "description": "Get the context of the code", "parameters": {
            "type": "object",
            "properties": {},
            "required": [],
        }},
        {"name": "search_and_replace", "description": "search for a string and replace it in the codebase", "parameters": {
            "type": "object",
            "properties": {
                "search": { "type": "string", "description": "the string to search for" },
                "replace": { "type": "string", "description": "the string to replace with" },
                "path": { "type": "string", "description": "relative path to the file" }
            },
            "required": ["search", "replace", "path"]
        }},
    ])
}
