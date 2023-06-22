# Oak Valkyrie Parser

[![Crates.io](https://img.shields.io/crates/v/oak-valkyrie.svg)](https://crates.io/crates/oak-valkyrie)
[![Documentation](https://docs.rs/oak-valkyrie/badge.svg)](https://docs.rs/oak-valkyrie)

High-performance incremental Valkyrie parser for the oak ecosystem with flexible configuration, optimized for modern Valkyrie syntax and features.

## 🎯 Overview

Oak of valkyrie is a robust parser for Valkyrie, designed to handle complete Valkyrie syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for Valkyrie language processing.

## ✨ Features

- **Complete Valkyrie Syntax**: Supports all Valkyrie features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust,ignore
use oak_valkyrie::ValkyrieParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = ValkyrieParser::new();
    let valkyrie_code = r#"
module Main {
    data List a = Nil | Cons a (List a)
    
    length : List a -> Int
    length Nil = 0
    length (Cons _ xs) = 1 + length xs
    
    main : IO ()
    main = print (length (Cons 1 (Cons 2 (Cons 3 Nil))))
}
    "#;
    
    let module = parser.parse_module(valkyrie_code)?;
    println!("Parsed Valkyrie module successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Module Parsing
```rust,ignore
use oak_valkyrie::{ValkyrieParser, ast::Module};

let parser = ValkyrieParser::new();
let valkyrie_code = r#"
module Math {
    data Nat = Zero | Succ Nat
    
    add : Nat -> Nat -> Nat
    add Zero n = n
    add (Succ m) n = Succ (add m n)
    
    multiply : Nat -> Nat -> Nat
    multiply Zero _ = Zero
    multiply (Succ m) n = add (multiply m n) n
}
"#;

let module = parser.parse_module(valkyrie_code)?;
println!("Declarations: {}", module.declarations.len());
println!("Functions: {}", module.functions.len());
```

### Expression Parsing
```rust,ignore
use oak_valkyrie::{ValkyrieParser, ast::Expression};

let parser = ValkyrieParser::new();
let expression_code = r#"
let x = 42 in
let y = x * 2 in
if y > 80 then "large" else "small"
"#;

let expression = parser.parse_expression(expression_code)?;
println!("Expression type: {:?}", expression.kind);
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust,ignore
use oak_valkyrie::{ValkyrieParser, lexer::Token};

let parser = ValkyrieParser::new();
let tokens = parser.tokenize("data List a = Nil | Cons a (List a)")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust,ignore
use oak_valkyrie::ValkyrieParser;

let parser = ValkyrieParser::new();
let invalid_valkyrie = r#"
module Broken {
    data List a = Nil | Cons a List a  -- Missing parentheses
    
    bad_function : Int -> String
    bad_function x = x ++ "hello"  -- Type mismatch
}
"#;

match parser.parse_module(invalid_valkyrie) {
    Ok(module) => println!("Parsed Valkyrie module successfully."),
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

- **Module**: Root container for Valkyrie modules
- **DataType**: Algebraic data type definitions
- **Function**: Function definitions with type signatures
- **Expression**: Expressions including let-bindings, conditionals, and function calls
- **Pattern**: Pattern matching constructs
- **Type**: Type annotations and type constructors

## 📊 Performance

- **Streaming**: Parse large Valkyrie files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak of valkyrie integrates seamlessly with:

- **Functional Programming**: Build functional programming languages and tools
- **Type Systems**: Implement advanced type checking and inference
- **IDE Support**: Language server protocol compatibility for Valkyrie
- **Educational Tools**: Build programming language learning environments
- **Research Tools**: Support academic research in programming languages

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Valkyrie module parsing
- Type inference and checking
- Pattern matching and algebraic data types
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-valkyrie) or open [issues](https://github.com/ygg-lang/oaks/issues).