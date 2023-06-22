#![cfg(feature = "mcp")]

use crate::lsp::PowerShellLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

pub async fn serve_powershell_mcp(vfs: MemoryVfs) {
    let service = PowerShellLanguageService::new(vfs);
    let server = service.into_mcp_server();
    server.run().await.unwrap();
}

#[cfg(feature = "axum")]
pub async fn serve_powershell_mcp_axum(vfs: MemoryVfs) {
    let service = PowerShellLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3028").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
