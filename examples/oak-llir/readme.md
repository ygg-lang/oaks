# Oak LLVM Parser

[![Crates.io](https://img.shields.io/crates/v/oak-llvm.svg)](https://crates.io/crates/oak-llvm)
[![Documentation](https://docs.rs/oak-llvm/badge.svg)](https://docs.rs/oak-llvm)

High-performance incremental LLVM IR parser for the oak ecosystem with flexible configuration, optimized for LLVM intermediate representation processing.

## 🎯 Overview

Oak LLVM is a robust parser for LLVM IR (Intermediate Representation), designed to handle complete LLVM syntax including modern specifications. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for LLVM IR analysis and code generation.

## ✨ Features

- **Complete LLVM IR Syntax**: Supports all LLVM IR features including types, instructions, and metadata
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_llvm::{Parser, LlvmLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
        define i32 @add(i32 %a, i32 %b) {
        entry:
            %sum = add i32 %a, %b
            ret i32 %sum
        }
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed LLVM IR successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Definition Parsing
```rust
use oak_llvm::{Parser, LlvmLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    define i32 @multiply(i32 %a, i32 %b) {
    entry:
        %product = mul i32 %a, %b
        ret i32 %product
    }
"#);

let result = parser.parse(&source);
println!("Function definition parsed successfully.");
```

### Type Declaration Parsing
```rust
use oak_llvm::{Parser, LlvmLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    %Person = type { i32, [10 x i8], i8* }
    %ListNode = type { i32, %ListNode* }
"#);

let result = parser.parse(&source);
println!("Type declarations parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_llvm::{Parser, LlvmLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("define i32 @test() { ret i32 0 }");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_llvm::{Parser, LlvmLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    define i32 @invalid(
        %result = add i32 %a %b
    // Missing closing parenthesis and return
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

- **Module**: LLVM module definitions
- **Function**: Function definitions with parameters and basic blocks
- **BasicBlock**: Basic blocks with instructions
- **Instruction**: LLVM instructions and operands
- **Type**: Type definitions
- **Value**: Values including constants and variables

## 📊 Performance

- **Streaming**: Parse large LLVM IR files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak LLVM integrates seamlessly with:

- **Compiler Tools**: LLVM IR analysis and transformation
- **IDE Support**: Language server protocol compatibility for LLVM IR
- **Optimization**: Analyzing and optimizing LLVM IR
- **Code Generation**: Generating LLVM IR from AST representations
- **Static Analysis**: Analyzing LLVM IR for security and correctness

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete LLVM IR module parsing
- Function and instruction analysis
- Type system analysis
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-llvm) or open [issues](https://github.com/ygg-lang/oaks/issues).