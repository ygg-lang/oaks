use crate::lsp::HaskellLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_haskell_mcp(vfs: MemoryVfs) {
    let service = HaskellLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
