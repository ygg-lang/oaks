use crate::lsp::ValkyrieLanguageService;
use oak_vfs::MemoryVfs;

/// 启动 Valkyrie MCP 服务
pub async fn serve_valkyrie_mcp(vfs: MemoryVfs) {
    let service = ValkyrieLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
