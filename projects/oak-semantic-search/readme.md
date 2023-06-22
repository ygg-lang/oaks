# Oak Semantic Search

[![Crates.io](https://img.shields.io/crates/v/oak-semantic-search.svg)](https://crates.io/crates/oak-semantic-search)
[![Documentation](https://docs.rs/oak-semantic-search/badge.svg)](https://docs.rs/oak-semantic-search)

Advanced AI-powered semantic search for source code, leveraging AST-aware chunking and vector embeddings.

## 🎯 Overview

Oak Semantic Search goes beyond traditional keyword search by understanding the structure and meaning of your code. It uses `oak-core` to intelligently chunk source code into meaningful units (like functions, classes, and documentation) and indexes them using state-of-the-art embedding models and vector databases.

## ✨ Features

- **AST-Aware Chunking**: Intelligently splits code based on logical boundaries (Definitions, Statements, etc.) rather than simple line counts.
- **Embedding Integration**: Built-in support for `fastembed` to generate high-quality vector representations of code.
- **Vector DB Support**: Designed to work with `vectordb` (LanceDB) for efficient similarity search.
- **Contextual Search**: Find code by describing its functionality in natural language.
- **Role-Based Indexing**: Prioritizes definitions and documentation for better search relevance.

## 🚀 Quick Start

Basic usage of the `SemanticSearcher`:

```rust
use oak_semantic_search::SemanticSearcher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let searcher = SemanticSearcher::new("./my_vector_db").await?;
    
    // Indexing code is handled via the index_code method
    // searcher.index_code::<MyLanguage>(&root, source, "my_project_table").await?;
    
    // Search using natural language
    let results = searcher.search("how to handle user authentication", 5).await?;
    for result in results {
        println!("Found: {}", result);
    }
    
    Ok(())
}
```

## 📋 Examples

### Intelligent Code Chunking

The library uses a `ChunkCollector` to extract meaningful pieces of code:

```rust
// Internally, it identifies nodes with roles like:
// - UniversalElementRole::Definition
// - UniversalElementRole::Statement
// - UniversalElementRole::Documentation
```

## 🔧 Advanced Features

### Custom Embedding Models

Oak Semantic Search leverages `fastembed`, allowing you to choose from various pre-trained models optimized for code or general text.

### Integration with MCP

The library implements the `SemanticSearch` trait, making it compatible with the Model Context Protocol (MCP) for AI agent integration.

## 🏗️ Integration

- **Oak MCP**: Powers the semantic search tool in AI-assisted coding environments.
- **Documentation Portals**: Enhances documentation with "search by meaning" capabilities.
- **Code Discovery**: Helps developers find relevant code patterns in large monorepos.

## 📊 Performance

- **Fast Indexing**: Concurrent embedding generation for high throughput.
- **Scalable Search**: Vector-based retrieval remains fast even with millions of code chunks.
- **Efficient Storage**: Optimized vector storage with minimal disk footprint.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oak Semantic Search** - Understanding the meaning behind the code 🚀
