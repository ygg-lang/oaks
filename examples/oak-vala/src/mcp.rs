use crate::lsp::ValaLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// 启动 Vala MCP 服务
pub async fn serve_vala_mcp(vfs: MemoryVfs) {
    let service = ValaLanguageService::new(vfs);
    let server = service.into_mcp_server();
    server.run().await.unwrap();
}

/// 启动 Vala MCP 服务 (Axum)
#[cfg(feature = "axum")]
pub async fn serve_vala_mcp_axum(vfs: MemoryVfs) {
    let service = ValaLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3104").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
