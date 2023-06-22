# Oak PureScript Parser

## Overview

`Oak of PureScript` is a powerful and efficient parser for the PureScript programming language, built using the `oak` parser combinator library. It provides a robust solution for parsing PureScript syntax, enabling various applications such as static analysis of PureScript code, refactoring tools, and automated code generation.

## Features

- **Comprehensive PureScript Grammar**: Supports all standard PureScript constructs, including modules, functions, types, and expressions.
- **High Performance**: Leverages `oak`'s optimized parsing techniques for speed.
- **Abstract Syntax Tree (AST)**: Generates a detailed and easy-to-navigate AST representing the PureScript code structure.
- **Error Handling**: Provides meaningful error messages for better debugging of malformed PureScript code.
- **Extensible**: Easily extendable to support custom PureScript extensions or dialects.

## Quick Start

To use `Oak of PureScript` in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
Oak of PureScript = "0.1.0" # Replace with the latest version
oak = "0.1.0" # Replace with the latest version
```

## Parsing Examples

Here's a simple example demonstrating how to parse PureScript code:

```rust
use pex_purescript::purescript_parser;

fn main() {
    let input = r#"
module HelloWorld where

import Prelude

greet :: String -> String
greet name = "Hello, " <> name <> "!"

main :: Effect Unit
main = log (greet "PureScript")
"#;
    match purescript_parser::parse(input) {
        Ok(ast) => {
            println!("Successfully parsed PureScript code:\n{:#?}", ast);
        }
        Err(err) => {
            eprintln!("Failed to parse PureScript code: {}", err);
        }
    }
}
```

## Advanced Features

### Customizing the Parser

The `oak` library allows for flexible customization of the parser. You can modify the grammar rules or add new ones to suit your specific needs, such as supporting experimental PureScript features. Refer to the `oak` documentation for more details on parser customization.

### Error Recovery

`Oak of PureScript` can be extended with error recovery mechanisms to handle malformed PureScript code gracefully, allowing for partial parsing and better resilience in real-world scenarios.

## AST Structure

The generated AST for PureScript provides a hierarchical representation of the code elements. For instance, a function definition might result in an AST structure similar to this:

```rust
// Simplified AST representation for:
// greet :: String -> String
// greet name = "Hello, " <> name <> "!"
pex_purescript::ast::Node::FunctionDefinition {
    name: "greet".to_string(),
    type_signature: Some(TypeSignature {
        input: "String".to_string(),
        output: "String".to_string()
    }),
    implementation: // ... implementation details ...
}
```

## Performance

`Oak of PureScript` is designed for performance. Benchmarks show efficient parsing of large PureScript codebases. Optimizations include memoization, efficient backtracking, and direct AST construction.

## Integration

`Oak of PureScript` can be integrated into various tools and applications:

- **PureScript IDEs**: Provide syntax highlighting, code completion, and refactoring capabilities.
- **Static Analyzers**: Identify potential bugs, code smells, and security vulnerabilities.
- **Code Transformers**: Automate code modifications and migrations.

## Examples

Explore the `examples` directory within the `oak-purescript` project for more usage examples and demonstrations of specific PureScript parsing features.

## Contributing

Contributions to `Oak of PureScript` are welcome! If you find a bug or have a feature request, please open an issue on the GitHub repository. For major changes, please open a discussion first.