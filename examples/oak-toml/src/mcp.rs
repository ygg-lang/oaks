use crate::lsp::TomlLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// 为 TOML 语义启动 MCP 服务器 (Stdio)。
pub async fn serve_toml_mcp(vfs: MemoryVfs) {
    let service = TomlLanguageService::new(vfs);
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

/// 为 TOML MCP 提供一键式 Axum 集成。
#[cfg(feature = "axum")]
pub async fn serve_toml_mcp_axum(vfs: MemoryVfs) {
    let service = TomlLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3102").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
