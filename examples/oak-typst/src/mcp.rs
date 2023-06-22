#[cfg(feature = "io-std")]
use crate::lsp::TypstLanguageService;
#[cfg(feature = "io-std")]
use oak_vfs::MemoryVfs;

/// 启动 Typst MCP 服务
#[cfg(feature = "io-std")]
pub async fn serve_typst_mcp(vfs: MemoryVfs) {
    let service = TypstLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
