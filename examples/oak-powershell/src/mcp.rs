use crate::lsp::PowerShellLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_powershell_mcp(vfs: MemoryVfs) {
    let service = PowerShellLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
