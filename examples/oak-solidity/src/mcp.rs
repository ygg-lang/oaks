use crate::lsp::SolidityLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_solidity_mcp(vfs: MemoryVfs) {
    let service = SolidityLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
