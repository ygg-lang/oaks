# Oak F# Parser

[![Crates.io](https://img.shields.io/crates/v/oak-fsharp.svg)](https://crates.io/crates/oak-fsharp)
[![Documentation](https://docs.rs/oak-fsharp/badge.svg)](https://docs.rs/oak-fsharp)

High-performance incremental F# parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak F# is a robust parser for F#, designed to handle complete F# syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete F# Syntax**: Supports all F# features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_fsharp::{FSharpParser, FSharpLanguage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = ParseSession::<FSharpLanguage>::default();
    let parser = FSharpParser::new();
    let source = SourceText::new(r#"
let helloWorld = 
    printfn "Hello, F#!"

helloWorld
    "#);
    
    let result = parser.parse(&source, &[], &mut session);
    println!("Parsed F# successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_fsharp::{FSharpParser, FSharpLanguage};

let mut session = ParseSession::<FSharpLanguage>::default();
let parser = FSharpParser::new();
let source = SourceText::new(r#"
let add x y = x + y
let result = add 5 10
"#);

let result = parser.parse(&source, &[], &mut session);
println!("Function parsed successfully.");
```

### Type Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_fsharp::{FSharpParser, FSharpLanguage};

let mut session = ParseSession::<FSharpLanguage>::default();
let parser = FSharpParser::new();
let source = SourceText::new(r#"
type Person = {
    Name: string
    Age: int
}
"#);

let result = parser.parse(&source, &[], &mut session);
println!("Type parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_fsharp::{FSharpParser, FSharpLanguage};

let mut session = ParseSession::<FSharpLanguage>::default();
let parser = FSharpParser::new();
let source = SourceText::new("let x = 42");
let result = parser.parse(&source, &[], &mut session);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_fsharp::{FSharpParser, FSharpLanguage};

let mut session = ParseSession::<FSharpLanguage>::default();
let parser = FSharpParser::new();
let source = SourceText::new(r#"
let broken = 
    if x > 0 then
# Missing else
"#);

let result = parser.parse(&source, &[], &mut session);
if let Some(errors) = result.result.err() {
    println!("Parse errors found: {:?}", errors);
} else {
    println!("Parsed successfully.");
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **FSharpProgram**: Root container for F# programs
- **Module**: F# module definitions
- **Function**: F# functions and methods
- **Expression**: Various expression types including operators
- **Type**: F# type system constructs
- **Statement**: Various statement types

## 📊 Performance

- **Streaming**: Parse large F# files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak F# integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from F# AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from F# code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete F# program parsing
- Function and module analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-fsharp) or open [issues](https://github.com/ygg-lang/oaks/issues).