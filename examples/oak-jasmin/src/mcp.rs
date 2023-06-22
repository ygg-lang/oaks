use crate::lsp::JasminLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_jasmin_mcp(vfs: MemoryVfs) {
    let service = JasminLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
