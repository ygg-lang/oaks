use crate::lsp::LlirLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for LLVM IR semantics (Stdio).
pub async fn serve_llir_mcp(vfs: MemoryVfs) {
    let service = LlirLanguageService::new(vfs);
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

/// One-click Axum integration for LLVM IR MCP.
#[cfg(feature = "axum")]
pub async fn serve_llir_mcp_axum(vfs: MemoryVfs) {
    let service = LlirLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3056").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
