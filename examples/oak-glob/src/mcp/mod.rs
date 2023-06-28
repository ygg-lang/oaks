use oak_mcp::McpService;

/// MCP service for glob pattern language.
pub struct GlobMcpService;

impl McpService for GlobMcpService {
    type Language = crate::language::GlobLanguage;

    fn language(&self) -> Self::Language {
        Self::Language::default()
    }
}

/// Serves the glob pattern MCP service.
pub fn serve_glob_mcp() {
    use oak_mcp::serve;
    let service = GlobMcpService;
    serve(service);
}
