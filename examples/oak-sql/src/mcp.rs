use crate::lsp::SqlLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_sql_mcp(vfs: MemoryVfs) {
    let service = SqlLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
