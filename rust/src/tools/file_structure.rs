use anyhow::Result;
use serde_json::{json, Value};
use regex::Regex;
use std::fs;

pub struct FileStructureTool;

impl FileStructureTool {
    pub async fn execute(params: Value) -> Result<Value> {
        let path = params
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'path' parameter"))?;

        let content = fs::read_to_string(path)?;
        let language = detect_language(path);
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        let outline = match language.as_str() {
            "python" => parse_python(&lines),
            "javascript" | "typescript" => parse_javascript(&lines),
            _ => vec![],
        };

        Ok(json!({
            "outline": outline,
            "language": language,
            "total_lines": total_lines,
        }))
    }
}

fn detect_language(path: &str) -> String {
    if path.ends_with(".py") {
        "python".to_string()
    } else if path.ends_with(".js") {
        "javascript".to_string()
    } else if path.ends_with(".ts") || path.ends_with(".tsx") {
        "typescript".to_string()
    } else {
        "unknown".to_string()
    }
}

fn parse_python(lines: &[&str]) -> Vec<Value> {
    let mut outline = Vec::new();
    let class_re = Regex::new(r"^class\s+(\w+)").unwrap();
    let func_re = Regex::new(r"^(?:\s{4})?def\s+(\w+)").unwrap();

    for (i, line) in lines.iter().enumerate() {
        if let Some(caps) = class_re.captures(line) {
            outline.push(json!({
                "type": "class",
                "name": caps.get(1).map(|m| m.as_str()),
                "line": i,
            }));
        } else if let Some(caps) = func_re.captures(line) {
            outline.push(json!({
                "type": "function",
                "name": caps.get(1).map(|m| m.as_str()),
                "line": i,
            }));
        }
    }

    outline
}

fn parse_javascript(lines: &[&str]) -> Vec<Value> {
    let mut outline = Vec::new();
    let func_re = Regex::new(r"(?:function|const|let)\s+(\w+|{|[)\s*[:\(=]").unwrap();
    let class_re = Regex::new(r"^class\s+(\w+)").unwrap();

    for (i, line) in lines.iter().enumerate() {
        if let Some(caps) = class_re.captures(line) {
            outline.push(json!({
                "type": "class",
                "name": caps.get(1).map(|m| m.as_str()),
                "line": i,
            }));
        } else if let Some(caps) = func_re.captures(line) {
            outline.push(json!({
                "type": "function",
                "name": caps.get(1).map(|m| m.as_str()),
                "line": i,
            }));
        }
    }

    outline
}

#[async_trait::async_trait]
impl super::ToolHandler for FileStructureTool {
    async fn call(&self, params: Value) -> Result<Value> {
        FileStructureTool::execute(params).await
    }

    fn name(&self) -> &str {
        "file_structure"
    }
}
