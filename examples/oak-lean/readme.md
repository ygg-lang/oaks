# Oak Lean Parser

[![Crates.io](https://img.shields.io/crates/v/oak-lean.svg)](https://crates.io/crates/oak-lean)
[![Documentation](https://docs.rs/oak-lean/badge.svg)](https://docs.rs/oak-lean)

A high-performance Lean theorem prover parser for Rust, built with the Oak parser combinator framework. Parse Lean code with comprehensive AST generation and error handling.

## Overview

Oak Lean provides robust parsing capabilities for Lean theorem prover files, supporting tactics, definitions, theorems, proofs, and all major Lean constructs. Built on the Oak parser combinator framework, it delivers excellent performance and detailed error messages.

## Features

- ✅ **Complete Lean Support**: Parse tactics, definitions, theorems, proofs, and imports
- ✅ **Modern Rust API**: Type-safe parsing with comprehensive error handling
- ✅ **High Performance**: Built on the efficient Oak parser combinator framework
- ✅ **Rich AST**: Detailed Abstract Syntax Tree with source location tracking
- ✅ **Extensible**: Easy to extend for custom Lean dialects
- ✅ **Well Tested**: Comprehensive test suite with real-world examples

## Quick Start

## Parsing Examples

### Basic Definition Parsing

```rust
use oak::{Parser, Language};
use oak_lean::LeanLanguage;

fn main() {
    let source = r#"
        def factorial : ℕ → ℕ
        | 0 := 1
        | (n + 1) := (n + 1) * factorial n
        
        theorem factorial_pos (n : ℕ) : factorial n > 0 :=
        begin
            induction n with n ih,
            { simp [factorial] },
            { simp [factorial, ih, mul_pos] }
        end
    "#;
    
    let mut parser = Parser::<LeanLanguage>::new();
    match parser.parse(&source) {
        Ok(ast) => {
            println!("Parsed AST: {:#?}", ast);
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

### Complex Theorem with Tactics

```rust
use oak::{Parser, Language};
use oak_lean::LeanLanguage;

fn main() {
    let source = r#"
        import data.nat.basic
        
        namespace my_namespace
        
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
        
        end my_namespace
    "#;
    
    let mut parser = Parser::<LeanLanguage>::new();
    match parser.parse(&source) {
        Ok(ast) => {
            println!("Theorem parsed successfully!");
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

## Advanced Features

### Inductive Types

Oak Lean supports parsing inductive type definitions:

```rust
let source = r#"
    inductive tree (α : Type)
    | leaf : tree
    | node : α → tree → tree → tree
"#;
```

### Type Classes

Parse type class instances and definitions:

```rust
let source = r#"
    class monoid (α : Type) extends has_mul α, has_one α :=
    (mul_assoc : ∀ a b c : α, (a * b) * c = a * (b * c))
    (one_mul : ∀ a : α, 1 * a = a)
    (mul_one : ∀ a : α, a * 1 = a)
"#;
```

## AST Structure

The parser generates a rich AST with the following main node types:

- `LeanFile` - Root node containing the entire file
- `Import` - Import statements
- `Definition` - Function and constant definitions
- `Theorem` - Theorem statements and proofs
- `Inductive` - Inductive type definitions
- `Structure` - Structure/record definitions
- `Instance` - Type class instances
- `TacticBlock` - Tactic proof blocks
- `Expression` - Lean expressions and terms

## Performance

Oak Lean is designed for high performance:

- **Zero-copy parsing** where possible
- **Streaming support** for large files
- **Efficient memory usage** with minimal allocations
- **Fast error recovery** for better developer experience

## Integration

Oak Lean integrates seamlessly with the Oak ecosystem:

```rust
use oak::{Parser, Language};
use oak_lean::LeanLanguage;

// Use with other Oak parsers
let mut parser = Parser::<LeanLanguage>::new();
let result = parser.parse(lean_source);
```

## Examples

More examples can be found in the [examples directory](https://github.com/axodotdev/oak/tree/main/examples/oak-lean/examples):

- [Basic definitions](examples/definitions.rs)
- [Theorem proving](examples/theorems.rs)
- [Tactic parsing](examples/tactics.rs)
- [Error handling](examples/error_handling.rs)

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/axodotdev/oak/blob/main/CONTRIBUTING.md) for details.