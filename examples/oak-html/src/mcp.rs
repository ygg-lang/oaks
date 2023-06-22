use crate::lsp::HtmlLanguageService;
use oak_vfs::MemoryVfs;

#[cfg(feature = "mcp-stdio")]
pub async fn serve_html_mcp(vfs: MemoryVfs) {
    let service = HtmlLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
