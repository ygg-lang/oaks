# Oak R Parser

[![Crates.io](https://img.shields.io/crates/v/oak-r.svg)](https://crates.io/crates/oak-r)
[![Documentation](https://docs.rs/oak-r/badge.svg)](https://docs.rs/oak-r)

High-performance incremental R parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak R is a robust parser for R, designed to handle complete R syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete R Syntax**: Supports all R features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_r::{RParser, RLanguage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = ParseSession::<RLanguage>::default();
    let parser = RParser::new();
    let source = SourceText::new(r#"
print("Hello, World!")
x <- c(1, 2, 3, 4, 5)
mean(x)
    "#);
    
    let result = parser.parse(&[], &mut session);
    println!("Parsed R successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_r::{RParser, RLanguage};

let mut session = ParseSession::<RLanguage>::default();
let parser = RParser::new();
let source = SourceText::new(r#"
add <- function(a, b) {
  return(a + b)
}
"#);

let result = parser.parse(&[], &mut session);
println!("Function parsed successfully.");
```

### Data Structure Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_r::{RParser, RLanguage};

let mut session = ParseSession::<RLanguage>::default();
let parser = RParser::new();
let source = SourceText::new(r#"
data <- data.frame(
  name = c("Alice", "Bob", "Charlie"),
  age = c(25, 30, 35),
  city = c("New York", "London", "Paris")
)
"#);

let result = parser.parse(&[], &mut session);
println!("Data structure parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_r::{RParser, RLanguage};

let mut session = ParseSession::<RLanguage>::default();
let parser = RParser::new();
let source = SourceText::new("x <- c(1, 2, 3)");
let result = parser.parse(&[], &mut session);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_r::{RParser, RLanguage};

let mut session = ParseSession::<RLanguage>::default();
let parser = RParser::new();
let source = SourceText::new(r#"
# Invalid R code example
invalid_function(
"#);

let result = parser.parse(&[], &mut session);
if let Some(errors) = result.result.err() {
    println!("Parse errors found: {:?}", errors);
} else {
    println!("Parsed successfully.");
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **RProgram**: Root container for R programs
- **Function**: R functions and methods
- **Expression**: R expressions and operations
- **DataFrame**: R data structures
- **Vector**: R vector operations

## 📊 Performance

- **Streaming**: Parse large R files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak R integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from R AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from R code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete R program parsing
- Function and data analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-r) or open [issues](https://github.com/ygg-lang/oaks/issues).