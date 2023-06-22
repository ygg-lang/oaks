use oak_mcp::OakMcpService;
use oak_rust::RustLanguageService;
use oak_vfs::MemoryVfs;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // Set up tracing for logging to stderr
    tracing_subscriber::fmt().with_writer(std::io::stderr).init();

    // Create a simple VFS and a Rust service
    let vfs = MemoryVfs::new();

    // Example: Add a virtual file
    vfs.write_file("example.rs", "fn main() { println!(\"Hello, MCP!\"); }".to_string());

    let service = RustLanguageService::new(vfs);

    // Convert the service into an MCP server
    let server = service.into_mcp_server();

    println!("Starting Rust MCP server on Stdio...");
    server.run().await
}
