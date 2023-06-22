use crate::lsp::MsilLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for MSIL semantics (Stdio).
pub async fn serve_msil_mcp(vfs: MemoryVfs) {
    let service = MsilLanguageService::new(vfs);
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

#[cfg(feature = "axum")]
pub async fn serve_msil_mcp_axum(vfs: MemoryVfs) {
    let service = MsilLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3071").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
