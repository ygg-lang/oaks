# 🛠️ Tcl Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-tcl`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-tcl = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing Tcl scripts, supporting commands, variables, and complex command substitutions:

```rust
use oak_tcl::{TclParser, SourceText, TclLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        set name "Oak"
        proc greet {msg} {
            puts "$msg, $::name!"
        }
        greet "Hello"
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = TclLanguage::new();
    let parser = TclParser::new(&config);

    // 3. Execute parsing
    let result = parser.parse(&source);

    // 4. Handle results
    if result.is_success() {
        println!("Parsing successful! AST node count: {}", result.node_count());
    } else {
        eprintln!("Errors found during parsing.");
        for diag in result.diagnostics() {
            println!("[{}:{}] {}", diag.line, diag.column, diag.message);
        }
    }
}
```

## 🔍 Core API Usage

### 1. Syntax Tree Traversal
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Tcl specific constructs like commands, arguments, variable references, and scripts within braces.

### 2. Incremental Parsing
Tcl scripts are often used in embedded systems or as configuration. `oak-tcl` supports sub-millisecond incremental updates:
```rust
// Re-parse only the modified section
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Error Recovery
The parser is designed for industrial-grade fault tolerance, recovering gracefully from unclosed braces or quotes to provide continuous feedback in IDEs.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Tcl source text, supporting Tcl's unique "everything is a string" philosophy, command substitutions, and variable expansions.
- **Parser**: A high-performance recursive descent parser that handles Tcl's command-based structure and nested scripts.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/readme.md](tests/readme.md) for details on our snapshot-based testing.
