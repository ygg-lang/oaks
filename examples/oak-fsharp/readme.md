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
use oak_fsharp::{Parser, FSharpLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
let helloWorld = 
    printfn "Hello, F#!"

helloWorld
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed F# successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_fsharp::{Parser, FSharpLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
let rec factorial n =
    if n <= 1 then 1
    else n * factorial (n - 1)

let main() =
    let result = factorial 5
    printfn "Factorial of 5 is %d" result

main()
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

### Module Parsing
```rust
use oak_fsharp::{Parser, FSharpLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
module Calculator =
    let add x y = x + y
    let subtract x y = x - y
    let multiply x y = x * y
    let divide x y = x / y

open Calculator

let result = add 10 5
printfn "10 + 5 = %d" result
"#);

let result = parser.parse(&source);
println!("Module parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_fsharp::{Parser, FSharpLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("let x = 42");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_fsharp::{Parser, FSharpLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
// Invalid F# code example
let brokenFunction =
    printfn "Hello"
    // Missing proper function definition
"#);

let result = parser.parse(&source);
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