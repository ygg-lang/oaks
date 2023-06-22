use crate::lsp::OCamlLanguageService;
use oak_mcp::OakMcpService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for OCaml semantics (Stdio).
pub async fn serve_ocaml_mcp(vfs: MemoryVfs) {
    let service = OCamlLanguageService::new(vfs);
    let server = service.into_mcp_server();

    server.run().await.unwrap();
}

/// One-click Axum integration for OCaml MCP.
#[cfg(feature = "axum")]
pub async fn serve_ocaml_mcp_axum(vfs: MemoryVfs) {
    let service = OCamlLanguageService::new(vfs);
    let app = service.into_mcp_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3063").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
