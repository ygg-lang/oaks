use crate::lsp::KotlinLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Kotlin semantics (Stdio).
pub async fn serve_kotlin_mcp(vfs: MemoryVfs) {
    let service = KotlinLanguageService::new(crate::language::KotlinLanguage::default());
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

/// One-click Axum integration for Kotlin MCP.
#[cfg(feature = "axum")]
pub async fn serve_kotlin_mcp_axum(_vfs: MemoryVfs) {
    let service = KotlinLanguageService::new(crate::language::KotlinLanguage::default());
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3058").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
