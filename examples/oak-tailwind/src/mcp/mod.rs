#![doc = include_str!("readme.md")]
use crate::lsp::TailwindLanguageService;
use oak_vfs::MemoryVfs;

/// 为 Tailwind 语义启动 MCP 服务器 (Stdio)。
#[cfg(feature = "mcp")]
pub async fn serve_tailwind_mcp(vfs: MemoryVfs) {
    let service = TailwindLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    let reader = tokio::io::BufReader::new(tokio::io::stdin());
    let writer = tokio::io::BufWriter::new(tokio::io::stdout());
    server.run(reader, writer).await.unwrap()
}
