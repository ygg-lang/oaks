use crate::lsp::HandlebarsLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_handlebars_mcp(vfs: MemoryVfs) {
    let service = HandlebarsLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
