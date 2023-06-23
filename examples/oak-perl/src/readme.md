# 🛠️ oak-perl Developer Guide

Welcome to the internal implementation of the Perl parser. This module provides the core logic for tokenizing and parsing Perl source code into a high-fidelity syntax tree.

## 📦 Core Components

- **Lexer**: Tokenizes Perl source code, handling complex sigils, regex literals, and heredocs with full fidelity.
- **Parser**: Implements the Perl grammar, producing a Green Tree that represents the concrete syntax.
- **AST**: Provides a type-safe Red Tree facade for easy traversal and analysis.
- **Language**: Defines the `PerlLanguage` configuration and integration with the Oak framework.

## 🚀 Usage Example

### Basic Parsing

```rust
use oak_perl::{PerlParser, SourceText, PerlLanguage};

fn parse_perl_code(code: &str) {
    let source = SourceText::new(code);
    let config = PerlLanguage::new();
    let parser = PerlParser::new(&config);
    let result = parser.parse(&source);

    if result.is_success() {
        let root = result.root();
        println!("AST Root: {:?}", root);
    }
}
```

### Incremental Parsing

```rust
use oak_perl::{PerlParser, SourceText, PerlLanguage};

fn incremental_update(old_code: &str, new_code: &str) {
    let config = PerlLanguage::new();
    let mut parser = PerlParser::new(&config);
    
    // Initial parse
    let initial_source = SourceText::new(old_code);
    let _ = parser.parse(&initial_source);
    
    // Incremental update
    let updated_source = SourceText::new(new_code);
    let result = parser.parse(&updated_source);
    
    if result.is_success() {
        println!("Incremental parse completed successfully.");
    }
}
```

## 🔍 Diagnostics

The parser provides detailed diagnostics for syntax errors, including error ranges and helpful messages.

```rust
let result = parser.parse(&source);
for diagnostic in result.diagnostics() {
    println!("Error at {:?}: {}", diagnostic.range(), diagnostic.message());
}
```
