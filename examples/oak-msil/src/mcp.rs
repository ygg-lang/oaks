use crate::lsp::MsilLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for MSIL semantics (Stdio).
pub async fn serve_msil_mcp(vfs: MemoryVfs) {
    let service = MsilLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
