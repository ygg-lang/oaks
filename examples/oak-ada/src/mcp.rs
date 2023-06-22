use crate::lsp::AdaLanguageService;
use oak_vfs::MemoryVfs;

/// 为 Ada 语言启动 MCP 服务
pub async fn serve_ada_mcp(vfs: MemoryVfs) {
    let service = AdaLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
