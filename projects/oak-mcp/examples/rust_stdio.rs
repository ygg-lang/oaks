use oak_rust::RustLanguageService;
use oak_vfs::MemoryVfs;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // Set up tracing for logging to stderr
    tracing_subscriber::fmt().with_writer(std::io::stderr).init();

    // Create a simple VFS and a Rust service
    let vfs = MemoryVfs::new();

    // Example: Add a virtual file
    vfs.write_file("example.rs", "fn main() { println!(\"Hello, MCP!\") }".to_string());

    let service = RustLanguageService::new(vfs);

    // Convert the service into an MCP server
    let server = oak_mcp::McpServer::new(service);

    println!("Starting Rust MCP server on Stdio...");
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    server.run(tokio::io::BufReader::new(stdin), stdout).await.map_err(|e| tokio::io::Error::new(tokio::io::ErrorKind::Other, e))
}
