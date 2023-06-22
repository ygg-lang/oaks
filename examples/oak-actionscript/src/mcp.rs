use crate::lsp::ActionScriptLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for ActionScript semantics (Stdio).
pub async fn serve_actionscript_mcp(vfs: MemoryVfs) {
    let service = ActionScriptLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
