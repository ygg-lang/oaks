use oak_vfs::MemoryVfs;

#[cfg(feature = "io-std")]
// use crate::lsp::MarkdownLanguageService;

/// Start an MCP server for Markdown semantics (Stdio).
#[cfg(feature = "io-std")]
pub async fn serve_markdown_mcp(vfs: MemoryVfs) {
    let service = MarkdownLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}

/// Start an MCP server for Markdown semantics (Stdio).
#[cfg(not(feature = "io-std"))]
pub async fn serve_markdown_mcp(_vfs: MemoryVfs) {
    panic!("MCP server requires io-std feature");
}
