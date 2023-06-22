use crate::lsp::ObjectiveCLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Objective-C semantics (Stdio).
pub async fn serve_objective_c_mcp(vfs: MemoryVfs) {
    let service = ObjectiveCLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
