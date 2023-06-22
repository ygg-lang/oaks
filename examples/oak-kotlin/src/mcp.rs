use crate::lsp::KotlinLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Kotlin semantics (Stdio).
pub async fn serve_kotlin_mcp(_vfs: MemoryVfs) {
    let service = KotlinLanguageService::new(crate::language::KotlinLanguage::default());
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
