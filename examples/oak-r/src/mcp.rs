use crate::lsp::RLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_r_mcp(vfs: MemoryVfs) {
    let service = RLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
