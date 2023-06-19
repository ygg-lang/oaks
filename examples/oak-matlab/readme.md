# Oak MATLAB Parser

[![Crates.io](https://img.shields.io/crates/v/oak-matlab.svg)](https://crates.io/crates/oak-matlab)
[![Documentation](https://docs.rs/oak-matlab/badge.svg)](https://docs.rs/oak-matlab)

High-performance incremental MATLAB parser for the oak ecosystem with flexible configuration, optimized for scientific computing and data analysis.

## 🎯 Overview

Oak MATLAB is a robust parser for MATLAB, designed to handle complete MATLAB syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete MATLAB Syntax**: Supports all MATLAB features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_matlab::{Parser, MatlabLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
function result = add(a, b)
    result = a + b;
end

disp('Hello, MATLAB!');
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed MATLAB successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_matlab::{Parser, MatlabLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
function result = factorial(n)
    if n <= 1
        result = 1;
    else
        result = n * factorial(n - 1);
    end
end
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

### Matrix Operations Parsing
```rust
use oak_matlab::{Parser, MatlabLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
A = [1, 2, 3; 4, 5, 6; 7, 8, 9];
B = A';
C = A .* B;
D = A \ B;
E = det(A);
F = eig(A);
"#);

let result = parser.parse(&source);
println!("Matrix operations parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_matlab::{Parser, MatlabLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("x = 42;");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_matlab::{Parser, MatlabLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
% Invalid MATLAB code example
function broken_function(
    disp('Hello'
% Missing closing parenthesis and end
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

- **MatlabProgram**: Root container for MATLAB programs
- **Function**: MATLAB functions and methods
- **Statement**: Various statement types including control flow
- **Expression**: Various expression types including operators
- **Matrix**: MATLAB matrix constructs

## 📊 Performance

- **Streaming**: Parse large MATLAB files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak MATLAB integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from MATLAB AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from MATLAB code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete MATLAB program parsing
- Function and matrix analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-matlab) or open [issues](https://github.com/ygg-lang/oaks/issues).