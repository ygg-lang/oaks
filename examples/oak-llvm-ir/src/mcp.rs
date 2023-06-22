use crate::lsp::LlirLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for LLVM IR semantics (Stdio).
pub async fn serve_llir_mcp(vfs: MemoryVfs) {
    let service = LlirLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
