#![cfg(feature = "mcp")]

use crate::lsp::PrologLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

pub async fn serve_prolog_mcp(vfs: MemoryVfs) {
    let service = PrologLanguageService::new(vfs);
    let server = service.into_mcp_server();
    server.run().await.unwrap();
}

#[cfg(feature = "axum")]
pub async fn serve_prolog_mcp_axum(vfs: MemoryVfs) {
    let service = PrologLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3026").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
