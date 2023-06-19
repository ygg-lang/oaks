#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
#[cfg(feature = "highlight")]
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;

// 重新导出主要类型
pub use crate::{kind::JsonSyntaxKind, language::JsonLanguage};
