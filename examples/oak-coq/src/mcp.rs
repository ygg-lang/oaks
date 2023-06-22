use crate::lsp::CoqLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Coq semantics (Stdio).
pub async fn serve_coq_mcp(vfs: MemoryVfs) {
    let service = CoqLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
