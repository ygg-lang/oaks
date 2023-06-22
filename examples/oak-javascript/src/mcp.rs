use crate::lsp::JavaScriptLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for JavaScript semantics (Stdio).
pub async fn serve_javascript_mcp(vfs: MemoryVfs) {
    let service = JavaScriptLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
