#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod language;

pub mod kind;
pub mod lexer;

pub use crate::{kind::HlslSyntaxKind, language::HlslLanguage, lexer::HlslLexer};
