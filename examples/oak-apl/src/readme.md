# 🛠️ APL Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-apl`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing an APL expression:

```rust
use oak_apl::{AplParser, AplLanguage};
use oak_core::{SourceText, Parser, parser::ParseSession};

fn main() {
    // 1. Prepare source code
    let code = r#"
        A ← 1 2 3
        B ← 4 5 6
        C ← A + B
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = AplLanguage::default();
    let parser = AplParser::new(&config);

    // 3. Execute parsing
    let mut session = ParseSession::new(1024);
    let result = parser.parse(&source, &[], &mut session);
    
    if result.result.is_ok() {
        println!("APL code parsed successfully!");
    }
}
```

### Incremental Parsing Example

Oak parsers excel at incremental updates. Here's how to re-parse code after a change:

```rust
use oak_apl::{AplParser, AplLanguage};
use oak_core::{SourceText, Parser, parser::ParseSession};

fn main() {
    let config = AplLanguage::default();
    let parser = AplParser::new(&config);

    // 1. Initial parse
    let old_code = "A ← 1 2 3";
    let old_source = SourceText::new(old_code);
    let mut session = ParseSession::new(1024);
    let old_result = parser.parse(&old_source, &[], &mut session);

    // 2. Modified code (incremental change)
    let new_code = "A ← 1 2 3 4";
    let new_source = SourceText::new(new_code);
    
    // In a real scenario, you would provide the TextEdit that describes the change.
    // Here we perform a full re-parse for simplicity, but using the same session
    // allows the framework to reuse parts of the previous parse tree.
    let new_result = parser.parse(&new_source, &[], &mut session);
    
    assert!(new_result.result.is_ok());
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes APL source text into a stream of tokens, handling unique APL symbols (←, ⍴, ⍳), numeric literals (with ¯ for negative), and strings.
- **Parser**: Syntax analyzer designed for APL's right-to-left evaluation and array-oriented structure.
- **AST**: A strongly-typed syntax abstraction layer designed for building high-performance APL analysis tools and IDEs.
