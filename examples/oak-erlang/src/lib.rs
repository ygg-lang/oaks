#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
pub mod kind;
pub mod language;
pub mod lexer;

pub use ast::ErlangRoot;
pub use kind::ErlangSyntaxKind;
pub use language::ErlangLanguage;
pub use lexer::ErlangLexer;
