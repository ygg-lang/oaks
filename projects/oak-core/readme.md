# Oak Core

[![Crates.io](https://img.shields.io/crates/v/oak-core.svg)](https://crates.io/crates/oak-core)
[![Documentation](https://docs.rs/oak-core/badge.svg)](https://docs.rs/oak-core)

The foundational parser framework providing core primitives for building robust, incremental parsers in Rust.

## 🎯 Overview

`oak-core` is the heart of the Oak ecosystem, offering a comprehensive set of primitives that form the building blocks for language parsers. It provides a language-agnostic architecture for building high-performance lexers and parsers with built-in support for IDE features.

## ✨ Features

- **Language-Agnostic Design**: Define your language's tokens and elements using traits.
- **Zero-copy Lexing**: Efficiently tokenize source text without unnecessary allocations.
- **Incremental Parsing**: Built-in support for incremental re-parsing using specialized caching.
- **Green/Red Trees**: Persistent syntax tree structures inspired by Roslyn, enabling efficient immutability and easy traversal.
- **Error Recovery**: Graceful handling of malformed input with integrated diagnostics and "panic mode" recovery.
- **Pratt Parsing**: Built-in support for operator precedence parsing.
- **Source Mapping**: Accurate mapping between byte offsets and line/column positions.

## 🚀 Quick Start

To use Oak Core, you first define your language by implementing the `Language` trait.

```rust
#![feature(new_range_api)]
use oak_core::{Language, TokenType, ElementType, UniversalTokenRole, UniversalElementRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MyToken { Identifier, Whitespace, End }

impl TokenType for MyToken {
    const END_OF_STREAM: Self = MyToken::End;
    type Role = UniversalTokenRole;
    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MyElement { Root, Identifier }

impl ElementType for MyElement {
    type Role = UniversalElementRole;
    fn role(&self) -> Self::Role { UniversalElementRole::None }
}

struct MyLanguage;

impl Language for MyLanguage {
    const NAME: &'static str = "my-language";
    type TokenType = MyToken;
    type ElementType = MyElement;
    type TypedRoot = ();
}
```

## 📋 Core Components

- **`GreenNode`**: An immutable, pointer-free, and position-independent representation of the AST. Perfect for caching and sharing.
- **`RedNode`**: A thin wrapper around `GreenNode` that adds parent pointers and absolute position information for easy traversal.
- **`Lexer`**: A high-performance lexing engine that supports custom scanners for identifiers, numbers, and strings.
- **`Parser`**: A flexible parsing framework that supports both recursive descent and Pratt parsing.
- **`Visitor`**: A trait-based utility for walking the syntax tree and performing analysis.

## 🔧 Advanced Usage

### Incremental Parsing

Oak Core supports incremental parsing out of the box. When the source text changes, you can re-parse only the affected parts by providing an `IncrementalCache`.

```rust
use oak_core::tree::IncrementalCache;
use oak_core::builder::GreenBuilder;

let mut pool = GreenBuilder::new();
let cache = IncrementalCache::new();
let result = parser.parse_incremental(new_source, &cache);
```

### Pratt Parsing for Expressions

Handle complex operator precedence with ease using the `PrattParser`.

```rust
use oak_core::parser::PrattParser;

let mut pratt = PrattParser::new();
pratt.add_postfix(TokenKind::Inc, 10);
pratt.add_prefix(TokenKind::Minus, 9);
pratt.add_infix(TokenKind::Plus, 5, Associativity::Left);

let expr = pratt.parse(&mut parser_context)?;
```

## 📊 Performance

- **Optimized Memory Layout**: Green nodes use a compact, cache-friendly memory representation.
- **Minimal Allocations**: Lexers and parsers use internal pooling to minimize heap allocations.
- **Fast Traversal**: Red trees provide O(1) access to parent nodes and absolute offsets.

## 🔗 Integration

Oak Core is the foundational dependency for all other Oak projects, including:

- **Oak Highlight**: Uses core lexers for syntax highlighting.
- **Oak LSP**: Builds on core trees to provide language server features.
- **Oak Visualize**: Visualizes the green and red trees.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oak Core** - Building blocks for modern language tools 🚀
