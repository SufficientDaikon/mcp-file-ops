use crate::errors::FileOpsError;
use crate::utils;
use anyhow::Result;
use serde_json::{json, Value};

pub struct FileInsertTool;

impl FileInsertTool {
    pub async fn execute(params: Value) -> Result<Value> {
        let path = params
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FileOpsError::Other("Missing 'path' parameter".to_string()))?;

        let line_num = params
            .get("line")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| FileOpsError::Other("Missing 'line' parameter".to_string()))? as usize;

        let content = params
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FileOpsError::Other("Missing 'content' parameter".to_string()))?;

        let (mut lines, encoding, line_ending) = utils::read_file_lines(path).await?;
        let original = lines.clone();

        if line_num > lines.len() {
            return Err(FileOpsError::OutOfBounds(line_num, lines.len()).into());
        }

        lines.insert(line_num, content.to_string());

        let diff = utils::unified_diff(&original, &lines);
        let new_hash = utils::content_hash(&lines);

        utils::atomic_write(path, &lines, &encoding, &line_ending).await?;

        Ok(json!({
            "diff": diff,
            "new_total_lines": lines.len(),
            "new_content_hash": new_hash,
        }))
    }
}

#[async_trait::async_trait]
impl super::ToolHandler for FileInsertTool {
    async fn call(&self, params: Value) -> Result<Value> {
        FileInsertTool::execute(params).await
    }

    fn name(&self) -> &str {
        "file_insert"
    }
}
