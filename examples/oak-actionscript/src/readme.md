# 🛠️ ActionScript Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-actionscript`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing an ActionScript class featuring metadata and E4X:

```rust
use oak_actionscript::{Parser, SourceText};

fn main() {
    // 1. Prepare source code
    let code = r#"
        package com.example {
            [Bindable]
            public class User {
                public var name:String;
                
                public function toXML():XML {
                    return <user name={name}/>;
                }
            }
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let parser = Parser::new();

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract ActionScript-specific constructs like metadata tags (`[Bindable]`), E4X XML literals, or complex package/class hierarchies.

### 2. Incremental Parsing
No need to re-parse the entire source file when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-actionscript` provides rich error contexts specifically tailored for AS3 developers, handling legacy Flex/Flash syntax quirks:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes ActionScript source text into a stream of tokens, including support for metadata brackets and E4X XML tokens.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle complex AS3 expression precedence and E4X syntax integration.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance AS3 analysis tools and migration engines.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various AS3/Flex edge cases.
