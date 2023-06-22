use crate::lsp::JavadocLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Javadoc semantics (Stdio).
#[cfg(feature = "mcp-stdio")]
pub async fn serve_javadoc_mcp(vfs: MemoryVfs) {
    let service = JavadocLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
