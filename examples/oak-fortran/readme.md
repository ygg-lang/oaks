# Oak Fortran Parser

[![Crates.io](https://img.shields.io/crates/v/oak-fortran.svg)](https://crates.io/crates/oak-fortran)
[![Documentation](https://docs.rs/oak-fortran/badge.svg)](https://docs.rs/oak-fortran)

High-performance incremental Fortran parser for the oak ecosystem with flexible configuration, optimized for code analysis and compilation.

## 🎯 Overview

Oak Fortran is a robust parser for Fortran, designed to handle complete Fortran syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Fortran Syntax**: Supports all Fortran features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_fortran::{Parser, FortranLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
program hello
    print *, "Hello, Fortran!"
end program hello
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Fortran successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Program Parsing
```rust
use oak_fortran::{Parser, FortranLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
program calculator
    implicit none
    real :: a, b, result
    
    print *, "Enter two numbers:"
    read *, a, b
    result = a + b
    print *, "Sum:", result
end program calculator
"#);

let result = parser.parse(&source);
println!("Program parsed successfully.");
```

### Subroutine Parsing
```rust
use oak_fortran::{Parser, FortranLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
subroutine swap(a, b)
    implicit none
    real, intent(inout) :: a, b
    real :: temp
    
    temp = a
    a = b
    b = temp
end subroutine swap

program test
    implicit none
    real :: x = 1.0, y = 2.0
    
    print *, "Before swap: x =", x, "y =", y
    call swap(x, y)
    print *, "After swap: x =", x, "y =", y
end program test
"#);

let result = parser.parse(&source);
println!("Subroutine parsed successfully.");
```

### Function Parsing
```rust
use oak_fortran::{Parser, FortranLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
function factorial(n) result(fact)
    implicit none
    integer, intent(in) :: n
    integer :: fact
    
    if (n <= 0) then
        fact = 1
    else
        fact = n * factorial(n - 1)
    end if
end function factorial

program test
    implicit none
    integer :: n = 5
    
    print *, "Factorial of", n, "is", factorial(n)
end program test
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_fortran::{Parser, FortranLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("x = 42.0");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_fortran::{Parser, FortranLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
program broken
    implicit none
    real :: x
    
    print *, "Hello, Fortran!"
    x = 5
    ! Missing end program statement
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

- **Program**: Root container for Fortran programs
- **Subroutine**: Subroutine definitions
- **Function**: Function definitions
- **Module**: Module definitions
- **Statement**: Various statement types including control flow
- **Expression**: Various expression types including operators

## 📊 Performance

- **Streaming**: Parse large Fortran files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Fortran integrates seamlessly with:

- **Compilers**: Front-end for Fortran compilers
- **Static Analysis Tools**: Code quality and security analysis
- **IDE Support**: Language server protocol compatibility
- **Code Generation**: Generating code from AST

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Fortran program parsing
- Subroutine and function analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-fortran) or open [issues](https://github.com/ygg-lang/oaks/issues).