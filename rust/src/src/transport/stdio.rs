use crate::transport::message::{JsonRpcRequest, JsonRpcResponse, RequestId};
use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

pub struct StdioTransport {
    reader: tokio::io::BufReader<tokio::io::Stdin>,
    writer: Mutex<tokio::io::Stdout>,
}

impl StdioTransport {
    pub fn new() -> Self {
        StdioTransport {
            reader: tokio::io::BufReader::new(tokio::io::stdin()),
            writer: Mutex::new(tokio::io::stdout()),
        }
    }

    pub async fn read_request(&mut self) -> Result<JsonRpcRequest> {
        let mut line = String::new();
        let n = self.reader.read_line(&mut line).await?;
        if n == 0 {
            return Err(anyhow::anyhow!("EOF on stdin"));
        }

        let request: JsonRpcRequest = serde_json::from_str(&line)?;

        // Validate JSON-RPC 2.0 format
        if request.jsonrpc != "2.0" {
            return Err(anyhow::anyhow!("Invalid JSON-RPC version"));
        }

        Ok(request)
    }

    pub async fn send_response(&mut self, response: JsonRpcResponse) -> Result<()> {
        let json = serde_json::to_string(&response)?;
        let mut writer = self.writer.lock().await;
        writer.write_all(json.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
        Ok(())
    }

    pub async fn send_error(&mut self, id: Option<RequestId>, code: i32, message: String) -> Result<()> {
        let response = JsonRpcResponse::error(id, code, message);
        self.send_response(response).await
    }
}
