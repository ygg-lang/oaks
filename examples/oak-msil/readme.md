# Oak MSIL Parser

[![Crates.io](https://img.shields.io/crates/v/oak-msil.svg)](https://crates.io/crates/oak-msil)
[![Documentation](https://docs.rs/oak-msil/badge.svg)](https://docs.rs/oak-msil)

High-performance incremental MSIL (Microsoft Intermediate Language) parser for the oak ecosystem with flexible configuration, optimized for .NET assembly analysis and tooling.

## 🎯 Overview

Oak of msil is a robust parser for MSIL/CIL, designed to handle complete Microsoft Intermediate Language syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for .NET assembly processing and analysis.

## ✨ Features

- **Complete MSIL Syntax**: Supports all MSIL/CIL features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_msil::MsilParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = MsilParser::new();
    let msil_code = r#"
.assembly extern mscorlib {}
.assembly Test {}

.method static void Main() cil managed
{
    .entrypoint
    .maxstack 8
    
    ldstr "Hello, World!"
    call void [mscorlib]System.Console::WriteLine(string)
    ret
}
    "#;
    
    let assembly = parser.parse_assembly(msil_code)?;
    println!("Parsed MSIL assembly successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Assembly Parsing
```rust
use oak_msil::{MsilParser, ast::Assembly};

let parser = MsilParser::new();
let msil_code = r#"
.assembly Calculator
{
    .ver 1:0:0:0
}

.module Calculator.exe

.class public Calculator
{
    .method public static int32 Add(int32, int32) cil managed
    {
        .maxstack 2
        ldarg.0
        ldarg.1
        add
        ret
    }
}
"#;

let assembly = parser.parse_assembly(msil_code)?;
println!("Modules: {}", assembly.modules.len());
println!("Classes: {}", assembly.classes.len());
```

### Method Parsing
```rust
use oak_msil::{MsilParser, ast::Method};

let parser = MsilParser::new();
let method_code = r#"
.method public static int32 Factorial(int32) cil managed
{
    .maxstack 2
    .locals init (int32 V_0, int32 V_1)
    
    ldarg.0
    ldc.i4.1
    ble.s L1
    
    ldarg.0
    ldarg.0
    ldc.i4.1
    sub
    call int32 Calculator::Factorial(int32)
    mul
    ret
    
L1:
    ldc.i4.1
    ret
}
"#;

let method = parser.parse_method(method_code)?;
println!("Instructions: {}", method.instructions.len());
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_msil::{MsilParser, lexer::Token};

let parser = MsilParser::new();
let tokens = parser.tokenize("ldstr \"Hello\"\ncall void [mscorlib]System.Console::WriteLine(string)")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust
use oak_msil::MsilParser;

let parser = MsilParser::new();
let invalid_msil = r#"
.method public static void Broken() cil managed
{
    .maxstack 1
    ldstr "Hello"  -- Missing quotes
    call void [mscorlib]System.Console::WriteLine(string)
    ret
}
"#;

match parser.parse_method(invalid_msil) {
    Ok(method) => println!("Parsed MSIL method successfully."),
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

- **Assembly**: Root container for MSIL assemblies
- **Module**: Module definitions within assemblies
- **Class**: Class definitions with methods and fields
- **Method**: Method definitions with IL instructions
- **Instruction**: Individual IL instructions
- **ExceptionHandler**: Exception handling clauses
- **LocalVariable**: Local variable declarations

## 📊 Performance

- **Streaming**: Parse large MSIL files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak of msil integrates seamlessly with:

- **.NET Analysis**: Build .NET assembly analysis tools
- **Reverse Engineering**: Support reverse engineering workflows
- **IDE Support**: Language server protocol compatibility for MSIL
- **Compiler Development**: Build .NET language compilers
- **Security Tools**: Analyze .NET assemblies for security vulnerabilities

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete MSIL assembly parsing
- Instruction analysis and optimization
- Exception handling analysis
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-msil) or open [issues](https://github.com/ygg-lang/oaks/issues).