use crate::lsp::NixLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Nix semantics (Stdio).
pub async fn serve_nix_mcp(vfs: MemoryVfs) {
    let service = NixLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
