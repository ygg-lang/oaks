use crate::lsp::NginxLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Nginx semantics (Stdio).
pub async fn serve_nginx_mcp(vfs: MemoryVfs) {
    let service = NginxLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
