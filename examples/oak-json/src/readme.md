# 🛠️ JSON Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-json`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a JSON object:

```rust,no_run
use oak_json::{JsonParser, JsonLanguage};
use oak_core::{SourceText, Parser, parser::ParseSession};

fn main() {
    // 1. Prepare source code
    let code = r#"
        {
            "name": "Oak Framework",
            "version": "1.0.0",
            "features": ["Incremental Parsing", "High-Fidelity AST"]
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser with standard configuration
    let config = JsonLanguage::standard();
    let parser = JsonParser::new(&config);

    // 3. Execute parsing
    let mut session = ParseSession::new(1024);
    let result = parser.parse(&source, &[], &mut session);

    // 4. Handle results
    if result.result.is_ok() {
        println!("Parsing successful!");
    } else {
        eprintln!("Errors found during parsing.");
    }
}
```

## 🔍 Core API Usage

### 1. Syntax Tree Traversal
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract JSON values, keys, and structures.

### 2. Incremental Parsing
No need to re-parse massive JSON data when small changes occur:
```rust,no_run
use oak_json::{JsonParser, JsonLanguage};
use oak_core::{SourceText, Parser, parser::ParseSession};

# let config = JsonLanguage::standard();
# let parser = JsonParser::new(&config);
# let new_source = SourceText::new("{}");
// Assuming you have an old parse result and new source text 'new_source'
let mut session = ParseSession::new(1024);
let new_result = parser.parse(&new_source, &[], &mut session);
```

### 3. Diagnostics
`oak-json` provides precise error feedback for malformed JSON, such as missing colons, unmatched braces, or invalid escape sequences:
```rust,no_run
# use oak_json::{JsonParser, JsonLanguage};
# use oak_core::{SourceText, Parser, parser::ParseSession};
# let config = JsonLanguage::standard();
# let parser = JsonParser::new(&config);
# let source = SourceText::new("{");
# let mut session = ParseSession::new(1024);
# let result = parser.parse(&source, &[], &mut session);
for diag in result.diagnostics {
    println!("{:?}", diag);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes JSON source text into a stream of tokens, handling strings, numbers, booleans, and structural characters.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle JSON's recursive structure and various value types.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance JSON analysis, formatting, and validation tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of standard JSON and extended formats like JSON5.
