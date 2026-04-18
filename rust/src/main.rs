use file_ops_rs::{StdioTransport, RpcRouter, Metrics, RateLimiter, ToolRegistry};
use file_ops_rs::tools::{FileReadTool, FileEditTool, FileInsertTool, FileCreateTool, FileSearchTool, FileStructureTool};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup
    file_ops_rs::services::setup_logging();
    let metrics = Arc::new(Metrics::new());
    let rate_limiter = Arc::new(RateLimiter::new());

    // Register tools
    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(FileReadTool));
    registry.register(Arc::new(FileEditTool));
    registry.register(Arc::new(FileInsertTool));
    registry.register(Arc::new(FileCreateTool));
    registry.register(Arc::new(FileSearchTool));
    registry.register(Arc::new(FileStructureTool));

    // Create router
    let router = RpcRouter::new(Arc::new(registry), metrics, rate_limiter);

    // Main loop
    let mut transport = StdioTransport::new();

    loop {
        match transport.read_request().await {
            Ok(request) => {
                let router = &router;
                let response = router.dispatch(request).await;
                if let Err(e) = transport.send_response(response).await {
                    eprintln!("Error sending response: {}", e);
                }
            }
            Err(e) => {
                if e.to_string().contains("EOF") {
                    break;
                }
                eprintln!("Error reading request: {}", e);
            }
        }
    }

    Ok(())
}
