use crate::lsp::NimLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Nim semantics (Stdio).
pub async fn serve_nim_mcp(vfs: MemoryVfs) {
    let service = NimLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
