#![doc = include_str!("readme.md")]
use crate::lsp::IniLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_ini_mcp(vfs: MemoryVfs) {
    let service = IniLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    let reader = tokio::io::BufReader::new(tokio::io::stdin());
    let writer = tokio::io::BufWriter::new(tokio::io::stdout());
    server.run(reader, writer).await.unwrap()
}
