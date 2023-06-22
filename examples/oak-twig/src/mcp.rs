use crate::lsp::TwigLanguageService;
use oak_mcp::McpServer;
use oak_vfs::MemoryVfs;

/// 启动 Twig MCP 服务
pub async fn serve_twig_mcp(vfs: MemoryVfs) {
    let service = TwigLanguageService::new(vfs);
    let server = McpServer::new(service);
    server.run().await.unwrap();
}
