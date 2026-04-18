pub mod file_read;
pub mod file_edit;
pub mod file_insert;
pub mod file_create;
pub mod file_search;
pub mod file_structure;

pub use file_read::FileReadTool;
pub use file_edit::FileEditTool;
pub use file_insert::FileInsertTool;
pub use file_create::FileCreateTool;
pub use file_search::FileSearchTool;
pub use file_structure::FileStructureTool;

use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait ToolHandler: Send + Sync {
    async fn call(&self, params: Value) -> Result<Value>;
    fn name(&self) -> &str;
}

pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn ToolHandler>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        ToolRegistry {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Arc<dyn ToolHandler>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub async fn call(&self, tool_name: &str, params: Value) -> Result<Value> {
        let tool = self
            .tools
            .get(tool_name)
            .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", tool_name))?;
        tool.call(params).await
    }
}
