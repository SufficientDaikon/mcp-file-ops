use crate::errors::FileOpsError;
use crate::utils;
use anyhow::Result;
use serde_json::{json, Value};

pub struct FileReadTool;

impl FileReadTool {
    pub async fn execute(params: Value) -> Result<Value> {
        let path = params
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FileOpsError::Other("Missing 'path' parameter".to_string()))?;

        let start_line = params.get("start_line").and_then(|v| v.as_u64()).map(|n| n as usize);
        let end_line = params.get("end_line").and_then(|v| v.as_u64()).map(|n| n as usize);

        let (lines, encoding, line_ending) = utils::read_file_lines(path).await?;

        let total_lines = lines.len();
        let start = start_line.unwrap_or(0);
        let end = end_line.unwrap_or(total_lines);

        if start > total_lines || end > total_lines {
            return Err(FileOpsError::OutOfBounds(end.max(start), total_lines).into());
        }

        let selected_lines: Vec<_> = lines[start..end].to_vec();

        Ok(json!({
            "content": selected_lines.join("\n"),
            "total_lines": total_lines,
            "start_line": start,
            "end_line": end,
            "encoding": encoding,
            "line_ending": line_ending,
        }))
    }
}

#[async_trait::async_trait]
impl super::ToolHandler for FileReadTool {
    async fn call(&self, params: Value) -> Result<Value> {
        FileReadTool::execute(params).await
    }

    fn name(&self) -> &str {
        "file_read"
    }
}
