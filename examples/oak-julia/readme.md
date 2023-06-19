# Oak Julia Parser

[![Crates.io](https://img.shields.io/crates/v/oak-julia.svg)](https://crates.io/crates/oak-julia)
[![Documentation](https://docs.rs/oak-julia/badge.svg)](https://docs.rs/oak-julia)

High-performance incremental Julia parser for the oak ecosystem with flexible configuration, optimized for scientific computing and data analysis.

## 🎯 Overview

Oak Julia is a robust parser for Julia, designed to handle complete Julia syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for scientific computing and data analysis.

## ✨ Features

- **Complete Julia Syntax**: Supports all Julia features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_julia::{Parser, JuliaLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
function fibonacci(n)
    if n <= 1
        return n
    else
        return fibonacci(n-1) + fibonacci(n-2)
    end
end

println(fibonacci(10))
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Julia successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_julia::{Parser, JuliaLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
function greet(name::String)
    println("Hello, $name!")
end
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

### Type Parsing
```rust
use oak_julia::{Parser, JuliaLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
struct Person
    name::String
    age::Int
    
    Person(name::String) = new(name, 0)
end
"#);

let result = parser.parse(&source);
println!("Type parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_julia::{Parser, JuliaLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("x = [1, 2, 3, 4, 5]");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_julia::{Parser, JuliaLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
function broken_function(x)
    if x > 0
        println("Positive")
    // Missing 'end' keyword
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

- **JuliaProgram**: Root container for Julia programs
- **Function**: Julia functions and methods
- **Struct**: Julia struct definitions
- **Module**: Julia module definitions
- **Expression**: Various expression types including operators
- **Statement**: Control flow, loops, conditionals

## 📊 Performance

- **Streaming**: Parse large Julia files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Julia integrates seamlessly with:

- **Scientific Computing**: Julia code analysis and optimization
- **Data Analysis**: Processing and transforming Julia code
- **IDE Support**: Language server protocol compatibility
- **Code Generation**: Generating code from Julia AST
- **Documentation**: Generating documentation from Julia code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Julia program parsing
- Function and type analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-julia) or open [issues](https://github.com/ygg-lang/oaks/issues).