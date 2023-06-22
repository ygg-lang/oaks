use crate::lsp::RegexLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Regex semantics (Stdio).
pub async fn serve_regex_mcp(vfs: MemoryVfs) {
    let service = RegexLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
