use crate::lsp::TexLanguageService;
use oak_core::vfs::MemoryVfs;
use oak_mcp::OakMcpService;

/// 启动 TeX MCP 服务器
#[cfg(feature = "mcp")]
pub async fn serve_tex_mcp(vfs: MemoryVfs) {
    let service = TexLanguageService::new(vfs);
    let server = service.into_mcp_server();
    server.run().await.unwrap();
}

/// 启动 TeX MCP 服务器 (Axum)
#[cfg(all(feature = "mcp", feature = "axum"))]
pub async fn serve_tex_mcp_axum(vfs: MemoryVfs) {
    let service = TexLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3011").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
