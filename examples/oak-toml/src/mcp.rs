use crate::lsp::TomlLanguageService;
use oak_vfs::MemoryVfs;

/// 为 TOML 语义启动 MCP 服务器 (Stdio)。
#[cfg(feature = "mcp-stdio")]
pub async fn serve_toml_mcp(vfs: MemoryVfs) {
    let service = TomlLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
