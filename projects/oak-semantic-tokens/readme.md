# Oak Semantic Tokens

[![Crates.io](https://img.shields.io/crates/v/oak-semantic-tokens.svg)](https://crates.io/crates/oak-semantic-tokens)
[![Documentation](https://docs.rs/oak-semantic-tokens/badge.svg)](https://docs.rs/oak-semantic-tokens)

High-performance semantic highlighting engine for the Oak ecosystem, providing precise token classification for IDEs.

## 🎯 Overview

Oak Semantic Tokens provides the infrastructure for "Semantic Highlighting" — a technique that goes beyond simple regex-based syntax highlighting by using full AST analysis. It allows editors to distinguish between different types of identifiers (e.g., variables vs. functions vs. types) and apply context-aware colors.

## ✨ Features

- **LSP Compatible**: Implements the standard LSP Semantic Tokens format (delta-encoded).
- **High Performance**: Optimized for fast token generation during typing.
- **Context-Aware**: Uses Oak's red-green trees to accurately classify tokens based on their semantic role.
- **Language Agnostic**: Generic trait for implementing semantic highlighting for any language.

## 🚀 Quick Start

Basic usage of the `SemanticToken` structure:

```rust
use oak_semantic_tokens::SemanticToken;

let token = SemanticToken {
    delta_line: 0,
    delta_start: 5,
    length: 10,
    token_type: 1, // e.g., Function
    token_modifiers_bitmask: 0,
};
```

## 📋 Examples

### Implementing a Semantic Tokens Provider

```rust
use oak_semantic_tokens::{SemanticTokensProvider, SemanticToken};
use oak_core::tree::RedNode;
use my_language::MyLanguage;

struct MySemanticProvider;

impl SemanticTokensProvider<MyLanguage> for MySemanticProvider {
    fn semantic_tokens(&self, root: &RedNode<MyLanguage::ElementType>) -> Vec<SemanticToken> {
        let mut tokens = Vec::new();
        // Traverse the tree and generate tokens with correct types/modifiers
        tokens
    }
}
```

## 🏗️ Integration

Oak Semantic Tokens is used by:

- **Oak LSP**: Implements `textDocument/semanticTokens/full` and `range`.
- **Oak Highlight**: Can be used to enhance basic syntax highlighting with semantic data.
- **VS Code Extension**: Powers the advanced colorization in the official extension.

## 📊 Performance

- **Delta Encoding**: Efficiently transfers only changes to the editor.
- **Zero-Copy**: Designed to minimize data copying during token generation.
- **Incremental**: Works seamlessly with Oak's incremental re-parsing.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oak Semantic Tokens** - Precise semantic highlighting for every language 🚀
