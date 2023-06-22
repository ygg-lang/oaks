use crate::lsp::VampireLanguageService;
use oak_vfs::MemoryVfs;

/// 启动 Vampire MCP 服务
pub async fn serve_vampire_mcp(vfs: MemoryVfs) {
    let service = VampireLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
