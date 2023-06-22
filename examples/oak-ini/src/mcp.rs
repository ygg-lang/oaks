use crate::lsp::IniLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_ini_mcp(vfs: MemoryVfs) {
    let service = IniLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
