use crate::lsp::PhpLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// PHP MCP server implementation.
///
/// This module provides integration with the Model Context Protocol (MCP)
/// for PHP language services.
pub async fn serve_php_mcp(vfs: MemoryVfs) {
    let service = PhpLanguageService::new(vfs);
    let server = service.into_mcp_server();
    server.run().await.unwrap();
}

/// Serve PHP MCP over Axum.
#[cfg(feature = "axum")]
pub async fn serve_php_mcp_axum(vfs: MemoryVfs) {
    let service = PhpLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3088").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
