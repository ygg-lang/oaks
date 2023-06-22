use crate::lsp::MatlabLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for MATLAB semantics (Stdio).
pub async fn serve_matlab_mcp(vfs: MemoryVfs) {
    let service = MatlabLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
