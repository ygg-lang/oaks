use crate::lsp::PerlLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Perl semantics (Stdio).
pub async fn serve_perl_mcp(vfs: MemoryVfs) {
    let service = PerlLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
