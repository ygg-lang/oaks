# Oak Swift Parser

[![Crates.io](https://img.shields.io/crates/v/oak-swift.svg)](https://crates.io/crates/oak-swift)
[![Documentation](https://docs.rs/oak-swift/badge.svg)](https://docs.rs/oak-swift)

High-performance incremental Swift parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Swift is a robust parser for Swift, designed to handle complete Swift syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Swift Syntax**: Supports all Swift features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_swift::{Parser, SwiftLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
func hello() {
    print("Hello, World!")
}

hello()
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Swift successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_swift::{Parser, SwiftLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
func add(a: Int, b: Int) -> Int {
    return a + b
}

let result = add(a: 5, b: 3)
print("Result: \(result)")
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

### Struct Parsing
```rust
use oak_swift::{Parser, SwiftLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
struct Point {
    var x: Double
    var y: Double
    
    func distance(to other: Point) -> Double {
        let dx = self.x - other.x
        let dy = self.y - other.y
        return sqrt(dx * dx + dy * dy)
    }
}
"#);

let result = parser.parse(&source);
println!("Struct parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_swift::{Parser, SwiftLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("let x: Int = 42");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_swift::{Parser, SwiftLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
// Invalid Swift code example
func brokenFunction(
    print("Hello")
// Missing closing brace
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

- **SwiftProgram**: Root container for Swift programs
- **Function**: Swift functions and methods
- **Struct**: Swift struct definitions
- **Statement**: Various statement types including control flow
- **Expression**: Various expression types including operators
- **Type**: Swift type system constructs

## 📊 Performance

- **Streaming**: Parse large Swift files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Swift integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Swift AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Swift code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Swift program parsing
- Function and struct analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-swift) or open [issues](https://github.com/ygg-lang/oaks/issues).