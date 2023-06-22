use crate::lsp::LuaLanguageService;
use oak_vfs::MemoryVfs;

#[cfg(feature = "mcp-stdio")]
pub async fn serve_lua_mcp(vfs: MemoryVfs) {
    let service = LuaLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
