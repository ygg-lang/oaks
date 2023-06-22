#![cfg(feature = "mcp")]

use crate::lsp::NixLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Nix semantics (Stdio).
pub async fn serve_nix_mcp(vfs: MemoryVfs) {
    let service = NixLanguageService::new(vfs);
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

/// One-click Axum integration for Nix MCP.
#[cfg(feature = "axum")]
pub async fn serve_nix_mcp_axum(vfs: MemoryVfs) {
    let service = NixLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3085").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
