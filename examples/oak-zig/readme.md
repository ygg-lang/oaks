# Oak Zig Parser

[![Crates.io](https://img.shields.io/crates/v/oak-zig.svg)](https://crates.io/crates/oak-zig)
[![Documentation](https://docs.rs/oak-zig/badge.svg)](https://docs.rs/oak-zig)

High-performance incremental Zig parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Zig is a robust parser for Zig, designed to handle complete Zig syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Zig Syntax**: Supports all Zig features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_zig::{Parser, ZigLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
const std = @import("std");

pub fn main() void {
    const message = "Hello, Zig!";
    std.debug.print("{s}\n", .{message});
}
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Zig successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_zig::{Parser, ZigLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
pub fn add(a: i32, b: i32) i32 {
    return a + b;
}

pub fn main() void {
    const result = add(5, 3);
    std.debug.print("Result: {}\n", .{result});
}
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

### Struct Parsing
```rust
use oak_zig::{Parser, ZigLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
const Point = struct {
    x: f64,
    y: f64,
    
    pub fn distance(self: Point, other: Point) f64 {
        const dx = self.x - other.x;
        const dy = self.y - other.y;
        return @sqrt(dx * dx + dy * dy);
    }
};
"#);

let result = parser.parse(&source);
println!("Struct parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_zig::{Parser, ZigLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("const x: i32 = 42;");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_zig::{Parser, ZigLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
// Invalid Zig code example
pub fn broken_function(
    x: i32,
    // Missing closing parenthesis and return type
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

- **ZigProgram**: Root container for Zig programs
- **Function**: Zig functions and methods
- **Struct**: Zig struct definitions
- **Statement**: Various statement types including control flow
- **Expression**: Various expression types including operators
- **Type**: Zig type system constructs

## 📊 Performance

- **Streaming**: Parse large Zig files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Zig integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Zig AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Zig code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Zig program parsing
- Function and struct analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-zig) or open [issues](https://github.com/ygg-lang/oaks/issues).