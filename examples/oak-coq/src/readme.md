# 🛠️ Coq Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-coq`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a simple Coq theorem:

```rust
use oak_coq::{CoqParser, SourceText, CoqLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        Theorem add_0_n : forall n : nat, 0 + n = n.
        Proof.
          intros n. reflexivity.
        Qed.
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = CoqLanguage::new();
    let parser = CoqParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Coq constructs like theorem declarations, proof tactics, or Gallina terms.

### 2. Incremental Parsing
No need to re-parse the entire script when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-coq` provides rich error contexts specifically tailored for Coq developers:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Coq source text into a stream of tokens, handling keywords, operators, identifiers, and complex notation delimiters.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle Coq's extensible syntax, vernacular commands, and tactic blocks.
- **AST**: A strongly-typed syntax abstraction layer designed for building high-performance Coq analysis tools and interactive proof environments.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various Coq notation edge cases and formal development structures.
