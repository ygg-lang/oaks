use crate::lsp::RubyLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Ruby semantics (Stdio).
pub async fn serve_ruby_mcp(vfs: MemoryVfs) {
    let service = RubyLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
