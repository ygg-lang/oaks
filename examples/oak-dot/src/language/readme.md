# DOT Language Definition

This module contains the metadata and configuration options for the DOT language within the Oak framework.

## ⚙️ Configuration

The `DotLanguage` struct defines how the parser and lexer should behave to accommodate various graph description styles:

```rust
pub struct DotLanguage {}
```

Currently, `DotLanguage` serves as a marker struct for DOT support. Future versions may include configuration for:
- **`strict`**: Enforce Graphviz strict mode (no duplicate edges).
- **`attribute_validation`**: Validate attribute names against standard Graphviz properties.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords (`graph`, `digraph`, `subgraph`), operators, and identifiers.
- **`ElementType`**: Defined in the `parser` module, representing graphs, nodes, edges, and attribute lists.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the DOT graph description.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of DOT's graph-based structure.
