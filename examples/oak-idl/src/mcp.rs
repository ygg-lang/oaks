use crate::lsp::IdlLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_idl_mcp(vfs: MemoryVfs) {
    let service = IdlLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
