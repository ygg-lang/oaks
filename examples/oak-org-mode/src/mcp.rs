use crate::lsp::OrgModeLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Org-mode semantics (Stdio).
pub async fn serve_org_mode_mcp(vfs: MemoryVfs) {
    let service = OrgModeLanguageService::new(vfs);
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

/// One-click Axum integration for Org-mode MCP.
#[cfg(feature = "axum")]
pub async fn serve_org_mode_mcp_axum(vfs: MemoryVfs) {
    let service = OrgModeLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3046").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
