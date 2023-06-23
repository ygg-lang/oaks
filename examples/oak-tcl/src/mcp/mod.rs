#![doc = include_str!("readme.md")]
use crate::lsp::TclLanguageService;
use oak_vfs::MemoryVfs;

/// 为 Tcl 语义启动 MCP 服务器 (Stdio)。
pub async fn serve_tcl_mcp(vfs: MemoryVfs) {
    let service = TclLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    let reader = tokio::io::BufReader::new(tokio::io::stdin());
    let writer = tokio::io::BufWriter::new(tokio::io::stdout());
    server.run(reader, writer).await.unwrap()
}
