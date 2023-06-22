use crate::lsp::TclLanguageService;
use oak_vfs::MemoryVfs;

/// 为 Tcl 语义启动 MCP 服务器 (Stdio)。
pub async fn serve_tcl_mcp(vfs: MemoryVfs) {
    let service = TclLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
