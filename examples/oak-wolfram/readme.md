# Oak Wolfram Parser

[![Crates.io](https://img.shields.io/crates/v/oak-wolfram.svg)](https://crates.io/crates/oak-wolfram)
[![Documentation](https://docs.rs/oak-wolfram/badge.svg)](https://docs.rs/oak-wolfram)

High-performance incremental Wolfram Language parser for the oak ecosystem with flexible configuration, optimized for mathematical computation and symbolic analysis.

## 🎯 Overview

Oak of wolfram is a robust parser for the Wolfram Language, designed to handle complete Wolfram syntax including mathematical expressions, symbolic computations, and functional programming constructs. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for mathematical analysis and code generation.

## ✨ Features

- **Complete Wolfram Syntax**: Supports all Wolfram Language features including mathematical expressions
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_wolfram::WolframParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = WolframParser::new();
    let wolfram_code = r#"
        f[x_] := x^2 + 2*x + 1
        Plot[f[x], {x, -10, 10}]
    "#;
    
    let ast = parser.parse_expression(wolfram_code)?;
    println!("Parsed Wolfram expression successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Definition Parsing
```rust
use oak_wolfram::{WolframParser, ast::FunctionDefinition};

let parser = WolframParser::new();
let wolfram_code = r#"
    factorial[n_] := If[n <= 1, 1, n * factorial[n - 1]]
"#;

let function = parser.parse_function(wolfram_code)?;
println!("Function name: {}", function.name);
```

### Expression Parsing
```rust
use oak_wolfram::{WolframParser, ast::Expression};

let parser = WolframParser::new();
let wolfram_code = r#"
    Integrate[Sin[x], {x, 0, Pi}]
"#;

let expression = parser.parse_expression(wolfram_code)?;
println!("Expression type: {:?}", expression.kind);
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_wolfram::{WolframParser, lexer::Token};

let parser = WolframParser::new();
let tokens = parser.tokenize("f[x_] := x^2")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust
use oak_wolfram::WolframParser;

let parser = WolframParser::new();
let invalid_wolfram = r#"
    f[x_ := x^2 + 1
    (* Missing closing bracket *)
"#;

match parser.parse_expression(invalid_wolfram) {
    Ok(ast) => println!("Parsed Wolfram expression successfully."),
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

- **Expression**: Mathematical and symbolic expressions
- **FunctionDefinition**: Function definitions with patterns
- **Rule**: Rewrite rules and transformations
- **List**: Ordered collections of expressions
- **Symbol**: Atomic symbols and identifiers

## 📊 Performance

- **Streaming**: Parse large Wolfram expressions without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak of wolfram integrates seamlessly with:

- **Mathematical Computation**: Symbolic mathematics and calculus
- **Code Generation**: Generating code from Wolfram expressions
- **IDE Support**: Language server protocol compatibility
- **Educational Tools**: Mathematical expression parsing for learning platforms
- **Scientific Computing**: Parsing and analyzing mathematical models

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Wolfram expression parsing
- Function and rule analysis
- Mathematical expression transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-wolfram) or open [issues](https://github.com/ygg-lang/oaks/issues).