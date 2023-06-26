# � ActionScript Parser User Guide

ActionScript support for the Oak language framework.

This guide helps you integrate `oak-actionscript` into your project and perform common parsing tasks efficiently.

## � Quick Start

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

## 🔍 Core Functionality

### 1. Syntax Tree Traversal
After a successful parse, use the built-in visitor pattern or manually traverse the Green/Red Tree to extract ActionScript-specific constructs like metadata tags (`[Bindable]`), E4X XML literals, or complex package/class hierarchies.

### 2. Incremental Parsing
Optimize performance by only re-parsing changed sections:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics & Error Recovery
`oak-actionscript` provides detailed error contexts tailored for AS3, including support for legacy Flex/Flash syntax quirks:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🛠️ Performance & Reliability

- **High-Fidelity AST**: Retains all trivia (whitespace and comments), making it ideal for code formatting and refactoring tools.
- **Fault Tolerance**: Automatically recovers from syntax errors to provide as much information as possible from the rest of the file.
- **Memory Efficiency**: Leverages immutable data structures (Green Trees) for low-overhead tree management.
