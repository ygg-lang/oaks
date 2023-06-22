# Oak Hover

[![Crates.io](https://img.shields.io/crates/v/oak-hover.svg)](https://crates.io/crates/oak-hover)
[![Documentation](https://docs.rs/oak-hover/badge.svg)](https://docs.rs/oak-hover)

A flexible hover information provider for the Oak ecosystem, enabling rich documentation and type information display on mouse hover.

## 🎯 Overview

Oak Hover provides the infrastructure for displaying context-aware information when a user hovers over code elements. It defines standard structures for hover content (Markdown support) and a trait-based system for language-specific hover implementations.

## ✨ Features

- **Markdown Support**: Rich text formatting for hover tooltips.
- **Context Awareness**: Provides information based on the exact AST node under the cursor.
- **Range Highlighting**: Optional ability to highlight the specific code range the hover applies to.
- **LSP Ready**: Fully compatible with the Language Server Protocol `textDocument/hover` request.
- **Language Extensible**: Simple trait to implement for any Oak-supported language.

## 🚀 Quick Start

Basic usage of the `Hover` structure:

```rust
use oak_hover::Hover;
use core::range::Range;

let hover = Hover {
    contents: "### Function: `println!`\nPrints to the standard output.".to_string(),
    range: Some(0..10),
};
```

## 📋 Examples

### Implementing a Hover Provider

```rust
use oak_hover::{HoverProvider, Hover};
use oak_core::{language::Language, tree::RedNode};
use core::range::Range;

struct MyHoverProvider;

impl<L: Language> HoverProvider<L> for MyHoverProvider {
    fn hover(&self, root: &RedNode<L::ElementType>, range: Range<usize>) -> Option<Hover> {
        // Find the node at the given range and return relevant documentation
        let node = root.find_node_at_range(range)?;
        
        Some(Hover {
            contents: format!("Documentation for node: {:?}", node.kind()),
            range: Some(node.range()),
        })
    }
}
```

## 🔧 Advanced Features

### Rich Markdown Formatting

Oak Hover encourages the use of Markdown for beautiful presentation:

```rust
let hover = Hover {
    contents: r#"
# Standard Library `vec`
Creates a `Vec` containing the arguments.

## Example
```rust
let v = vec![1, 2, 3];
```
"#.to_string(),
    range: None,
};
```

## 🏗️ Integration

Oak Hover is designed to work with:

- **Oak LSP**: Maps `HoverProvider` results to LSP `Hover` responses.
- **Oak Documentation**: Can be used to generate tooltip-like documentation in static docs.
- **IDE Tooltips**: Powers the "Peek Definition" and documentation popups.

## 📊 Performance

- **Fast Node Lookup**: Efficiently identifies the relevant AST node for any given coordinate.
- **Lazy Content Generation**: Hover content is only generated when requested.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oak Hover** - Beautiful hover information for every language 🚀
