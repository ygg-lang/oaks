#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
pub mod kind;
pub mod language;
pub mod lexer;

pub use ast::OrgModeRoot;
pub use kind::{OrgModeSyntaxKind, OrgModeToken};
pub use language::OrgModeLanguage;
pub use lexer::OrgModeLexer;
