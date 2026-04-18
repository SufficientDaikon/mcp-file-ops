use serde_json::{json, Value};

pub fn get_tool_schema(tool_name: &str) -> Value {
    match tool_name {
        "file_read" => json!({
            "type": "object",
            "properties": {
                "path": { "type": "string" },
                "start_line": { "type": "integer", "minimum": 0 },
                "end_line": { "type": "integer", "minimum": 0 }
            },
            "required": ["path"]
        }),
        "file_edit" => json!({
            "type": "object",
            "properties": {
                "path": { "type": "string" },
                "edits": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "line": { "type": "integer" },
                            "old_text": { "type": "string" },
                            "new_text": { "type": "string" }
                        },
                        "required": ["line", "old_text", "new_text"]
                    }
                },
                "dry_run": { "type": "boolean" },
                "expected_hash": { "type": "string" }
            },
            "required": ["path", "edits"]
        }),
        "file_insert" => json!({
            "type": "object",
            "properties": {
                "path": { "type": "string" },
                "line": { "type": "integer" },
                "content": { "type": "string" }
            },
            "required": ["path", "line", "content"]
        }),
        "file_create" => json!({
            "type": "object",
            "properties": {
                "path": { "type": "string" },
                "content": { "type": "string" },
                "create_dirs": { "type": "boolean" }
            },
            "required": ["path"]
        }),
        "file_search" => json!({
            "type": "object",
            "properties": {
                "path": { "type": "string" },
                "pattern": { "type": "string" },
                "literal": { "type": "boolean" },
                "context": { "type": "integer" }
            },
            "required": ["path", "pattern"]
        }),
        "file_structure" => json!({
            "type": "object",
            "properties": {
                "path": { "type": "string" }
            },
            "required": ["path"]
        }),
        _ => json!({}),
    }
}
