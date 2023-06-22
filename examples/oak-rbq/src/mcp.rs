use crate::lsp::RbqLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_rbq_mcp(vfs: MemoryVfs) {
    let service = RbqLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
