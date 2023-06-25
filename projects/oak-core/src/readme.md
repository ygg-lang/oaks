# 🌳 Oak Core

Oak Core is the foundational library for the Oak language framework, providing the essential building blocks for high-performance, fault-tolerant language processing.

## 🚀 Key Features

- **Green/Red Tree Architecture**: Implements an immutable, persistent syntax tree structure inspired by Roslyn and rust-analyzer, enabling efficient tree manipulation and sharing.
- **Incremental Lexing & Parsing**: Supports incremental updates to handle real-time editing in IDEs with minimal re-computation.
- **Arena-based Memory Management**: Utilizes custom arenas and bump allocators for high-performance memory allocation during tree construction.
- **SIMD-accelerated Lexing**: Leverages hardware acceleration for extremely fast tokenization of source text.
- **Fault Tolerance**: Designed to handle incomplete and syntactically incorrect source code, essential for IDE features like IntelliSense.

## 🚦 Quick Start

### Basic Lexing and Parsing

```rust
# use oak_core::{Language, Parser, Lexer, Source, ParseSession, RedTree, TextEdit, TokenType, ElementType, UniversalTokenRole, UniversalElementRole, ParserState, ParseOutput, OakError, GreenNode, ParseCache};
#
# #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
# #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
# enum MyToken { End, Let, Ident, Equal, Int, Semi }
# impl TokenType for MyToken {
#     const END_OF_STREAM: Self = MyToken::End;
#     type Role = UniversalTokenRole;
#     fn role(&self) -> Self::Role { UniversalTokenRole::None }
# }
#
# #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
# #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
# enum MyElement { Root }
# impl ElementType for MyElement {
#     type Role = UniversalElementRole;
#     fn role(&self) -> Self::Role { UniversalElementRole::None }
# }
# impl From<MyToken> for MyElement {
#     fn from(_: MyToken) -> Self { MyElement::Root }
# }
#
# #[derive(Clone, Copy)]
# struct MyLanguage;
# impl Language for MyLanguage {
#     const NAME: &'static str = "test";
#     type TokenType = MyToken;
#     type ElementType = MyElement;
#     type TypedRoot = ();
# }
#
# struct MyParser;
# impl Parser<MyLanguage> for MyParser {
#     fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<MyLanguage>) -> ParseOutput<'a, MyLanguage> {
#         use oak_core::OakDiagnostics;
#         // Dummy implementation
#         let arena = (*cache).arena();
#         let children = arena.alloc_slice_fill_iter(0, std::iter::empty());
#         let node = GreenNode::new(MyElement::Root, children);
#         let root = arena.alloc(node);
#         ParseOutput::success(root)
#     }
# }
#
// 1. Define your language-specific types (TokenType, ElementType)
// 2. Implement the Language trait
// 3. Create a Lexer and Parser for your language
// 4. Parse source text:
let source = "let x = 42;";
let mut cache = ParseSession::<MyLanguage>::new(1024);
let parser = MyParser;
let output = parser.parse(&source, &[], &mut cache);

// 5. Access the resulting syntax tree
let green_tree = output.green();
let red_tree = RedTree::new(green_tree);
```

## 🏗️ Core Components

### 1. Syntax Trees (Green & Red)
- **Green Trees**: Immutable, lossless, and position-independent trees. They are cheap to clone and easy to cache.
- **Red Trees**: State-ful, position-aware views over Green Trees. They provide parent pointers and absolute offsets.

### 2. Incremental Builder
The `Builder` and `BuilderCache` allow for constructing syntax trees incrementally, reusing existing subtrees when the source code changes.

### 3. Memory Management
Uses `SyntaxArena` for efficient allocation of tree nodes, reducing fragmentation and improving cache locality.

### 4. Source & Range
Provides robust source text management with `Source` and precise location tracking using `Range` and `SourceId`.

## 🔍 Module Overview

- [builder](file:///e:/yydb%20%E6%95%B0%E6%8D%AE%E5%BA%93/oaks/projects/oak-core/src/builder/mod.rs): Incremental tree construction.
- [errors](file:///e:/yydb%20%E6%95%B0%E6%8D%AE%E5%BA%93/oaks/projects/oak-core/src/errors/mod.rs): Diagnostic and error reporting.
- [language](file:///e:/yydb%20%E6%95%B0%E6%8D%AE%E5%BA%93/oaks/projects/oak-core/src/language/mod.rs): Language-specific definitions.
- [lexer](file:///e:/yydb%20%E6%95%B0%E6%8D%AE%E5%BA%93/oaks/projects/oak-core/src/lexer/mod.rs): High-performance tokenization.
- [memory](file:///e:/yydb%20%E6%95%B0%E6%8D%AE%E5%BA%93/oaks/projects/oak-core/src/memory/mod.rs): Arena and bump allocation.
- [parser](file:///e:/yydb%20%E6%95%B0%E6%8D%AE%E5%BA%93/oaks/projects/oak-core/src/parser/mod.rs): Recursive descent and Pratt parsing.
- [tree](file:///e:/yydb%20%E6%95%B0%E6%8D%AE%E5%BA%93/oaks/projects/oak-core/src/tree/mod.rs): Green and Red tree implementations.
- [source](file:///e:/yydb%20%E6%95%B0%E6%8D%AE%E5%BA%93/oaks/projects/oak-core/src/source/mod.rs): Source text and location management.

