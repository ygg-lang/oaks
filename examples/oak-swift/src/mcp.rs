use crate::lsp::SwiftLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_swift_mcp(vfs: MemoryVfs) {
    let service = SwiftLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
