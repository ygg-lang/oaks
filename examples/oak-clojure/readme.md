# Oak Clojure Parser

[![Crates.io](https://img.shields.io/crates/v/oak-clojure.svg)](https://crates.io/crates/oak-clojure)
[![Documentation](https://docs.rs/oak-clojure/badge.svg)](https://docs.rs/oak-clojure)

High-performance incremental Clojure parser for the oak ecosystem with flexible configuration, optimized for code analysis and compilation.

## 🎯 Overview

Oak-clojure is a robust parser for Clojure, designed to handle complete Clojure syntax including modern features like macros, data structures, and functional programming constructs. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for code analysis and compilation.

## ✨ Features

- **Complete Clojure Syntax**: Supports all Clojure features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_clojure::ClojureParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = ClojureParser::new();
    let clojure_code = r#"
(defn greet [name]
  (println (str "Hello, " name "!")))

(greet "World")
"#;
    
    let program = parser.parse_program(clojure_code)?;
    println!("Parsed Clojure program successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_clojure::{ClojureParser, ast::Function};

let parser = ClojureParser::new();
let clojure_code = r#"
(defn add [a b]
  (+ a b))
"#;

let function = parser.parse_function(clojure_code)?;
println!("Function name: {}", function.name);
```

### Data Structure Parsing
```rust
use oak_clojure::{ClojureParser, ast::DataStructure};

let parser = ClojureParser::new();
let clojure_code = r#"
{:name "Alice" :age 25 :city "New York"}
"#;

let data = parser.parse_data_structure(clojure_code)?;
println!("Parsed data structure with {} elements", data.elements.len());
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_clojure::{ClojureParser, lexer::Token};

let parser = ClojureParser::new();
let tokens = parser.tokenize("(defn add [x y] (+ x y))")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust
use oak_clojure::ClojureParser;

let parser = ClojureParser::new();
let invalid_clojure = r#"
(defn broken-function
  "This function has unbalanced parentheses"
  [x y]
  (+ x y ; Missing closing parenthesis
"#;

match parser.parse_program(invalid_clojure) {
    Ok(program) => println!("Parsed Clojure program successfully."),
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

Oak-clojure integrates seamlessly with:

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