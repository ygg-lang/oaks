use crate::lsp::NoteLanguageService;
use oak_vfs::MemoryVfs;

/// Start an MCP server for Notedown semantics (Stdio).
pub async fn serve_note_mcp(vfs: MemoryVfs) {
    let service = NoteLanguageService::new(vfs);
    let server = oak_mcp::McpServer::new(service);

    server.run().await.unwrap();
}
