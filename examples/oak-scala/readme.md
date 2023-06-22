# Oak Scala Parser

[![Crates.io](https://img.shields.io/crates/v/oak-scala.svg)](https://crates.io/crates/oak-scala)
[![Documentation](https://docs.rs/oak-scala/badge.svg)](https://docs.rs/oak-scala)

High-performance incremental Scala parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Scala is a robust parser for Scala, designed to handle complete Scala syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Scala Syntax**: Supports all Scala features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_scala::{ScalaParser, ScalaLanguage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = ParseSession::<ScalaLanguage>::default();
    let parser = ScalaParser::new();
    let source = SourceText::new(r#"
object HelloWorld {
    def main(args: Array[String]): Unit = {
        println("Hello, World!")
    }
}
    "#);
    
    let result = parser.parse(&source, &mut session);
    println!("Parsed Scala successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Object Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_scala::{ScalaParser, ScalaLanguage};

let mut session = ParseSession::<ScalaLanguage>::default();
let parser = ScalaParser::new();
let source = SourceText::new(r#"
object Calculator {
    def add(a: Int, b: Int): Int = a + b
    def subtract(a: Int, b: Int): Int = a - b
    def multiply(a: Int, b: Int): Int = a * b
    
    def main(args: Array[String]): Unit = {
        println(s"2 + 3 = ${add(2, 3)}")
        println(s"5 - 2 = ${subtract(5, 2)}")
        println(s"4 * 6 = ${multiply(4, 6)}")
    }
}
"#);

let result = parser.parse(&source, &mut session);
println!("Object parsed successfully.");
```

### Class Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_scala::{ScalaParser, ScalaLanguage};

let mut session = ParseSession::<ScalaLanguage>::default();
let parser = ScalaParser::new();
let source = SourceText::new(r#"
class Person(val name: String, var age: Int) {
    def greet(): Unit = {
        println(s"Hello, I'm $name and I'm $age years old")
    }
    
    def haveBirthday(): Unit = {
        age += 1
        println(s"Happy birthday! Now I'm $age")
    }
}

object Main {
    def main(args: Array[String]): Unit = {
        val person = new Person("Alice", 25)
        person.greet()
        person.haveBirthday()
    }
}
"#);

let result = parser.parse(&source, &mut session);
println!("Class parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_scala::{ScalaParser, ScalaLanguage};

let mut session = ParseSession::<ScalaLanguage>::default();
let parser = ScalaParser::new();
let source = SourceText::new("val x = 42");
let result = parser.parse(&source, &mut session);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_scala::{ScalaParser, ScalaLanguage};

let mut session = ParseSession::<ScalaLanguage>::default();
let parser = ScalaParser::new();
let source = SourceText::new(r#"
object Broken {
    def main(args: Array[String]): Unit = {
        val x = 
        // Missing value
    }
}
"#);

let result = parser.parse(&source, &mut session);
if let Some(errors) = result.result.err() {
    println!("Parse errors found: {:?}", errors);
} else {
    println!("Parsed successfully.");
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **ScalaProgram**: Root container for Scala programs
- **Object**: Scala object definitions
- **Class**: Scala class definitions
- **Method**: Scala methods and functions
- **Statement**: Various statement types including control flow
- **Expression**: Various expression types including operators
- **Type**: Scala type system constructs

## 📊 Performance

- **Streaming**: Parse large Scala files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Scala integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Scala AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Scala code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Scala program parsing
- Object and class analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-scala) or open [issues](https://github.com/ygg-lang/oaks/issues).