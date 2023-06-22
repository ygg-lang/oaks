use crate::lsp::ScssLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for SCSS semantics (Stdio).
pub async fn serve_scss_mcp(vfs: MemoryVfs) {
    let service = ScssLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
