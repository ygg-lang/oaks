use crate::lsp::RegexLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Regex semantics (Stdio).
pub async fn serve_regex_mcp(vfs: MemoryVfs) {
    let service = RegexLanguageService::new(vfs);
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

/// One-click Axum integration for Regex MCP.
#[cfg(feature = "axum")]
pub async fn serve_regex_mcp_axum(vfs: MemoryVfs) {
    let service = RegexLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3091").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
