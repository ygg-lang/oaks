# 🛠️ Ruby Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-ruby`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-ruby = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing Ruby code, including support for classes, methods, and blocks:

```rust
use oak_ruby::{RubyParser, SourceText, RubyLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        class Greeter
          def initialize(name)
            @name = name
          end

          def greet
            puts "Hello, #{@name}!"
          end
        end

        greeter = Greeter.new("Oak")
        greeter.greet
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = RubyLanguage::new();
    let parser = RubyParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Ruby specific constructs like class definitions, method bodies, block expressions, and complex literals.

### 2. Incremental Parsing
Ruby codebases (especially Rails apps) can be massive. `oak-ruby` supports sub-millisecond incremental updates:
```rust
// Re-parse only the modified section
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. DSL and Metaprogramming
The parser is designed to handle Ruby's flexible nature, providing high-fidelity trees for code that heavily uses DSLs or dynamic method calls.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Ruby source text, including support for complex string interpolations, heredocs, and various percent literals.
- **Parser**: A high-performance syntax analyzer that handles Ruby's expression-heavy syntax and complex grammar rules.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/readme.md](tests/readme.md) for handling of various Ruby versions and edge cases.
