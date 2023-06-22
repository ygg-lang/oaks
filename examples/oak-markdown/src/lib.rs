#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod language;

pub mod ast;
pub mod builder;
pub mod highlighter;
pub mod kind;
pub mod lexer;
pub mod lsp;
pub mod mcp;
pub mod parser;

pub use crate::{ast::MarkdownRoot, builder::MarkdownBuilder, highlighter::MarkdownHighlighter, kind::MarkdownSyntaxKind, language::MarkdownLanguage, lexer::MarkdownLexer, parser::MarkdownParser};

pub use crate::mcp::serve_markdown_mcp;
