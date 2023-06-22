# Oak C Parser

[![Crates.io](https://img.shields.io/crates/v/oak-c.svg)](https://crates.io/crates/oak-c)
[![Documentation](https://docs.rs/oak-c/badge.svg)](https://docs.rs/oak-c)

High-performance incremental C parser for the oak ecosystem with flexible configuration, optimized for code analysis and compilation.

## 🎯 Overview

Oak C is a robust parser for C, designed to handle complete C syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete C Syntax**: Supports all C features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_c::{CParser, CLanguage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = ParseSession::<CLanguage>::default();
    let parser = CParser::new();
    let source = SourceText::new(r#"
        #include <stdio.h>

        int main() {
            printf("Hello, World!\n");
            return 0;
        }
    "#);
    
    let result = parser.parse(&[], &mut session);
    println!("Parsed C program successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_c::{CParser, CLanguage};

let mut session = ParseSession::<CLanguage>::default();
let parser = CParser::new();
let source = SourceText::new(r#"
    int add(int a, int b) {
        return a + b;
    }
"#);

let result = parser.parse(&[], &mut session);
println!("Parsed C function successfully.");
```

### Struct Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_c::{CParser, CLanguage};

let mut session = ParseSession::<CLanguage>::default();
let parser = CParser::new();
let source = SourceText::new(r#"
    struct Point {
        int x;
        int y;
    };
"#);

let result = parser.parse(&[], &mut session);
println!("Parsed C struct successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_c::{CParser, CLanguage};

let mut session = ParseSession::<CLanguage>::default();
let parser = CParser::new();
let source = SourceText::new("int main() { return 0; }");
let result = parser.parse(&[], &mut session);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_c::{CParser, CLanguage};

let mut session = ParseSession::<CLanguage>::default();
let parser = CParser::new();
let source = SourceText::new(r#"
    int main() {
        printf("Hello, World!\n")
        return 0;
    }
"#);

let result = parser.parse(&[], &mut session);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Program**: Root container for C programs
- **FunctionDefinition**: Function declarations and definitions
- **StructDefinition**: Structure definitions
- **Declaration**: Variable and type declarations
- **Statement**: Control flow, expressions, blocks

## 📊 Performance

- **Streaming**: Parse large C files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak C integrates seamlessly with:

- **Compilers**: Front-end for C compilers
- **Static Analysis Tools**: Code quality and security analysis
- **IDE Support**: Language server protocol compatibility
- **Code Generation**: Generating code from AST

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete C program parsing
- Function and struct analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-c) or open [issues](https://github.com/ygg-lang/oaks/issues).