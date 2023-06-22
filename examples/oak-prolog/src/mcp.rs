use crate::lsp::PrologLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_prolog_mcp(vfs: MemoryVfs) {
    let service = PrologLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
