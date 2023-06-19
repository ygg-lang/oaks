#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

mod kind;
mod language;
mod lexer;

pub use kind::{LuaSyntaxKind, LuaToken};
pub use language::LuaLanguage;
pub use lexer::LuaLexer;
