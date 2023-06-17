# Oak Erlang Parser

## Overview

`Oak of erlang` is a powerful and efficient parser for the Erlang programming language, built using the `oak` parser combinator library. It provides a robust solution for parsing Erlang syntax, enabling various applications such as static analysis of Erlang code, refactoring tools, and automated code generation.

## Features

- **Comprehensive Erlang Grammar**: Supports all standard Erlang constructs, including modules, functions, patterns, and expressions.
- **High Performance**: Leverages `oak`'s optimized parsing techniques for speed.
- **Abstract Syntax Tree (AST)**: Generates a detailed and easy-to-navigate AST representing the Erlang code structure.
- **Error Handling**: Provides meaningful error messages for better debugging of malformed Erlang code.
- **Extensible**: Easily extendable to support custom Erlang extensions or dialects.

## Quick Start

To use `Oak of erlang` in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
Oak of erlang = "0.1.0" # Replace with the latest version
oak = "0.1.0" # Replace with the latest version
```

## Parsing Examples

Here's a simple example demonstrating how to parse Erlang code:

```rust
use pex_erlang::erlang_parser;

fn main() {
    let input = r#"
-module(my_module).
-export([my_function/0]).

my_function() ->
    io:format("Hello, Erlang!~n").
"#;
    match erlang_parser::parse(input) {
        Ok(ast) => {
            println!("Successfully parsed Erlang code:\n{:#?}", ast);
        }
        Err(err) => {
            eprintln!("Failed to parse Erlang code: {}", err);
        }
    }
}
```

## Advanced Features

### Customizing the Parser

The `oak` library allows for flexible customization of the parser. You can modify the grammar rules or add new ones to suit your specific needs, such as supporting experimental Erlang features. Refer to the `oak` documentation for more details on parser customization.

### Error Recovery

`Oak of erlang` can be extended with error recovery mechanisms to handle malformed Erlang code gracefully, allowing for partial parsing and better resilience in real-world scenarios.

## AST Structure

The generated AST for Erlang provides a hierarchical representation of the code elements. For instance, a function definition might result in an AST structure similar to this:

```rust
// Simplified AST representation for:
// my_function() -> io:format("Hello, Erlang!~n").
pex_erlang::ast::Node::FunctionDefinition {
    name: "my_function".to_string(),
    arity: 0,
    clauses: vec![
        // ... clause details ...
    ],
}
```

## Performance

`Oak of erlang` is designed for performance. Benchmarks show efficient parsing of large Erlang codebases. Optimizations include memoization, efficient backtracking, and direct AST construction.

## Integration

`Oak of erlang` can be integrated into various tools and applications:

- **Erlang IDEs**: Provide syntax highlighting, code completion, and refactoring capabilities.
- **Static Analyzers**: Identify potential bugs, code smells, and security vulnerabilities.
- **Code Transformers**: Automate code modifications and migrations.

## Examples

Explore the `examples` directory within the `oak-erlang` project for more usage examples and demonstrations of specific Erlang parsing features.

## Contributing

Contributions to `Oak of erlang` are welcome! If you find a bug or have a feature request, please open an issue on the GitHub repository. For major changes, please open a discussion first.