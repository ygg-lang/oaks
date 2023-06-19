# Oak JASMIN Parser

[![Crates.io](https://img.shields.io/crates/v/oak-jasmin.svg)](https://crates.io/crates/oak-jasmin)
[![Documentation](https://docs.rs/oak-jasmin/badge.svg)](https://docs.rs/oak-jasmin)

High-performance incremental JASMIN parser for the oak ecosystem with flexible configuration, optimized for JVM bytecode assembly and analysis.

## 🎯 Overview

Oak JASMIN is a robust parser for JASMIN, designed to handle complete Java assembler syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for JVM bytecode processing and analysis.

## ✨ Features

- **Complete JASMIN Syntax**: Supports all JASMIN features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_jasmin::{Parser, JasminLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
.class public HelloWorld
.super java/lang/Object

.method public static main([Ljava/lang/String;)V
    .limit stack 2
    .limit locals 1
    
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "Hello, World!"
    invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
    
    return
.end method
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed JASMIN successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Class Parsing
```rust
use oak_jasmin::{Parser, JasminLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
.class public Calculator
.super java/lang/Object

.method public static add(II)I
    .limit stack 2
    .limit locals 2
    
    iload_0
    iload_1
    iadd
    ireturn
.end method

.method public static multiply(II)I
    .limit stack 2
    .limit locals 2
    
    iload_0
    iload_1
    imul
    ireturn
.end method
"#);

let result = parser.parse(&source);
println!("Class parsed successfully.");
```

### Method Parsing
```rust
use oak_jasmin::{Parser, JasminLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
.method public factorial(I)I
    .limit stack 2
    .limit locals 2
    
    iload_1
    iconst_1
    if_icmple Lbase
    
    iload_1
    iload_1
    iconst_1
    isub
    aload_0
    invokevirtual Calculator/factorial(I)I
    imul
    ireturn
    
Lbase:
    iconst_1
    ireturn
.end method
"#);

let result = parser.parse(&source);
println!("Method parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_jasmin::{Parser, JasminLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("getstatic java/lang/System/out Ljava/io/PrintStream;");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_jasmin::{Parser, JasminLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
.class Broken
.method public bad_method()V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "Hello"  -- Missing semicolon
    invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
    return
.end method
.end class
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

- **JasminProgram**: Root container for JASMIN programs
- **ClassFile**: JASMIN class definitions
- **Method**: Method definitions with bytecode instructions
- **Field**: Field definitions with types and access modifiers
- **Instruction**: Individual bytecode instructions
- **ConstantPool**: Constant pool entries
- **Attribute**: Class and method attributes

## 📊 Performance

- **Streaming**: Parse large JASMIN files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak JASMIN integrates seamlessly with:

- **JVM Analysis**: Build JVM bytecode analysis tools
- **Compiler Development**: Generate JVM bytecode from high-level languages
- **Reverse Engineering**: Support reverse engineering workflows
- **IDE Support**: Language server protocol compatibility for JASMIN
- **Educational Tools**: Build JVM bytecode learning environments

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete JASMIN class parsing
- Bytecode instruction analysis
- Constant pool management
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-jasmin) or open [issues](https://github.com/ygg-lang/oaks/issues).