# Oak Dhall Parser

## Overview

`Oak of Dhall` is a powerful and efficient parser for the Dhall programming language, built using the `oak` parser combinator library. It provides a robust solution for parsing Dhall syntax, enabling various applications such as static analysis of Dhall code, refactoring tools, and automated code generation.

## Features

- **Comprehensive Dhall Grammar**: Supports all standard Dhall constructs, including expressions, types, and imports.
- **High Performance**: Leverages `oak`'s optimized parsing techniques for speed.
- **Abstract Syntax Tree (AST)**: Generates a detailed and easy-to-navigate AST representing the Dhall code structure.
- **Error Handling**: Provides meaningful error messages for better debugging of malformed Dhall code.
- **Extensible**: Easily extendable to support custom Dhall extensions or dialects.

## 🚀 Quick Start

Here's a simple example demonstrating how to parse Dhall code:

```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_dhall::{DhallParser, DhallLanguage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = ParseSession::<DhallLanguage>::default();
    let parser = DhallParser::new();
    let source = SourceText::new(r#"
let greeting = "Hello, Dhall!"
in greeting
"#);
    
    let result = parser.parse(&source, &[], &mut session);
    if let Some(errors) = result.result.err() {
        println!("Parse errors found: {:?}", errors);
    } else {
        println!("Parsed Dhall successfully.");
    }
    Ok(())
}
```

## 📋 Parsing Examples

### Expression Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_dhall::{DhallParser, DhallLanguage};

let mut session = ParseSession::<DhallLanguage>::default();
let parser = DhallParser::new();
let source = SourceText::new(r#"{ x = 1, y = 2 }"#);

let result = parser.parse(&source, &[], &mut session);
println!("Parsed Dhall expression successfully.");
```

## Advanced Features

### Customizing the Parser

The `oak` library allows for flexible customization of the parser. You can modify the grammar rules or add new ones to suit your specific needs, such as supporting experimental Dhall features. Refer to the `oak` documentation for more details on parser customization.

### Error Recovery

`Oak of Dhall` can be extended with error recovery mechanisms to handle malformed Dhall code gracefully, allowing for partial parsing and better resilience in real-world scenarios.

## AST Structure

The generated AST for Dhall provides a hierarchical representation of the code elements. For instance, a let expression might result in an AST structure similar to this:

```rust
// Simplified AST representation for:
// let greeting = "Hello, Dhall!" in greeting
pex_dhall::ast::Node::LetExpression {
    bindings: vec![
        // ... binding details ...
    ],
    expression: // ... expression details ...
}
```

## Performance

`Oak of Dhall` is designed for performance. Benchmarks show efficient parsing of large Dhall codebases. Optimizations include memoization, efficient backtracking, and direct AST construction.

## Integration

`Oak of Dhall` can be integrated into various tools and applications:

- **Dhall IDEs**: Provide syntax highlighting, code completion, and refactoring capabilities.
- **Static Analyzers**: Identify potential bugs, code smells, and security vulnerabilities.
- **Code Transformers**: Automate code modifications and migrations.

## Examples

Explore the `examples` directory within the `oak-dhall` project for more usage examples and demonstrations of specific Dhall parsing features.

## Contributing

Contributions to `Oak of Dhall` are welcome! If you find a bug or have a feature request, please open an issue on the GitHub repository. For major changes, please open a discussion first.