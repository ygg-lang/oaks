# Oak WAT Parser

[![Crates.io](https://img.shields.io/crates/v/oak-wat.svg)](https://crates.io/crates/oak-wat)
[![Documentation](https://docs.rs/oak-wat/badge.svg)](https://docs.rs/oak-wat)

High-performance incremental WAT (WebAssembly Text) parser for the oak ecosystem with flexible configuration, optimized for WebAssembly text format processing.

## 🎯 Overview

Oak WAT is a robust parser for WAT (WebAssembly Text), designed to handle complete WAT syntax including modern specifications. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for WebAssembly text format analysis and code generation.

## ✨ Features

- **Complete WAT Syntax**: Supports all WAT features including modules, functions, and instructions
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_wat::{Parser, WatLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
        (module
            (func $add (param $a i32) (param $b i32) (result i32)
                local.get $a
                local.get $b
                i32.add
            )
            (export "add" (func $add))
        )
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed WAT successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Module Parsing
```rust
use oak_wat::{Parser, WatLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    (module
        (func $multiply (param $a i32) (param $b i32) (result i32)
            local.get $a
            local.get $b
            i32.mul
        )
        (export "multiply" (func $multiply))
    )
"#);

let result = parser.parse(&source);
println!("Module parsed successfully.");
```

### Function Parsing
```rust
use oak_wat::{Parser, WatLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    (func $factorial (param $n i32) (result i32)
        local.get $n
        i32.const 1
        i32.le_s
        if (result i32)
            i32.const 1
        else
            local.get $n
            local.get $n
            i32.const 1
            i32.sub
            call $factorial
            i32.mul
        end
    )
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_wat::{Parser, WatLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("(module (func $test))");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_wat::{Parser, WatLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    (module
        (func $invalid
            local.get $x
        // Missing closing parenthesis
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

- **Module**: WebAssembly module definitions
- **Function**: Function definitions with parameters and instructions
- **Instruction**: WebAssembly instructions and operands
- **Export**: Export definitions
- **Import**: Import definitions
- **Type**: Type definitions

## 📊 Performance

- **Streaming**: Parse large WAT files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak WAT integrates seamlessly with:

- **WebAssembly Tools**: Text format analysis and generation
- **IDE Support**: Language server protocol compatibility for WAT
- **Compilation**: Converting WAT to WASM binary format
- **Code Generation**: Generating WAT from AST representations
- **Validation**: Validating WAT syntax and semantics

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete WAT module parsing
- Function and instruction analysis
- WAT to WASM conversion
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-wat) or open [issues](https://github.com/ygg-lang/oaks/issues).