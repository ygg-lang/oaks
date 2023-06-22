use crate::lsp::HlslLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for HLSL semantics (Stdio).
pub async fn serve_hlsl_mcp(vfs: MemoryVfs) {
    let service = HlslLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
