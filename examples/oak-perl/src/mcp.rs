#![cfg(feature = "mcp")]

use crate::lsp::PerlLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Perl semantics (Stdio).
pub async fn serve_perl_mcp(vfs: MemoryVfs) {
    let service = PerlLanguageService::new(vfs);
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

/// One-click Axum integration for Perl MCP.
#[cfg(feature = "axum")]
pub async fn serve_perl_mcp_axum(vfs: MemoryVfs) {
    let service = PerlLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3087").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
