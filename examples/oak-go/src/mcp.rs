use crate::lsp::GoLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Go semantics (Stdio).
pub async fn serve_go_mcp(vfs: MemoryVfs) {
    let service = GoLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
