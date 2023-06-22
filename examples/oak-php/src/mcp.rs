use crate::lsp::PhpLanguageService;
use oak_vfs::MemoryVfs;

/// PHP MCP server implementation.
///
/// This module provides integration with the Model Context Protocol (MCP)
/// for PHP language services.
pub async fn serve_php_mcp(vfs: MemoryVfs) {
    let service = PhpLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
