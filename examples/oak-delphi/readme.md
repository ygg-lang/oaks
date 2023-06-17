# Oak DOT Language Parser

[![Crates.io](https://img.shields.io/crates/v/oak-dot.svg)](https://crates.io/crates/oak-dot)
[![Documentation](https://docs.rs/oak-dot/badge.svg)](https://docs.rs/oak-dot)

High-performance incremental DOT parser for the oak ecosystem with flexible configuration, optimized for graph description and visualization.

## 🎯 Overview

Oak DOT is a robust parser for the DOT graph description language, designed to handle complete DOT syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for graph analysis and visualization.

## ✨ Features

- **Complete DOT Syntax**: Supports all DOT features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_dot::{Parser, DotLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
digraph G {
    A -> B;
    B -> C;
    C -> A;
}
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed DOT graph successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Graph Parsing
```rust
use oak_dot::{Parser, DotLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
graph G {
    A -- B -- C;
    A -- C;
}
"#);

let result = parser.parse(&source);
println!("Parsed DOT graph successfully.");
```

### Digraph Parsing
```rust
use oak_dot::{Parser, DotLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
digraph workflow {
    start -> process1;
    process1 -> decision;
    decision -> process2 [label="yes"];
    decision -> end [label="no"];
}
"#);

let result = parser.parse(&source);
println!("Parsed DOT digraph successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_dot::{Parser, DotLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("A -> B [color=red];");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_dot::{Parser, DotLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
digraph G {
    A -> B
    B -> C;
    C -> A;
}
"#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Graph**: Root container for undirected graphs
- **Digraph**: Root container for directed graphs
- **Node**: Node definitions with attributes
- **Edge**: Edge definitions with attributes
- **Subgraph**: Nested graph structures
- **Attribute**: Key-value attribute pairs

## 📊 Performance

- **Streaming**: Parse large DOT files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak of dot integrates seamlessly with:

- **Graph Visualization**: Parse DOT files for rendering
- **Graph Analysis**: Analyze graph structures for properties and patterns
- **Code Generation**: Generate DOT files from other data structures
- **IDE Support**: Language server protocol compatibility for DOT
- **Documentation**: Generate documentation from graph definitions

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete DOT graph parsing
- Node and edge analysis
- Attribute processing
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-delphi) or open [issues](https://github.com/ygg-lang/oaks/issues).