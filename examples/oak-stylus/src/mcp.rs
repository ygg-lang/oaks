use crate::lsp::StylusLanguageService;
use oak_mcp::McpServer;
use oak_vfs::MemoryVfs;

pub async fn serve_stylus_mcp(vfs: MemoryVfs) {
    let service = StylusLanguageService::new(vfs);
    let server = McpServer::new(service);
    server.run().await.unwrap();
}
