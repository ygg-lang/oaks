#![cfg(feature = "mcp")]

use crate::lsp::PascalLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Pascal semantics (Stdio).
pub async fn serve_pascal_mcp(vfs: MemoryVfs) {
    let service = PascalLanguageService::new(vfs);
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

#[cfg(feature = "axum")]
pub async fn serve_pascal_mcp_axum(vfs: MemoryVfs) {
    let service = PascalLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3086").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
