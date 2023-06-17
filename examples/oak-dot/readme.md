# Oak DOT Language Parser

## Overview

`Oak of dot` is a powerful and efficient parser for the DOT graph description language, built using the `oak` parser combinator library. It provides a robust solution for parsing DOT syntax, enabling various applications such as static analysis of graph definitions, visualization tools, and automated graph generation.

## Features

- **Comprehensive DOT Grammar**: Supports all standard DOT graph, subgraph, node, and edge definitions.
- **High Performance**: Leverages `oak`'s optimized parsing techniques for speed.
- **Abstract Syntax Tree (AST)**: Generates a detailed and easy-to-navigate AST representing the DOT structure.
- **Error Handling**: Provides meaningful error messages for better debugging of malformed DOT files.
- **Extensible**: Easily extendable to support custom DOT extensions or dialects.

## Quick Start

To use `Oak of dot` in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
Oak of dot = "0.1.0" # Replace with the latest version
oak = "0.1.0" # Replace with the latest version
```

## Parsing Examples

Here's a simple example demonstrating how to parse DOT language content:

```rust
use pex_dot::dot_parser;

fn main() {
    let input = r#"
digraph G {
    A -> B;
    B -> C;
    C -> A;
}
"#;
    match dot_parser::parse(input) {
        Ok(ast) => {
            println!("Successfully parsed DOT:\n{:#?}", ast);
        }
        Err(err) => {
            eprintln!("Failed to parse DOT: {}", err);
        }
    }
}
```

## Advanced Features

### Customizing the Parser

The `oak` library allows for flexible customization of the parser. You can modify the grammar rules or add new ones to suit your specific needs, such as supporting experimental DOT features. Refer to the `oak` documentation for more details on parser customization.

### Error Recovery

`Oak of dot` can be extended with error recovery mechanisms to handle malformed DOT files gracefully, allowing for partial parsing and better resilience in real-world scenarios.

## AST Structure

The generated AST for DOT provides a hierarchical representation of the graph elements. For instance, a node definition might result in an AST structure similar to this:

```rust
// Simplified AST representation for:
// A -> B;
pex_dot::ast::Node::Edge {
    source: "A".to_string(),
    target: "B".to_string(),
    attributes: vec![],
}
```

## Performance

`Oak of dot` is designed for performance. Benchmarks show efficient parsing of large DOT files. Optimizations include memoization, efficient backtracking, and direct AST construction.

## Integration

`Oak of dot` can be integrated into various tools and applications:

- **Graph Visualization Tools**: Parse DOT files for rendering.
- **Graph Analysis Tools**: Analyze graph structures for properties and patterns.
- **Code Generation**: Generate DOT files from other data structures.

## Examples

Explore the `examples` directory within the `oak-dot` project for more usage examples and demonstrations of specific DOT parsing features.

## Contributing

Contributions to `Oak of dot` are welcome! If you find a bug or have a feature request, please open an issue on the GitHub repository. For major changes, please open a discussion first.