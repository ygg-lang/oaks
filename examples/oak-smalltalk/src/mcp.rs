use crate::lsp::SmalltalkLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Smalltalk semantics (Stdio).
pub async fn serve_smalltalk_mcp(vfs: MemoryVfs) {
    let service = SmalltalkLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
