use crate::lsp::ProtobufLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

pub async fn serve_protobuf_mcp(vfs: MemoryVfs) {
    let service = ProtobufLanguageService::new(vfs);
    let server = service.into_mcp_server();
    server.run().await.unwrap();
}

#[cfg(feature = "axum")]
pub async fn serve_protobuf_mcp_axum(vfs: MemoryVfs) {
    let service = ProtobufLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3027").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
