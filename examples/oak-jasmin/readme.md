# Oak JASMIN Parser

[![Crates.io](https://img.shields.io/crates/v/oak-jasmin.svg)](https://crates.io/crates/oak-jasmin)
[![Documentation](https://docs.rs/oak-jasmin/badge.svg)](https://docs.rs/oak-jasmin)

High-performance incremental JASMIN parser for the oak ecosystem with flexible configuration, optimized for JVM bytecode assembly and analysis.

## 🎯 Overview

Oak of jasmin is a robust parser for JASMIN, designed to handle complete Java assembler syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for JVM bytecode processing and analysis.

## ✨ Features

- **Complete JASMIN Syntax**: Supports all JASMIN features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_jasmin::JasminParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = JasminParser::new();
    let jasmin_code = r#"
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
    "#;
    
    let class_file = parser.parse_class(jasmin_code)?;
    println!("Parsed JASMIN class successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Class Parsing
```rust
use oak_jasmin::{JasminParser, ast::ClassFile};

let parser = JasminParser::new();
let jasmin_code = r#"
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
"#;

let class_file = parser.parse_class(jasmin_code)?;
println!("Methods: {}", class_file.methods.len());
println!("Fields: {}", class_file.fields.len());
```

### Method Parsing
```rust
use oak_jasmin::{JasminParser, ast::Method};

let parser = JasminParser::new();
let method_code = r#"
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
"#;

let method = parser.parse_method(method_code)?;
println!("Instructions: {}", method.instructions.len());
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_jasmin::{JasminParser, lexer::Token};

let parser = JasminParser::new();
let tokens = parser.tokenize("getstatic java/lang/System/out Ljava/io/PrintStream;")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust
use oak_jasmin::JasminParser;

let parser = JasminParser::new();
let invalid_jasmin = r#"
.class Broken
    .method public bad_method()V
        getstatic java/lang/System/out Ljava/io/PrintStream;
        ldc "Hello"  -- Missing semicolon
        invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
        return
    .end method
.end class
"#;

match parser.parse_class(invalid_jasmin) {
    Ok(class_file) => println!("Parsed JASMIN class successfully."),
    Err(e) => {
        println!("Parse error at line {} column {}: {}", 
            e.line(), e.column(), e.message());
        if let Some(context) = e.context() {
            println!("Error context: {}", context);
        }
    }
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **ClassFile**: Root container for JASMIN classes
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

Oak of jasmin integrates seamlessly with:

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