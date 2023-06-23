# 🛠️ R Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-r`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing an R script:

```rust
use oak_r::{RParser, SourceText, RLanguage};

fn main() {
    // 1. Prepare R source code
    let code = r#"
        # Calculate summary statistics
        calculate_stats <- function(data) {
          mean_val <- mean(data)
          sd_val <- sd(data)
          return(list(mean = mean_val, sd = sd_val))
        }

        dataset <- c(10, 20, 30, 40, 50)
        results <- calculate_stats(dataset)
        print(results)
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = RLanguage::new();
    let parser = RParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract R constructs like function definitions, vectorized operations, or formula notations.

### 2. Incremental Parsing
No need to re-parse the entire script when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-r` provides rich error contexts specifically tailored for R developers, handling complex scenarios like unmatched brackets or malformed formula syntax:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes R source text into a stream of tokens, handling keywords, operators, literals, and R's unique identifier rules.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle R's expression precedence, lazy evaluation, and hierarchical structures.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance R analysis tools, IDEs, and data processing engines.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various R standards and edge cases.
