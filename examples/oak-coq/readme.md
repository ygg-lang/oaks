# Oak Coq Parser

[![Crates.io](https://img.shields.io/crates/v/oak-coq.svg)](https://crates.io/crates/oak-coq)
[![Documentation](https://docs.rs/oak-coq/badge.svg)](https://docs.rs/oak-coq)

High-performance incremental Coq parser for the oak ecosystem with flexible configuration, optimized for theorem proving and formal verification.

## 🎯 Overview

Oak Coq is a robust parser for Coq, designed to handle complete Coq syntax including modern features like tactics, definitions, and proofs. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for theorem proving and formal verification.

## ✨ Features

- **Complete Coq Syntax**: Supports all Coq features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_coq::{Parser, CoqLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
Theorem plus_comm : forall n m : nat, n + m = m + n.
Proof.
  intros n m.
  induction n as [| n' IHn'].
  - simpl. reflexivity.
  - simpl. rewrite IHn'. reflexivity.
Qed.
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Coq theorem successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Theorem Parsing
```rust
use oak_coq::{Parser, CoqLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
Theorem plus_assoc : forall n m p : nat,
  n + (m + p) = (n + m) + p.
Proof.
  intros n m p.
  induction n as [| n' IHn'].
  - simpl. reflexivity.
  - simpl. rewrite IHn'. reflexivity.
Qed.
    "#);

let result = parser.parse(&source);
println!("Parsed Coq theorem successfully.");
```

### Definition Parsing
```rust
use oak_coq::{Parser, CoqLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
Definition double (n : nat) : nat :=
  n + n.

Fixpoint factorial (n : nat) : nat :=
  match n with
  | 0 => 1
  | S n' => n * factorial n'
  end.
    "#);

let result = parser.parse(&source);
println!("Parsed Coq definitions successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_coq::{Parser, CoqLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("Theorem plus_comm : forall n m : nat, n + m = m + n.");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_coq::{Parser, CoqLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
Theorem invalid : forall n : nat,
  n = n.
Proof.
  intros n.
  (* Missing Qed *)
    "#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **VernacularCommand**: Top-level commands (Theorem, Definition, etc.)
- **Proof**: Proof scripts with tactics
- **Term**: Coq terms and expressions
- **Inductive**: Inductive definitions
- **Fixpoint**: Recursive function definitions
- **Tactic**: Proof tactics

## 📊 Performance

- **Streaming**: Parse large Coq files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Coq integrates seamlessly with:

- **Proof Assistants**: Integration with Coq and related tools
- **Formal Verification**: Analyzing and verifying formal specifications
- **IDE Support**: Language server protocol compatibility for Coq
- **Documentation**: Extracting documentation from Coq source code
- **Educational Tools**: Building interactive learning environments

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Coq theorem parsing
- Proof script analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-coq) or open [issues](https://github.com/ygg-lang/oaks/issues).