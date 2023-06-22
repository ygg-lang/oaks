#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
mod builder;
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod lsp;
pub mod parser;

// pub mod mcp;

pub use crate::{ast::LuaRoot, builder::LuaBuilder, highlighter::LuaHighlighter, kind::LuaSyntaxKind, language::LuaLanguage, lexer::LuaLexer, lsp::LuaLanguageService, parser::LuaParser};
