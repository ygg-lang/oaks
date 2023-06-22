use crate::lsp::TypeScriptLanguageService;
use oak_mcp::McpServer;
use oak_vfs::MemoryVfs;

/// 启动 TypeScript MCP 服务
pub async fn serve_typescript_mcp(vfs: MemoryVfs) {
    let service = TypeScriptLanguageService::new(vfs);
    let server = McpServer::new(service);
    server.run().await.unwrap();
}
