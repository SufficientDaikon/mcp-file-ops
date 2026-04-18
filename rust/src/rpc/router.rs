use crate::errors::FileOpsError;
use crate::transport::message::{JsonRpcRequest, JsonRpcResponse};
use crate::services::{Metrics, RateLimiter};
use crate::tools::ToolRegistry;
use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;

pub struct RpcRouter {
    tool_registry: Arc<ToolRegistry>,
    metrics: Arc<Metrics>,
    rate_limiter: Arc<RateLimiter>,
}

impl RpcRouter {
    pub fn new(tool_registry: Arc<ToolRegistry>, metrics: Arc<Metrics>, rate_limiter: Arc<RateLimiter>) -> Self {
        RpcRouter {
            tool_registry,
            metrics,
            rate_limiter,
        }
    }

    pub async fn dispatch(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let start = Instant::now();
        let request_id = request.id.clone();

        // Rate limiting
        if let Err(_) = self.rate_limiter.check_rate(&request) {
            self.metrics.record_request_error();
            return JsonRpcResponse::error(
                Some(request_id),
                -32005,
                "Rate limit exceeded".to_string(),
            );
        }

        // Route to handler
        let result = match request.method.as_str() {
            "tools/call" => self.handle_tool_call(&request).await,
            _ => Err(anyhow::anyhow!("Unknown method: {}", request.method)),
        };

        let latency_ms = start.elapsed().as_millis() as u64;
        self.metrics.record_latency(&request.method, latency_ms);

        match result {
            Ok(value) => {
                self.metrics.record_request_success();
                JsonRpcResponse::success(request_id, value)
            }
            Err(e) => {
                self.metrics.record_request_error();
                let json_error = if let Some(fe) = e.downcast_ref::<FileOpsError>() {
                    fe.to_json_rpc_error()
                } else {
                    crate::errors::JsonRpcError {
                        code: -32603,
                        message: e.to_string(),
                        data: None,
                    }
                };
                JsonRpcResponse::error(Some(request_id), json_error.code, json_error.message)
            }
        }
    }

    async fn handle_tool_call(&self, request: &JsonRpcRequest) -> Result<serde_json::Value> {
        let tool_name = request
            .params
            .get("tool")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FileOpsError::Other("Missing 'tool' parameter".to_string()))?;

        let tool_input = request.params.get("input").cloned().unwrap_or_default();

        self.tool_registry.call(tool_name, tool_input).await
    }
}
