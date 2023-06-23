# 🛠️ Java Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-java`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a Java class with modern features like Records and Annotations:

```rust
use oak_java::{JavaParser, SourceText, JavaLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        package com.example;
        
        import java.util.List;
        
        /**
         * Represents a user in the system.
         */
        @Entity
        public record User(String name, int age) {
            public void greet() {
                System.out.println("Hello, " + name);
            }
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = JavaLanguage::new();
    let parser = JavaParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Java-specific constructs like class/record definitions, annotations, or complex method bodies.

### 2. Incremental Parsing
No need to re-parse the entire source file when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-java` provides rich error contexts specifically tailored for Java developers, handling complex scenarios like missing semicolons or malformed annotations:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Java source text into a stream of tokens, handling keywords, operators, literals, and support for Unicode identifiers.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle Java's expression precedence, structural declarations, and modern language features.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance Java analysis tools, IDEs, and refactoring engines.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various Java versions and edge cases.
