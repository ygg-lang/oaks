use crate::lsp::ObjectiveCLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Objective-C semantics (Stdio).
pub async fn serve_objective_c_mcp(vfs: MemoryVfs) {
    let service = ObjectiveCLanguageService::new(vfs);
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

/// One-click Axum integration for Objective-C MCP.
#[cfg(feature = "axum")]
pub async fn serve_objective_c_mcp_axum(vfs: MemoryVfs) {
    let service = ObjectiveCLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3045").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
