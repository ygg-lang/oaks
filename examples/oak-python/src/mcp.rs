use crate::lsp::PythonLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Python semantics (Stdio).
pub async fn serve_python_mcp(vfs: MemoryVfs) {
    let service = PythonLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
