use crate::lsp::AdaLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// 为 Ada 语言启动 MCP 服务
pub async fn serve_ada_mcp(vfs: MemoryVfs) {
    let service = AdaLanguageService::new(vfs);
    let server = service.into_mcp_server();
    server.run().await.unwrap();
}

/// 为 Ada 语言启动基于 Axum 的 MCP 服务
#[cfg(feature = "axum")]
pub async fn serve_ada_mcp_axum(vfs: MemoryVfs) {
    let service = AdaLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3070").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
