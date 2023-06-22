#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

extern crate oak_core;
extern crate serde;

pub mod ast;
mod builder;
mod formatter;
pub mod highlighter;
mod language;
pub mod lexer;
pub mod lsp;
pub mod parser;

pub mod mcp;

// 重新导出主要类型
pub use crate::{
    ast::ActionScriptRoot,
    builder::ActionScriptBuilder,
    formatter::ActionScriptFormatter,
    highlighter::ActionScriptHighlighter,
    language::ActionScriptLanguage,
    lexer::{ActionScriptLexer, ActionScriptTokenType},
    lsp::ActionScriptLanguageService,
    parser::{ActionScriptElementType, ActionScriptParser},
};

pub use crate::mcp::serve_actionscript_mcp;
