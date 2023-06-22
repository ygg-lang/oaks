use crate::lsp::ProtobufLanguageService;
use oak_vfs::MemoryVfs;

pub async fn serve_protobuf_mcp(vfs: MemoryVfs) {
    let service = ProtobufLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);
    server.run().await.unwrap();
}
