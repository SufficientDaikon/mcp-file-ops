use crate::errors::FileOpsError;
use crate::utils;
use anyhow::Result;
use serde_json::{json, Value};

pub struct FileEditTool;

impl FileEditTool {
    pub async fn execute(params: Value) -> Result<Value> {
        let path = params
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FileOpsError::Other("Missing 'path' parameter".to_string()))?;

        let edits = params
            .get("edits")
            .and_then(|v| v.as_array())
            .ok_or_else(|| FileOpsError::Other("Missing 'edits' array".to_string()))?;

        let dry_run = params.get("dry_run").and_then(|v| v.as_bool()).unwrap_or(false);
        let expected_hash = params.get("expected_hash").and_then(|v| v.as_str());

        let (mut lines, encoding, line_ending) = utils::read_file_lines(path).await?;

        // Verify hash if provided
        if let Some(hash) = expected_hash {
            let actual_hash = utils::content_hash(&lines);
            if &actual_hash != hash {
                return Err(FileOpsError::ExternalChange {
                    expected: hash.to_string(),
                    actual: actual_hash,
                }
                .into());
            }
        }

        let original = lines.clone();

        // Sort edits bottom-up to avoid line number shifts
        let mut sorted_edits: Vec<_> = edits.iter().collect();
        sorted_edits.sort_by_key(|e| {
            e.get("line")
                .and_then(|v| v.as_u64())
                .unwrap_or(0)
        });
        sorted_edits.reverse();

        for edit in sorted_edits {
            let line_num = edit
                .get("line")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| FileOpsError::Other("Missing 'line' in edit".to_string()))? as usize;

            let old_text = edit
                .get("old_text")
                .and_then(|v| v.as_str())
                .ok_or_else(|| FileOpsError::Other("Missing 'old_text' in edit".to_string()))?;

            let new_text = edit
                .get("new_text")
                .and_then(|v| v.as_str())
                .ok_or_else(|| FileOpsError::Other("Missing 'new_text' in edit".to_string()))?;

            if line_num >= lines.len() {
                return Err(FileOpsError::OutOfBounds(line_num, lines.len()).into());
            }

            if lines[line_num] != old_text {
                return Err(
                    FileOpsError::Other(format!("Line {} content mismatch", line_num)).into()
                );
            }

            lines[line_num] = new_text.to_string();
        }

        let diff = utils::unified_diff(&original, &lines);
        let new_hash = utils::content_hash(&lines);

        if !dry_run {
            utils::atomic_write(path, &lines, &encoding, &line_ending).await?;
        }

        Ok(json!({
            "diff": diff,
            "new_total_lines": lines.len(),
            "new_content_hash": new_hash,
            "applied": !dry_run,
        }))
    }
}

#[async_trait::async_trait]
impl super::ToolHandler for FileEditTool {
    async fn call(&self, params: Value) -> Result<Value> {
        FileEditTool::execute(params).await
    }

    fn name(&self) -> &str {
        "file_edit"
    }
}
