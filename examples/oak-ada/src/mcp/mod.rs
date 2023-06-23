#![doc = include_str!("readme.md")]
use crate::lsp::AdaLanguageService;
use oak_vfs::MemoryVfs;

/// 为 Ada 语言启动 MCP 服务
pub async fn serve_ada_mcp(vfs: MemoryVfs) {
    let service = AdaLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    let reader = tokio::io::BufReader::new(tokio::io::stdin());
    let writer = tokio::io::BufWriter::new(tokio::io::stdout());
    server.run(reader, writer).await.unwrap()
}
