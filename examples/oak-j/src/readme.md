# 🛠️ J Parser Developer Guide

J support for the Oak language framework.

This guide is designed to help you quickly get started with developing and integrating `oak-j`.

## 🚀 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing J source code:

```rust
use oak_j::{JParser, JLanguage};
use oak_core::{SourceText, Parser, parser::ParseSession};

fn main() {
    // 1. Prepare source code
    let code = "a =: 1 + 2";
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = JLanguage::default();
    let parser = JParser::new(&config);

    // 3. Execute parsing
    let mut session = ParseSession::new(1024);
    let result = parser.parse(&source, &[], &mut session);

    // 4. Handle result
    if result.result.is_ok() {
        println!("Parsed successfully!");
    }
}
```

## 🔍 Core API Usage

### 1. Syntax Tree Traversal
After successful parsing, you can use the built-in visitor pattern or manually traverse the Green/Red Tree.

### 2. Incremental Parsing
When the source code changes slightly, there's no need to re-parse the entire document:
```rust
use oak_j::{JParser, JLanguage};
use oak_core::{SourceText, Parser, parser::ParseSession};

// Assume we already have a parser instance
# let config = JLanguage::default();
# let parser = JParser::new(&config);
// Assume you have the old parse result and new source code
# let new_source = SourceText::new("a =: 2");
let mut session = ParseSession::new(1024);
// In a real scenario, the session will keep the old tree for incremental comparison
let new_result = parser.parse(&new_source, &[], &mut session);
```

### 3. Diagnostics
`oak-j` provides rich error context:

```rust
# use oak_j::{JParser, JLanguage};
# use oak_core::{SourceText, Parser, parser::ParseSession};
# let config = JLanguage::default();
# let parser = JParser::new(&config);
# let source = SourceText::new("a =:");
# let mut session = ParseSession::new(1024);
# let result = parser.parse(&source, &[], &mut session);
for diag in result.diagnostics {
    println!("{:?}", diag);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes J source text into a stream of tokens, handling keywords (case-insensitive), operators, and numeric literals.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle J's structural declarations and expression precedence.
- **AST**: A strongly-typed syntax abstraction layer designed for building high-performance J analysis tools and IDEs.

## 📚 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various J edge cases and language versions.
