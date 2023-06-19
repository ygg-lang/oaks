# Oak Clojure Parser

[![Crates.io](https://img.shields.io/crates/v/oak-clojure.svg)](https://crates.io/crates/oak-clojure)
[![Documentation](https://docs.rs/oak-clojure/badge.svg)](https://docs.rs/oak-clojure)

High-performance incremental Clojure parser for the oak ecosystem with flexible configuration, optimized for code analysis and compilation.

## 🎯 Overview

Oak Clojure is a robust parser for Clojure, designed to handle complete Clojure syntax including modern features like macros, data structures, and functional programming constructs. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for code analysis and compilation.

## ✨ Features

- **Complete Clojure Syntax**: Supports all Clojure features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_clojure::{Parser, ClojureLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
(defn greet [name]
  (println (str "Hello, " name "!")))

(greet "World")
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Clojure program successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_clojure::{Parser, ClojureLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
(defn add [a b]
  (+ a b))
    "#);

let result = parser.parse(&source);
println!("Parsed Clojure function successfully.");
```

### Data Structure Parsing
```rust
use oak_clojure::{Parser, ClojureLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
{:name "Alice" :age 25 :city "New York"}
    "#);

let result = parser.parse(&source);
println!("Parsed Clojure data structure successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_clojure::{Parser, ClojureLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("(defn add [x y] (+ x y))");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_clojure::{Parser, ClojureLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
(defn broken-function
  "This function has unbalanced parentheses"
  [x y]
  (+ x y ; Missing closing parenthesis
    "#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Program**: Root container for Clojure programs
- **Function**: Function definitions and declarations
- **Macro**: Macro definitions
- **DataStructure**: Maps, vectors, lists, and sets
- **Expression**: Various expression types (function calls, special forms, etc.)

## 📊 Performance

- **Streaming**: Parse large Clojure files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Clojure integrates seamlessly with:

- **Clojure IDEs**: Provide syntax highlighting, code completion, and refactoring capabilities
- **Static Analyzers**: Identify potential bugs and code smells in Clojure code
- **Code Transformers**: Automate code modifications and migrations
- **REPL Integration**: Support for interactive development environments
- **Build Tools**: Integration with Clojure build systems

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Clojure program parsing
- Function and macro analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-clojure) or open [issues](https://github.com/ygg-lang/oaks/issues).