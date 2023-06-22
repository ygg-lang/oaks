# Oak Groovy Parser

[![Crates.io](https://img.shields.io/crates/v/oak-groovy.svg)](https://crates.io/crates/oak-groovy)
[![Documentation](https://docs.rs/oak-groovy/badge.svg)](https://docs.rs/oak-groovy)

High-performance incremental Groovy parser for the oak ecosystem with flexible configuration, optimized for build systems and dynamic language applications.

## 🎯 Overview

Oak Groovy is a robust parser for Apache Groovy, designed to handle complete Groovy syntax including modern language features and DSL capabilities. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for Groovy analysis and tooling.

## ✨ Features

- **Complete Groovy Syntax**: Supports all Groovy features including closures, builders, and dynamic typing
- **DSL Support**: Handles domain-specific languages and Groovy-specific constructs
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_groovy::{Parser, GroovyLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
def hello() {
    println "Hello, Groovy!"
}
hello()
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Groovy script successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Class Parsing
```rust
use oak_groovy::{Parser, GroovyLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
class Person {
    String name
    int age
    
    Person(String name, int age) {
        this.name = name
        this.age = age
    }
    
    def greet() {
        println "Hello, I'm ${name} and I'm ${age} years old."
    }
}

def person = new Person("Alice", 30)
person.greet()
"#);

let result = parser.parse(&source);
println!("Parsed Groovy class successfully.");
```

### Closure Parsing
```rust
use oak_groovy::{Parser, GroovyLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
def numbers = [1, 2, 3, 4, 5]
def doubled = numbers.collect { it * 2 }
println "Original: ${numbers}"
println "Doubled: ${doubled}"

// Method reference
def strings = ["apple", "banana", "cherry"]
def lengths = strings.collect(String::length)
println "Lengths: ${lengths}"
"#);

let result = parser.parse(&source);
println!("Parsed Groovy closures successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_groovy::{Parser, GroovyLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("def hello() { println 'Hello' }");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_groovy::{Parser, GroovyLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
def brokenMethod() {
    def x = "string"
    x++  // Type mismatch error
    if (x == 5 {  // Missing closing parenthesis
        println "This won't compile"
    }
}
"#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Class**: Groovy class definitions
- **Method**: Method and function definitions
- **Closure**: Groovy closure expressions
- **Statement**: Assignment, control flow, and expression statements
- **Expression**: Method calls, literals, and operators

## 📊 Performance

- **Streaming**: Parse large Groovy files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak-groovy integrates seamlessly with:

- **Build Systems**: Gradle and other build tool analysis
- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Groovy AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Groovy code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Groovy class parsing
- Closure and DSL analysis
- Build script processing
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-groovy) or open [issues](https://github.com/ygg-lang/oaks/issues).