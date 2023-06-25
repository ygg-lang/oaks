#![doc = include_str!("readme.md")]
use crate::lsp::TypeScriptLanguageService;
use oak_mcp::McpServer;
use oak_vfs::MemoryVfs;

/// Starts the TypeScript MCP service
pub async fn serve_typescript_mcp(vfs: MemoryVfs) {
    let service = TypeScriptLanguageService::new(vfs);
    let server = McpServer::new(service);

    let reader = tokio::io::BufReader::new(tokio::io::stdin());
    let writer = tokio::io::stdout();

    server.run(reader, writer).await.unwrap()
}
