#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
pub mod builder;
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod lsp;
#[cfg(feature = "mcp")]
pub mod mcp;
pub mod parser;

pub use crate::{
    ast::OrgModeRoot,
    builder::OrgModeBuilder,
    highlighter::{HighlightKind, Highlighter, OrgModeHighlighter},
    kind::OrgModeSyntaxKind,
    language::OrgModeLanguage,
    lexer::OrgModeLexer,
    lsp::OrgModeLanguageService,
    parser::OrgModeParser,
};

#[cfg(feature = "mcp")]
pub use crate::mcp::serve_org_mode_mcp;
