use crate::lsp::LeanLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Lean semantics (Stdio).
pub async fn serve_lean_mcp(_vfs: MemoryVfs) {
    let service = LeanLanguageService::new(crate::language::LeanLanguage::default());
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
