# Oak Kotlin Parser

[![Crates.io](https://img.shields.io/crates/v/oak-kotlin.svg)](https://crates.io/crates/oak-kotlin)
[![Documentation](https://docs.rs/oak-kotlin/badge.svg)](https://docs.rs/oak-kotlin)

High-performance incremental Kotlin parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Kotlin is a robust parser for Kotlin, designed to handle complete Kotlin syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Kotlin Syntax**: Supports all Kotlin features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_kotlin::{Parser, KotlinLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
fun main() {
    println("Hello, Kotlin!")
}
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Kotlin successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_kotlin::{Parser, KotlinLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
fun calculateArea(radius: Double): Double {
    return Math.PI * radius * radius
}

fun main() {
    val area = calculateArea(5.0)
    println("Area: $area")
}
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

### Class Parsing
```rust
use oak_kotlin::{Parser, KotlinLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
class Person(val name: String, var age: Int) {
    fun greet() {
        println("Hello, I'm $name and I'm $age years old")
    }
    
    fun haveBirthday() {
        age++
        println("Happy birthday! Now I'm $age")
    }
}

fun main() {
    val person = Person("Alice", 25)
    person.greet()
    person.haveBirthday()
}
"#);

let result = parser.parse(&source);
println!("Class parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_kotlin::{Parser, KotlinLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("val x = 42");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_kotlin::{Parser, KotlinLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
// Invalid Kotlin code example
fun brokenFunction {
    println("Hello")
    // Missing function parameters and return type
}
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

- **KotlinProgram**: Root container for Kotlin programs
- **Class**: Kotlin class definitions
- **Function**: Kotlin functions and methods
- **Statement**: Various statement types including control flow
- **Expression**: Various expression types including operators
- **Type**: Kotlin type system constructs

## 📊 Performance

- **Streaming**: Parse large Kotlin files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Kotlin integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Kotlin AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Kotlin code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Kotlin program parsing
- Function and class analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-kotlin) or open [issues](https://github.com/ygg-lang/oaks/issues).