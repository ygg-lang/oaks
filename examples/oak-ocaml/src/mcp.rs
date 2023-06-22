use crate::lsp::OCamlLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for OCaml semantics (Stdio).
pub async fn serve_ocaml_mcp(vfs: MemoryVfs) {
    let service = OCamlLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
