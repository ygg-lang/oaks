use crate::lsp::TexLanguageService;
use oak_vfs::MemoryVfs;

/// 启动 TeX MCP 服务器
pub async fn serve_tex_mcp(vfs: MemoryVfs) {
    let service = TexLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
