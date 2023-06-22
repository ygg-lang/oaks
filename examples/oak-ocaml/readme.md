# Oak OCaml Parser

[![Crates.io](https://img.shields.io/crates/v/oak-ocaml.svg)](https://crates.io/crates/oak-ocaml)
[![Documentation](https://docs.rs/oak-ocaml/badge.svg)](https://docs.rs/oak-ocaml)

High-performance incremental OCaml parser for the oak ecosystem with flexible configuration, optimized for functional programming language parsing and type system analysis.

## 🎯 Overview

Oak OCaml is a robust parser for OCaml, designed to handle complete OCaml syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for functional programming language parsing and type system analysis.

## ✨ Features

- **Complete OCaml Syntax**: Supports all OCaml features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_ocaml::{OCamlParser, OCamlLanguage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = OCamlLanguage;
    let mut session = ParseSession::<OCamlLanguage>::default();
    let parser = OCamlParser::new(&config);
    let source = SourceText::new(r#"
let rec factorial n =
  if n <= 1 then 1
  else n * factorial (n - 1)

let () =
  let result = factorial 5 in
  Printf.printf "Factorial of 5 is %d\n" result
    "#);
    
    let result = parser.parse(&source, &[], &mut session);
    println!("Parsed OCaml successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_ocaml::{OCamlParser, OCamlLanguage};

let config = OCamlLanguage;
let mut session = ParseSession::<OCamlLanguage>::default();
let parser = OCamlParser::new(&config);
let source = SourceText::new(r#"
let rec map f = function
  | [] -> []
  | h :: t -> f h :: map f t
"#);

let result = parser.parse(&source, &[], &mut session);
println!("Function parsed successfully.");
```

### Module Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_ocaml::{OCamlParser, OCamlLanguage};

let config = OCamlLanguage;
let mut session = ParseSession::<OCamlLanguage>::default();
let parser = OCamlParser::new(&config);
let source = SourceText::new(r#"
module Stack = struct
  type 'a t = 'a list
  
  let empty = []
  let push x s = x :: s
  let pop = function
    | [] -> failwith "Empty stack"
    | h :: t -> (h, t)
end
"#);

let result = parser.parse(&source, &[], &mut session);
println!("Module parsed successfully.");
```

### Pattern Matching Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_ocaml::{OCamlParser, OCamlLanguage};

let config = OCamlLanguage;
let mut session = ParseSession::<OCamlLanguage>::default();
let parser = OCamlParser::new(&config);
let source = SourceText::new(r#"
type expr = 
  | Const of int
  | Add of expr * expr
  | Mul of expr * expr

let rec eval = function
  | Const n -> n
  | Add (e1, e2) -> eval e1 + eval e2
  | Mul (e1, e2) -> eval e1 * eval e2
"#);

let result = parser.parse(&source, &[], &mut session);
println!("Pattern matching parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_ocaml::{OCamlParser, OCamlLanguage};

let config = OCamlLanguage;
let mut session = ParseSession::<OCamlLanguage>::default();
let parser = OCamlParser::new(&config);
let source = SourceText::new("let x = 42");
let result = parser.parse(&source, &[], &mut session);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_ocaml::{OCamlParser, OCamlLanguage};

let mut session = ParseSession::<OCamlLanguage>::default();
let config = OCamlLanguage;
let parser = OCamlParser::new(&config);
let source = SourceText::new(r#"
let x = 
# Missing value
"#);

let result = parser.parse(&source, &[], &mut session);
if let Some(errors) = result.result.err() {
    println!("Parse errors found: {:?}", errors);
} else {
    println!("Parsed successfully.");
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **OCamlSource**: Root container for OCaml source files
- **Module**: OCaml module definitions and structures
- **Expression**: Various expression types including functions and literals
- **Pattern**: Pattern matching expressions
- **Type**: Type definitions and annotations
- **Declaration**: Value and type declarations
- **Signature**: Module signatures and interfaces

## 📊 Performance

- **Streaming**: Parse large OCaml files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak OCaml integrates seamlessly with:

- **IDE Support**: Language server protocol compatibility for OCaml
- **Static Analysis**: Type checking and code analysis tools
- **Code Generation**: Generating code from OCaml AST
- **Documentation**: Extracting documentation from OCaml source
- **Refactoring**: Automated code refactoring tools

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete OCaml module parsing
- Functional programming patterns analysis
- Type system validation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-ocaml) or open [issues](https://github.com/ygg-lang/oaks/issues).