use crate::lsp::MatlabLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for MATLAB semantics (Stdio).
pub async fn serve_matlab_mcp(vfs: MemoryVfs) {
    let service = MatlabLanguageService::new(vfs);
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

/// One-click Axum integration for MATLAB MCP.
#[cfg(feature = "axum")]
pub async fn serve_matlab_mcp_axum(vfs: MemoryVfs) {
    let service = MatlabLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3047").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
