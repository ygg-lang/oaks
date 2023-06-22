use crate::lsp::ScalaLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Scala semantics (Stdio).
pub async fn serve_scala_mcp(vfs: MemoryVfs) {
    let service = ScalaLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
