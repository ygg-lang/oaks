# Oak Haskell Parser

[![Crates.io](https://img.shields.io/crates/v/oak-haskell.svg)](https://crates.io/crates/oak-haskell)
[![Documentation](https://docs.rs/oak-haskell/badge.svg)](https://docs.rs/oak-haskell)

High-performance incremental Haskell parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Haskell is a robust parser for Haskell, designed to handle complete Haskell syntax including modern extensions. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Haskell Syntax**: Supports all Haskell features including modern extensions
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_haskell::{Parser, HaskellLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
main :: IO ()
main = putStrLn "Hello, Haskell!"

add :: Int -> Int -> Int
add x y = x + y
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Haskell successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_haskell::{Parser, HaskellLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
factorial :: Integer -> Integer
factorial 0 = 1
factorial n = n * factorial (n - 1)

main :: IO ()
main = do
    let result = factorial 5
    putStrLn $ "Factorial of 5 is: " ++ show result
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

### Data Type Parsing
```rust
use oak_haskell::{Parser, HaskellLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
data Tree a = Empty | Node a (Tree a) (Tree a)
    deriving (Show, Eq)

treeSum :: Tree Int -> Int
treeSum Empty = 0
treeSum (Node value left right) = value + treeSum left + treeSum right

main :: IO ()
main = do
    let tree = Node 1 (Node 2 Empty Empty) (Node 3 Empty Empty)
    putStrLn $ "Sum of tree values: " ++ show (treeSum tree)
"#);

let result = parser.parse(&source);
println!("Data type parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_haskell::{Parser, HaskellLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("let x = 42 in x + 1");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_haskell::{Parser, HaskellLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
-- Invalid Haskell code example
brokenFunction :: Int -> Int
brokenFunction x =
    let y = x + 1
    -- Missing 'in' keyword
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

- **HaskellModule**: Root container for Haskell modules
- **Declaration**: Top-level declarations including functions and data types
- **Expression**: Haskell expressions including literals, applications, and lambdas
- **Pattern**: Pattern matching constructs
- **Type**: Haskell type system constructs
- **Statement**: Various statement types including do-notation

## 📊 Performance

- **Streaming**: Parse large Haskell files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Haskell integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Haskell AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Haskell code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Haskell module parsing
- Function and data type analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-haskell) or open [issues](https://github.com/ygg-lang/oaks/issues).