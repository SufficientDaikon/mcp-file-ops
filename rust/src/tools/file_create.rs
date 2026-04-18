use crate::errors::FileOpsError;
use crate::utils;
use anyhow::Result;
use serde_json::{json, Value};
use std::path::Path;
use tokio::fs;

pub struct FileCreateTool;

impl FileCreateTool {
    pub async fn execute(params: Value) -> Result<Value> {
        let path = params
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FileOpsError::Other("Missing 'path' parameter".to_string()))?;

        let content = params.get("content").and_then(|v| v.as_str()).unwrap_or("");
        let create_dirs = params.get("create_dirs").and_then(|v| v.as_bool()).unwrap_or(false);

        let path_obj = Path::new(path);

        if create_dirs {
            if let Some(parent) = path_obj.parent() {
                fs::create_dir_all(parent).await?;
            }
        }

        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        utils::atomic_write(path, &lines, "utf-8", "lf").await?;

        let hash = utils::content_hash(&lines);

        Ok(json!({
            "total_lines": lines.len(),
            "size_bytes": content.len(),
            "content_hash": hash,
        }))
    }
}

#[async_trait::async_trait]
impl super::ToolHandler for FileCreateTool {
    async fn call(&self, params: Value) -> Result<Value> {
        FileCreateTool::execute(params).await
    }

    fn name(&self) -> &str {
        "file_create"
    }
}
