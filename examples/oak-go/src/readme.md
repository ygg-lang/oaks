# 🛠️ Go Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-go`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a Go file:

```rust
use oak_go::{GoParser, SourceText, GoLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        package main
        
        import "fmt"
        
        func main() {
            fmt.Println("Hello from Oak Go!")
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = GoLanguage::new();
    let parser = GoParser::new(&config);

    // 3. Execute parsing
    let result = parser.parse(&source);

    // 4. Handle results
    if result.is_success() {
        println!("Parsing successful! AST node count: {}", result.node_count());
    } else {
        eprintln!("Errors found during parsing.");
    }
}
```

## 🔍 Core API Usage

### 1. Syntax Tree Traversal
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Go constructs like package declarations, struct methods, or channel operations.

### 2. Incremental Parsing
No need to re-parse the entire file when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-go` provides rich error contexts specifically tailored for Go developers:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Go source text into a stream of tokens, handling keywords, operators, and literals with support for Go's unique syntax rules.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle Go's expression precedence and structural declarations.
- **AST**: A strongly-typed syntax abstraction layer designed for building high-performance Go analysis tools and IDEs.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various Go edge cases and language versions.
