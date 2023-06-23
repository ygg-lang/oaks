#![doc = include_str!("readme.md")]
use crate::lsp::HaskellLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_haskell_mcp(vfs: MemoryVfs) {
    let service = HaskellLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    let reader = tokio::io::BufReader::new(tokio::io::stdin());
    let writer = tokio::io::BufWriter::new(tokio::io::stdout());
    server.run(reader, writer).await.unwrap()
}
