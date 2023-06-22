use crate::lsp::ValaLanguageService;
use oak_vfs::MemoryVfs;

/// 启动 Vala MCP 服务
pub async fn serve_vala_mcp(vfs: MemoryVfs) {
    let service = ValaLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
