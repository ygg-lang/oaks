use crate::lsp::YamlLanguageService;
use oak_mcp::McpServer;
use oak_vfs::MemoryVfs;

/// Start an MCP server for YAML semantics (Stdio).
pub async fn serve_yaml_mcp(vfs: MemoryVfs) {
    let service = YamlLanguageService::new(vfs);
    let server = McpServer::new(service);

    server.run().await.unwrap();
}
