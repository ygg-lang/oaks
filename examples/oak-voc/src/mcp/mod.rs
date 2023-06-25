//! MCP support for VOC.
use crate::lsp::VocLanguageService;
use oak_vfs::MemoryVfs;

/// Serves the VOC Model Context Protocol (MCP) server.
#[cfg(feature = "mcp")]
pub async fn serve_voc_mcp(vfs: MemoryVfs) {
    let service = VocLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    let reader = tokio::io::BufReader::new(tokio::io::stdin());
    let writer = tokio::io::BufWriter::new(tokio::io::stdout());
    server.run(reader, writer).await.unwrap()
}
