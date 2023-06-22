use crate::lsp::TailwindLanguageService;
use oak_vfs::MemoryVfs;

/// 启动 Tailwind MCP 服务
#[cfg(feature = "mcp-stdio")]
pub async fn serve_tailwind_mcp(vfs: MemoryVfs) {
    let service = TailwindLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
