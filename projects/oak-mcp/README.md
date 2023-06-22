# Oak MCP

[![Crates.io](https://img.shields.io/crates/v/oak-mcp.svg)](https://crates.io/crates/oak-mcp)
[![Documentation](https://docs.rs/oak-mcp/badge.svg)](https://docs.rs/oak-mcp)

Oak MCP is a Model Context Protocol (MCP) server implementation for the Oak framework, enabling AI models to interact directly with Oak's deep semantic analysis.

## 🎯 Overview

Oak MCP bridges the gap between AI agents (like Claude 3.5/4) and your source code. Unlike traditional LSP-based tools that return verbose JSON, Oak MCP is optimized for AI context windows, providing structured "code skeletons" and intelligent semantic search capabilities.

## ✨ Features

- **Compact Format**: Deeply optimized for AI context windows. Returns "code skeletons" using Oak's `RedTree` architecture to minimize token usage.
- **Fuzzy Semantics**: Integrated with `oak-semantic-search` for natural language intent-based code retrieval.
- **Local Embedding**: Uses `fastembed` for privacy-first, offline vector indexing.
- **LSP Bridge**: Supports standard features like Hover, Definition, References, and Symbols.
- **Cross-File Context**: Discovers logical relationships between files that go beyond simple naming conventions.

## 🚀 Quick Start

Add `oak-mcp` to your `Cargo.toml`:

```toml
[dependencies]
oak-mcp = "0.0.1"
```

### Running the MCP Server

```rust
use oak_mcp::OakMcpService;
use oak_rust::RustService; // Example language service

#[tokio::main]
async fn main() {
    let service = RustService::new();
    let server = service.into_mcp_server()
        .with_searcher(semantic_searcher);
    
    server.run().await.unwrap();
}
```

## 📋 Examples

### Integration with Claude Desktop

Add the following to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "oak": {
      "command": "path/to/your/oak-mcp-binary",
      "args": []
    }
  }
}
```

## 🏗️ Advanced Capabilities

### 1. Structured Summarization
Instead of sending thousands of lines of code, Oak MCP sends a hierarchical summary of symbols and their relationships, allowing the AI to understand the big picture instantly.

### 2. Intent-Based Search
"How does this project handle error retries?" — Oak MCP uses vector embeddings to find relevant code chunks even if the word "retry" isn't explicitly in the function name.

## 📊 Performance

- **Token Efficiency**: Up to 80% reduction in token usage compared to raw file contents.
- **Low Latency**: Local indexing and fast retrieval for real-time AI interactions.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oak MCP** - Deep code understanding for AI agents 🚀
