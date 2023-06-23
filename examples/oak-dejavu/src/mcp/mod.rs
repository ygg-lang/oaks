#![doc = include_str!("readme.md")]
use crate::lsp::DejavuLanguageService;
use oak_vfs::MemoryVfs;

/// 启动 Dejavu MCP 服务
pub async fn serve_dejavu_mcp(vfs: MemoryVfs) {
    let service = DejavuLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    let reader = tokio::io::BufReader::new(tokio::io::stdin());
    let writer = tokio::io::BufWriter::new(tokio::io::stdout());
    server.run(reader, writer).await.unwrap()
}
