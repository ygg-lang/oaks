# Oak Code Folding

[![Crates.io](https://img.shields.io/crates/v/oak-folding.svg)](https://crates.io/crates/oak-folding)
[![Documentation](https://docs.rs/oak-folding/badge.svg)](https://docs.rs/oak-folding)

A lightweight and efficient code folding engine for the Oak ecosystem, designed to provide accurate folding ranges based on AST analysis.

## 🎯 Overview

Oak Code Folding is a specialized library for calculating folding ranges in source code. It leverages `oak-core`'s red-green tree structure to identify logical blocks that can be collapsed in an editor, such as function bodies, class definitions, large comments, and import blocks.

## ✨ Features

- **AST-Based Folding**: Precise folding ranges derived from the syntax tree rather than indentation.
- **Folding Kinds**: Support for different types of folding ranges (Comments, Imports, Regions).
- **Language Agnostic**: Extensible trait-based system that works with any language implemented in Oak.
- **Zero-Cost Abstractions**: Minimal overhead for range calculations.
- **LSP Compatible**: Designed to integrate seamlessly with the Language Server Protocol.

## 🚀 Quick Start

Basic example of implementing a folding provider:

```rust
use oak_folding::{FoldingProvider, FoldingRange, FoldingRangeKind};
use oak_core::{language::Language, tree::RedNode};

struct MyFoldingProvider;

impl<L: Language> FoldingProvider<L> for MyFoldingProvider {
    fn folding_ranges(&self, root: &RedNode<L::ElementType>) -> Vec<FoldingRange> {
        let mut ranges = Vec::new();
        // Traverse the tree and identify nodes that should be foldable
        // ...
        ranges
    }
}
```

## 📋 Examples

### Implementing Folding for a Language

```rust
use oak_folding::{FoldingProvider, FoldingRange, FoldingRangeKind};
use oak_core::tree::RedNode;
use my_language::MyLanguage;

pub struct MyLanguageFoldingProvider;

impl FoldingProvider<MyLanguage> for MyLanguageFoldingProvider {
    fn folding_ranges(&self, root: &RedNode<MyLanguage::ElementType>) -> Vec<FoldingRange> {
        let mut ranges = Vec::new();
        
        // Example: Fold function bodies
        for node in root.descendants() {
            if node.kind().is_function() {
                ranges.push(FoldingRange {
                    range: node.range(),
                    kind: None,
                });
            }
        }
        
        ranges
    }
}
```

## 🏗️ Supported Folding Kinds

The engine supports several standard folding kinds:

- `FoldingRangeKind::Comment`: For multi-line comment blocks.
- `FoldingRangeKind::Imports`: For grouped import/include statements.
- `FoldingRangeKind::Region`: For custom user-defined regions (e.g., `#region` in C#).

## 📊 Performance

- **Fast Traversal**: Optimized tree traversal using `oak-core` visitors.
- **Minimal Allocations**: Uses efficient collection handling for range results.
- **Incremental Updates**: Designed to work with Oak's incremental parsing system.

## 🔗 Integration

Oak Code Folding is a core component used by:

- **Oak LSP**: Provides `textDocument/foldingRange` support.
- **Oak IDE Plugins**: Powers code folding in various editors.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oak Code Folding** - Accurate code folding for every language 🚀
