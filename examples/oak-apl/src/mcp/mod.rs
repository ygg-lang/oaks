#![doc = include_str!("readme.md")]
use crate::lsp::AplLanguageService;
use oak_vfs::MemoryVfs;

/// 为 APL 语言启动 MCP 服务
pub async fn serve_apl_mcp(vfs: MemoryVfs) {
    let service = AplLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    let reader = tokio::io::BufReader::new(tokio::io::stdin());
    let writer = tokio::io::BufWriter::new(tokio::io::stdout());
    server.run(reader, writer).await.unwrap()
}
