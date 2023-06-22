# Oak Java Parser

[![Crates.io](https://img.shields.io/crates/v/oak-java.svg)](https://crates.io/crates/oak-java)
[![Documentation](https://docs.rs/oak-java/badge.svg)](https://docs.rs/oak-java)

High-performance incremental Java parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Java is a robust parser for Java, designed to handle complete Java syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Java Syntax**: Supports all Java features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_java::{JavaParser, JavaLanguage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = ParseSession::<JavaLanguage>::default();
    let parser = JavaParser::new();
    let source = SourceText::new(r#"
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
    "#);
    
    let result = parser.parse(&[], &mut session);
    println!("Parsed Java successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Method Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_java::{JavaParser, JavaLanguage};

let mut session = ParseSession::<JavaLanguage>::default();
let parser = JavaParser::new();
let source = SourceText::new(r#"
public class Calculator {
    public int add(int a, int b) {
        return a + b;
    }
    
    public static void main(String[] args) {
        Calculator calc = new Calculator();
        int result = calc.add(5, 3);
        System.out.println("Result: " + result);
    }
}
"#);

let result = parser.parse(&[], &mut session);
println!("Method parsed successfully.");
```

### Class Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_java::{JavaParser, JavaLanguage};

let mut session = ParseSession::<JavaLanguage>::default();
let parser = JavaParser::new();
let source = SourceText::new(r#"
public class Person {
    private String name;
    private int age;
    
    public Person(String name, int age) {
        this.name = name;
        this.age = age;
    }
    
    public String getName() {
        return name;
    }
    
    public int getAge() {
        return age;
    }
}
"#);

let result = parser.parse(&[], &mut session);
println!("Class parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_java::{JavaParser, JavaLanguage};

let mut session = ParseSession::<JavaLanguage>::default();
let parser = JavaParser::new();
let source = SourceText::new("int x = 42;");
let result = parser.parse(&[], &mut session);
```
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_java::{Parser, JavaLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
// Invalid Java code example
public class BrokenClass {
    public static void main(String[] args {
        System.out.println("Hello")
    // Missing closing braces
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

- **JavaProgram**: Root container for Java programs
- **Class**: Java class definitions
- **Method**: Java methods and constructors
- **Statement**: Various statement types including control flow
- **Expression**: Various expression types including operators
- **Type**: Java type system constructs

## 📊 Performance

- **Streaming**: Parse large Java files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Java integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Java AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Java code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Java program parsing
- Method and class analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-java) or open [issues](https://github.com/ygg-lang/oaks/issues).