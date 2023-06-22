use crate::lsp::RustLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Rust semantics (Stdio).
pub async fn serve_rust_mcp(vfs: MemoryVfs) {
    let service = RustLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
