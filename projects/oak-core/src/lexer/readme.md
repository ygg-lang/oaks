# Oak Core Lexer Module

The lexer module provides a flexible and extensible framework for tokenizing source code across different programming languages. It offers reusable components for common lexical constructs and supports incremental parsing for efficient re-tokenization.

## Overview

This module serves as the foundation for lexical analysis in the Oak Core parsing framework. It provides:

- **Generic Lexer Interface**: A trait-based design that allows implementing language-specific lexers
- **Reusable Scanning Components**: Pre-built utilities for common tokens like whitespace, comments, strings, numbers, and identifiers
- **Incremental Parsing Support**: Efficient re-tokenization using caching mechanisms
- **Comprehensive Error Handling**: Integrated diagnostic system for reporting lexical errors

## Core Components

### Lexer Trait

The `Lexer` trait defines the interface for all language-specific lexers:

```rust,ignore
use oak_core::{Lexer, Language, Source, LexOutput, lexer::LexerCache};

struct MyLanguageLexer;

impl Lexer<MyLanguage> for MyLanguageLexer {
    fn lex_incremental(&self, source: impl Source, relex_from: usize, cache: &mut impl LexerCache<MyLanguage>) -> LexOutput<MyLanguage> {
        // Implementation here
        todo!()
    }
}
```

### Token Representation

Tokens are the fundamental units of lexical analysis:

```rust
#![feature(new_range_api)]
use oak_core::Token;
use core::range::Range;

let token = Token {
    kind: "identifier",
    span: Range { start: 0, end: 5 }
};

assert_eq!(token.length(), 5);
```

### Lexer State Management

The `LexerState` provides comprehensive state management during tokenization:

```rust
#![feature(new_range_api)]
use oak_core::lexer::{LexerState, Token};
use oak_core::{Language, TokenType, SourceText, UniversalTokenRole, UniversalElementRole, ElementType};
use core::range::Range;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum SimpleToken { Identifier, Whitespace, End }

impl TokenType for SimpleToken {
    const END_OF_STREAM: Self = SimpleToken::End;
    type Role = UniversalTokenRole;
    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SimpleElement {}

impl ElementType for SimpleElement {
    type Role = UniversalElementRole;
    fn role(&self) -> Self::Role { UniversalElementRole::None }
}

struct SimpleLanguage;

impl Language for SimpleLanguage {
    const NAME: &'static str = "simple";
    type TokenType = SimpleToken;
    type ElementType = SimpleElement;
    type TypedRoot = ();
}

let source = SourceText::new("hello world");
let mut state = LexerState::<_, SimpleLanguage>::new(&source);

// Tokenize identifier "hello"
state.add_token(SimpleToken::Identifier, 0, 5);
state.advance(5);

// Tokenize whitespace
state.add_token(SimpleToken::Whitespace, 5, 6);
state.advance(1);

// Tokenize identifier "world"
state.add_token(SimpleToken::Identifier, 6, 11);
state.advance(5);

// Add end-of-file token
state.add_eof();

assert_eq!(state.tokens().len(), 4);
```
