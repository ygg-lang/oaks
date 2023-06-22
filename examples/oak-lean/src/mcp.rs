use crate::lsp::LeanLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Lean semantics (Stdio).
pub async fn serve_lean_mcp(vfs: MemoryVfs) {
    let service = LeanLanguageService::new(crate::language::LeanLanguage::default());
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

/// One-click Axum integration for Lean MCP.
#[cfg(feature = "axum")]
pub async fn serve_lean_mcp_axum(_vfs: MemoryVfs) {
    let service = LeanLanguageService::new(crate::language::LeanLanguage::default());
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3055").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
