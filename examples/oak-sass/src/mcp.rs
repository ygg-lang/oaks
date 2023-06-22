use crate::lsp::SassLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Sass semantics (Stdio).
pub async fn serve_sass_mcp(vfs: MemoryVfs) {
    let service = SassLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
