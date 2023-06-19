# Oak COBOL Parser

[![Crates.io](https://img.shields.io/crates/v/oak-cobol.svg)](https://crates.io/crates/oak-cobol)
[![Documentation](https://docs.rs/oak-cobol/badge.svg)](https://docs.rs/oak-cobol)

High-performance incremental COBOL parser for the oak ecosystem with flexible configuration, optimized for legacy system analysis and mainframe development.

## 🎯 Overview

Oak COBOL is a robust parser for COBOL, designed to handle complete COBOL syntax including legacy and modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for legacy system analysis and mainframe development.

## ✨ Features

- **Complete COBOL Syntax**: Supports all COBOL features including legacy specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_cobol::{Parser, CobolLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
       IDENTIFICATION DIVISION.
       PROGRAM-ID. HELLO-WORLD.
       
       ENVIRONMENT DIVISION.
       CONFIGURATION SECTION.
       
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-MESSAGE PIC X(20) VALUE 'Hello, World!'.
       
       PROCEDURE DIVISION.
       MAIN-PROCEDURE.
           DISPLAY WS-MESSAGE
           STOP RUN.
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed COBOL program successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Program Parsing
```rust
use oak_cobol::{Parser, CobolLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
IDENTIFICATION DIVISION.
PROGRAM-ID. CALCULATOR.

DATA DIVISION.
WORKING-STORAGE SECTION.
01 NUM1 PIC 9(3) VALUE 10.
01 NUM2 PIC 9(3) VALUE 20.
01 RESULT PIC 9(4).

PROCEDURE DIVISION.
MAIN-PROCEDURE.
    COMPUTE RESULT = NUM1 + NUM2
    DISPLAY 'Result: ' RESULT
    STOP RUN.
    "#);

let result = parser.parse(&source);
println!("Parsed COBOL program successfully.");
```

### Data Division Parsing
```rust
use oak_cobol::{Parser, CobolLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
DATA DIVISION.
WORKING-STORAGE SECTION.
01 CUSTOMER-RECORD.
    05 CUSTOMER-ID PIC 9(5).
    05 CUSTOMER-NAME PIC X(30).
    05 CUSTOMER-BALANCE PIC 9(7)V99.
    "#);

let result = parser.parse(&source);
println!("Parsed COBOL data division successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_cobol::{Parser, CobolLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("IDENTIFICATION DIVISION.");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_cobol::{Parser, CobolLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
IDENTIFICATION DIVISION
PROGRAM-ID. INVALID
DATA DIVISION
PROCEDURE DIVISION
    DISPLAY 'Missing periods'
    STOP RUN
    "#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Program**: Root container for COBOL programs
- **IdentificationDivision**: Program identification information
- **EnvironmentDivision**: Environment and configuration settings
- **DataDivision**: Data definitions and working storage
- **ProcedureDivision**: Program logic and procedures
- **Statement**: Various COBOL statement types
- **Expression**: Arithmetic and conditional expressions

## 📊 Performance

- **Streaming**: Parse large COBOL files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak COBOL integrates seamlessly with:

- **Legacy System Analysis**: Analyze and understand legacy COBOL codebases
- **Mainframe Development**: Build tools for mainframe development environments
- **Migration Tools**: Convert COBOL code to modern languages
- **IDE Support**: Language server protocol compatibility for COBOL
- **Documentation Tools**: Extract documentation from COBOL source code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete COBOL program parsing
- Data division analysis
- Procedure division extraction
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-cobol) or open [issues](https://github.com/ygg-lang/oaks/issues).