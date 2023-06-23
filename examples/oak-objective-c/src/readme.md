# 🛠️ Objective-C Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-objective-c`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-objective-c = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing modern Objective-C with properties, categories, and blocks:

```rust
use oak_objective_c::{Parser, SourceText};

fn main() {
    // 1. Prepare source code
    let code = r#"
        #import <Foundation/Foundation.h>

        @interface User : NSObject
        @property (nonatomic, copy) NSString *name;
        - (void)greetWithCompletion:(void (^)(NSString *))completion;
        @end

        @implementation User
        - (void)greetWithCompletion:(void (^)(NSString *))completion {
            NSLog(@"Hello, %@", self.name);
            if (completion) {
                completion(@"Greeting finished");
            }
        }
        @end
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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Objective-C specific constructs like interface/implementation blocks, property declarations, method definitions, or block expressions.

### 2. Incremental Parsing
No need to re-parse a massive Objective-C file when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-objective-c` provides rich error contexts specifically tailored for Objective-C developers, handling complex scenarios like missing `@end` keywords, malformed message expressions, or invalid block syntax:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Objective-C source text into a stream of tokens, including support for `@`-prefixed keywords, message selectors, and C-style literals.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle Objective-C's unique message passing syntax, block structures, and mixed C/C++ integration.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance Objective-C analysis tools, static analyzers, and IDEs.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various Objective-C standards and edge cases.
