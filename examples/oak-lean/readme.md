# Oak Lean Parser

[![Crates.io](https://img.shields.io/crates/v/oak-lean.svg)](https://crates.io/crates/oak-lean)
[![Documentation](https://docs.rs/oak-lean/badge.svg)](https://docs.rs/oak-lean)

High-performance incremental Lean parser for the oak ecosystem with flexible configuration, optimized for theorem proving and formal verification.

## 🎯 Overview

Oak Lean is a robust parser for Lean theorem prover, designed to handle complete Lean syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for theorem proving and formal verification.

## ✨ Features

- **Complete Lean Syntax**: Supports all Lean features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_lean::{Parser, LeanLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
def factorial : ℕ → ℕ
| 0 := 1
| (n + 1) := (n + 1) * factorial n

theorem factorial_pos (n : ℕ) : factorial n > 0 :=
begin
    induction n with n ih,
    { simp [factorial] },
    { simp [factorial, ih, mul_pos] }
end
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Lean successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Definition Parsing
```rust
use oak_lean::{Parser, LeanLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
def is_even (n : ℕ) := ∃ k, n = 2 * k

theorem even_plus_even_is_even (m n : ℕ) 
    (h₁ : is_even m) (h₂ : is_even n) : is_even (m + n) :=
begin
    cases h₁ with k hk,
    cases h₂ with l hl,
    use k + l,
    rw [hk, hl],
    ring,
end
"#);

let result = parser.parse(&source);
println!("Definition parsed successfully.");
```

### Inductive Type Parsing
```rust
use oak_lean::{Parser, LeanLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
inductive tree (α : Type)
| leaf : tree
| node : α → tree → tree → tree
"#);

let result = parser.parse(&source);
println!("Inductive type parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_lean::{Parser, LeanLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("def hello := \"world\"");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_lean::{Parser, LeanLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
def broken_function
    println("Hello")
    // Missing function parameters and return type
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

- **LeanProgram**: Root container for Lean programs
- **Definition**: Lean function and constant definitions
- **Theorem**: Lean theorem statements and proofs
- **Inductive**: Lean inductive type definitions
- **Structure**: Lean structure/record definitions
- **Expression**: Lean expressions and terms
- **TacticBlock**: Lean tactic proof blocks

## 📊 Performance

- **Streaming**: Parse large Lean files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Lean integrates seamlessly with:

- **Theorem Proving**: Lean code analysis and verification
- **Formal Verification**: Processing and transforming Lean proofs
- **IDE Support**: Language server protocol compatibility
- **Code Generation**: Generating code from Lean AST
- **Documentation**: Generating documentation from Lean code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Lean program parsing
- Definition and theorem analysis
- Proof transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-lean) or open [issues](https://github.com/ygg-lang/oaks/issues).