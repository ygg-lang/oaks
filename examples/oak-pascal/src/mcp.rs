use crate::lsp::PascalLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Pascal semantics (Stdio).
pub async fn serve_pascal_mcp(vfs: MemoryVfs) {
    let service = PascalLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
