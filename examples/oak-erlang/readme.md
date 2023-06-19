# Oak Erlang Parser

[![Crates.io](https://img.shields.io/crates/v/oak-erlang.svg)](https://crates.io/crates/oak-erlang)
[![Documentation](https://docs.rs/oak-erlang/badge.svg)](https://docs.rs/oak-erlang)

High-performance incremental Erlang parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Erlang is a robust parser for Erlang, designed to handle complete Erlang syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Erlang Syntax**: Supports all Erlang features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_erlang::{Parser, ErlangLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
-module(hello).
-export([greet/0]).

greet() ->
    io:format("Hello, Erlang!~n").
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Erlang successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_erlang::{Parser, ErlangLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
-module(math).
-export([add/2, factorial/1]).

add(A, B) ->
    A + B.

factorial(0) ->
    1;
factorial(N) when N > 0 ->
    N * factorial(N - 1).
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

### Module Parsing
```rust
use oak_erlang::{Parser, ErlangLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
-module(calculator).
-export([new/0, add/2, subtract/2, get_result/1]).

new() ->
    0.

add(Acc, Value) ->
    Acc + Value.

subtract(Acc, Value) ->
    Acc - Value.

get_result(Acc) ->
    Acc.
"#);

let result = parser.parse(&source);
println!("Module parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_erlang::{Parser, ErlangLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("X = 42.");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_erlang::{Parser, ErlangLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
-module(broken).
-export([broken_function/0]).

broken_function() ->
    io:format("Hello"
    % Missing closing parenthesis
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

- **ErlangModule**: Root container for Erlang modules
- **Function**: Erlang functions and clauses
- **Pattern**: Pattern matching constructs
- **Expression**: Various expression types including operators
- **Statement**: Various statement types including control flow
- **Type**: Erlang type system constructs

## 📊 Performance

- **Streaming**: Parse large Erlang files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Erlang integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Erlang AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Erlang code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Erlang module parsing
- Function and pattern analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-erlang) or open [issues](https://github.com/ygg-lang/oaks/issues).