use crate::lsp::JavaLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Java semantics (Stdio).
pub async fn serve_java_mcp(vfs: MemoryVfs) {
    let service = JavaLanguageService::new(vfs, crate::language::JavaLanguage::default());
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
