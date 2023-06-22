use crate::lsp::ActionScriptLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for ActionScript semantics (Stdio).
pub async fn serve_actionscript_mcp(vfs: MemoryVfs) {
    let service = ActionScriptLanguageService::new(vfs);
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

/// One-click Axum integration for ActionScript MCP.
#[cfg(feature = "axum")]
pub async fn serve_actionscript_mcp_axum(vfs: MemoryVfs) {
    let service = ActionScriptLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3075").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
