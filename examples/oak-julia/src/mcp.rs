use crate::lsp::JuliaLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Julia semantics (Stdio).
pub async fn serve_julia_mcp(vfs: MemoryVfs) {
    let service = JuliaLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
