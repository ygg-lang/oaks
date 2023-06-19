# Oak Vampire Parser

[![Crates.io](https://img.shields.io/crates/v/oak-vampire.svg)](https://crates.io/crates/oak-vampire)
[![Documentation](https://docs.rs/oak-vampire/badge.svg)](https://docs.rs/oak-vampire)

High-performance incremental Vampire parser for the oak ecosystem with flexible configuration, optimized for automated theorem proving and formal verification.

## 🎯 Overview

Oak of vampire is a robust parser for Vampire, designed to handle complete Vampire syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for automated theorem proving and formal verification.

## ✨ Features

- **Complete Vampire Syntax**: Supports all Vampire features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust,no_run
use oak_vampire::{VampireLexer, VampireLanguage};
use oak_core::{Lexer, SourceText};

let language = VampireLanguage::default();
let lexer = VampireLexer::new(&language);
let vampire_code = r#"
fof(ax1, axiom, (
    ! [A] : ( human(A) => mortal(A) )).

fof(ax2, axiom, (
    human(socrates) )).

fof(conj, conjecture, (
    mortal(socrates) )).
    "#;
    
let source = SourceText::new(vampire_code);
let output = lexer.lex(&source);
println!("Parsed Vampire problem successfully.");
```

## 📋 Parsing Examples

### Problem Parsing
```rust,no_run
use oak_vampire::{VampireLexer, VampireLanguage, ast::ValkyrieModule};
use oak_core::{Lexer, SourceText};

let language = VampireLanguage::default();
let lexer = VampireLexer::new(&language);
let vampire_code = r#"
fof(commutativity, axiom, (
    ! [X, Y] : ( X + Y = Y + X ) )).

fof(associativity, axiom, (
    ! [X, Y, Z] : ( (X + Y) + Z = X + (Y + Z) ) )).

fof(goal, conjecture, (
    ! [A, B, C] : ( A + (B + C) = (C + A) + B ) )).
"#;

let source = SourceText::new(vampire_code);
let output = lexer.lex(&source);
println!("Tokens: {}", output.result.map_or(0, |tokens| tokens.len()));
```

### Formula Parsing
```rust,no_run
use oak_vampire::{VampireLexer, VampireLanguage, ast::ValkyrieInstruction};
use oak_core::{Lexer, SourceText};

let language = VampireLanguage::default();
let lexer = VampireLexer::new(&language);
let formula_code = r#"
! [X, Y] : ( parent(X, Y) => ( ancestor(X, Y) & ~ sibling(X, Y) ) )
"#;

let source = SourceText::new(formula_code);
let output = lexer.lex(source);
println!("Formula tokens: {}", output.result.map_or(0, |tokens| tokens.len()));
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust,no_run
use oak_vampire::{VampireLexer, VampireLanguage};
use oak_core::{Lexer, SourceText};

let language = VampireLanguage::default();
let lexer = VampireLexer::new(&language);
let source = SourceText::new("fof(ax1, axiom, ( human(socrates) )).");
let output = lexer.lex(source);
println!("Tokens: {}", output.result.map_or(0, |tokens| tokens.len()));
```

### Error Handling
```rust,no_run
use oak_vampire::{VampireLexer, VampireLanguage};
use oak_core::{Lexer, SourceText};

let language = VampireLanguage::default();
let lexer = VampireLexer::new(&language);
let invalid_code = "fof(invalid syntax here";
let source = SourceText::new(invalid_code);
let output = lexer.lex(source);

if let Some(errors) = output.errors {
    for error in errors {
        println!("Error: {:?}", error);
    }
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Problem**: Root container for Vampire problems
- **Formula**: Logical formulas with quantifiers and connectives
- **Term**: Function symbols and variables
- **Clause**: Disjunctions of literals
- **Literal**: Atomic formulas and their negations
- **Quantifier**: Universal and existential quantifiers

## 📊 Performance

- **Streaming**: Parse large Vampire files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak of vampire integrates seamlessly with:

- **Theorem Provers**: Build automated reasoning systems
- **Formal Verification**: Verify software and hardware correctness
- **IDE Support**: Language server protocol compatibility for Vampire
- **Educational Tools**: Build logic and proof assistants
- **Research Tools**: Support academic research in automated reasoning

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Vampire problem parsing
- Formula analysis and transformation
- Proof extraction and validation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-vampire) or open [issues](https://github.com/ygg-lang/oaks/issues).