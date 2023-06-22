use crate::lsp::SchemeLanguageService;
use oak_mcp::McpServer;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Scheme semantics (Stdio).
pub async fn serve_scheme_mcp(vfs: MemoryVfs) {
    let service = SchemeLanguageService::new(vfs);
    let server = McpServer::new(service);

    server.run().await.unwrap();
}
