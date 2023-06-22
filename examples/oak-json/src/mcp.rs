use crate::lsp::JsonLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for JSON semantics (Stdio).
#[cfg(feature = "mcp-stdio")]
pub async fn serve_json_mcp(vfs: MemoryVfs) {
    let service = JsonLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
