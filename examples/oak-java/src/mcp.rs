use crate::lsp::JavaLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Java semantics (Stdio).
pub async fn serve_java_mcp(vfs: MemoryVfs) {
    let service = JavaLanguageService::new(vfs, crate::language::JavaLanguage::default());
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

/// One-click Axum integration for Java MCP.
#[cfg(feature = "axum")]
pub async fn serve_java_mcp_axum(vfs: MemoryVfs) {
    let service = JavaLanguageService::new(vfs, crate::language::JavaLanguage::default());
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
