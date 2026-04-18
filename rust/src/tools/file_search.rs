use crate::errors::FileOpsError;
use anyhow::Result;
use serde_json::{json, Value};
use regex::Regex;
use std::fs;

pub struct FileSearchTool;

impl FileSearchTool {
    pub async fn execute(params: Value) -> Result<Value> {
        let path = params
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FileOpsError::Other("Missing 'path' parameter".to_string()))?;

        let pattern = params
            .get("pattern")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FileOpsError::Other("Missing 'pattern' parameter".to_string()))?;

        let literal = params.get("literal").and_then(|v| v.as_bool()).unwrap_or(false);
        let context = params.get("context").and_then(|v| v.as_u64()).unwrap_or(0) as usize;

        let content = fs::read_to_string(path)?;
        let lines: Vec<&str> = content.lines().collect();

        let mut matches = Vec::new();

        if literal {
            for (i, line) in lines.iter().enumerate() {
                if line.contains(pattern) {
                    matches.push(json!({
                        "line": i,
                        "content": line,
                        "context_before": lines[i.saturating_sub(context)..i]
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>(),
                        "context_after": lines[(i + 1)..(i + 1 + context).min(lines.len())]
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>(),
                    }));
                }
            }
        } else {
            let re = Regex::new(pattern)?;
            for (i, line) in lines.iter().enumerate() {
                if re.is_match(line) {
                    matches.push(json!({
                        "line": i,
                        "content": line,
                        "context_before": lines[i.saturating_sub(context)..i]
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>(),
                        "context_after": lines[(i + 1)..(i + 1 + context).min(lines.len())]
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>(),
                    }));
                }
            }
        }

        Ok(json!({
            "matches": matches,
            "pattern": pattern,
            "total_matches": matches.len(),
        }))
    }
}

#[async_trait::async_trait]
impl super::ToolHandler for FileSearchTool {
    async fn call(&self, params: Value) -> Result<Value> {
        FileSearchTool::execute(params).await
    }

    fn name(&self) -> &str {
        "file_search"
    }
}
