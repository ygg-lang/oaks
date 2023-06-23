#![doc = include_str!("readme.md")]
use crate::lsp::LuaLanguageService;
use oak_vfs::MemoryVfs;

/// 为 Lua 语义启动 MCP 服务器 (Stdio)。
#[cfg(feature = "mcp")]
pub async fn serve_lua_mcp(vfs: MemoryVfs) {
    let service = LuaLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    let reader = tokio::io::BufReader::new(tokio::io::stdin());
    let writer = tokio::io::BufWriter::new(tokio::io::stdout());
    server.run(reader, writer).await.unwrap()
}
