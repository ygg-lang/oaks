use crate::lsp::JasmLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_jasm_mcp(vfs: MemoryVfs) {
    let service = JasmLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
