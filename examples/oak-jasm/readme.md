# Oak JASM Parser

[![Crates.io](https://img.shields.io/crates/v/oak-jasm.svg)](https://crates.io/crates/oak-jasm)
[![Documentation](https://docs.rs/oak-jasm/badge.svg)](https://docs.rs/oak-jasm)

High-performance incremental JASM parser for the oak ecosystem with flexible configuration, optimized for assembly language analysis and JVM bytecode generation.

## 🎯 Overview

Oak JASM is a robust parser for Java ASseMbler (JASM), designed to handle complete JASM syntax including modern assembly features and JVM bytecode instructions. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for JASM analysis and tooling.

## ✨ Features

- **Complete JASM Syntax**: Supports all JASM features including class definitions, methods, and bytecode instructions
- **JVM Bytecode Support**: Handles all JVM bytecode instructions and type descriptors
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_jasm::{JasmParser, JasmLanguage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = ParseSession::<JasmLanguage>::default();
    let parser = JasmParser::new();
    let source = SourceText::new(r#"
.class public Hello
.super java/lang/Object

.method public <init>()V
    aload_0
    invokespecial java/lang/Object/<init>()V
    return
.end method

.method public static main([Ljava/lang/String;)V
    .limit stack 2
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "Hello, JASM!"
    invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
    return
.end method
.end class
    "#);
    
    let result = parser.parse(&source, &[], &mut session);
    println!("Parsed JASM class successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Class Definition Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_jasm::{JasmParser, JasmLanguage};

let mut session = ParseSession::<JasmLanguage>::default();
let parser = JasmParser::new();
let source = SourceText::new(r#"
.class public Calculator
.super java/lang/Object

.field private result I

.method public <init>()V
    aload_0
    invokespecial java/lang/Object/<init>()V
    aload_0
    iconst_0
    putfield Calculator/result I
    return
.end method

.method public add(I)V
    aload_0
    dup
    getfield Calculator/result I
    iload_1
    iadd
    putfield Calculator/result I
    return
.end method
.end class
"#);

let result = parser.parse(&source, &[], &mut session);
println!("Parsed JASM class with fields and methods successfully.");
```

### Method with Control Flow
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_jasm::{JasmParser, JasmLanguage};

let mut session = ParseSession::<JasmLanguage>::default();
let parser = JasmParser::new();
let source = SourceText::new(r#"
.class public LoopExample
.super java/lang/Object

.method public static count(I)V
    .limit locals 2
    iconst_0
    istore_1
Loop:
    iload_1
    iload_0
    if_icmpge End
    
    getstatic java/lang/System/out Ljava/io/PrintStream;
    iload_1
    invokevirtual java/io/PrintStream/println(I)V
    
    iinc 1 1
    goto Loop
End:
    return
.end method
.end class
"#);

let result = parser.parse(&source, &[], &mut session);
println!("Parsed JASM with control flow successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_jasm::{JasmParser, JasmLanguage};

let mut session = ParseSession::<JasmLanguage>::default();
let parser = JasmParser::new();
let source = SourceText::new(".class public MyClass");
let result = parser.parse(&source, &[], &mut session);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_jasm::{JasmParser, JasmLanguage};

let mut session = ParseSession::<JasmLanguage>::default();
let parser = JasmParser::new();
let source = SourceText::new(r#"
.class public Broken
# Missing super class or methods
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

- **Class**: JASM class definitions with access modifiers and inheritance
- **Field**: Field definitions with types and access modifiers
- **Method**: Method definitions with signatures and bytecode instructions
- **Instruction**: JVM bytecode instructions with operands
- **Constant**: Constant pool entries for strings, numbers, and references

## 📊 Performance

- **Streaming**: Parse large JASM files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak-jasm integrates seamlessly with:

- **Bytecode Analysis**: Security analysis and optimization of JVM bytecode
- **Code Generation**: Generating bytecode from high-level languages
- **IDE Support**: Language server protocol compatibility for assembly languages
- **Debugging Tools**: Debuggers and profilers for JVM applications
- **Documentation**: Generating documentation from assembly code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete JASM class parsing
- Method and instruction analysis
- Control flow and exception handling
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-jasm) or open [issues](https://github.com/ygg-lang/oaks/issues).