use crate::lsp::RLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

pub async fn serve_r_mcp(vfs: MemoryVfs) {
    let service = RLanguageService::new(vfs);
    let server = service.into_mcp_server();
    server.run().await.unwrap();
}

#[cfg(feature = "axum")]
pub async fn serve_r_mcp_axum(vfs: MemoryVfs) {
    let service = RLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3090").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
