#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;

mod builder;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;

mod formatter;
pub mod highlighter;
pub mod lsp;
pub mod mcp;

pub use crate::{ast::TwigRoot, builder::TwigBuilder, formatter::TwigFormatter, highlighter::TwigHighlighter, language::TwigLanguage, lexer::TwigLexer, lsp::TwigLanguageService, parser::TwigParser};

pub use crate::mcp::serve_twig_mcp;
