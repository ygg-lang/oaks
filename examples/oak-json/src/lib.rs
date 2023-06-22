#![feature(new_range_api)]
#![feature(portable_simd)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

extern crate self as oak_json;

pub mod ast;
pub mod builder;
#[cfg(feature = "highlight")]
pub mod highlighter;
pub mod kind;
pub mod language;
pub mod lexer;
#[cfg(feature = "lsp")]
pub mod lsp;
#[cfg(feature = "lsp")]
pub mod mcp;
pub mod parser;
pub mod serde;

// 重新导出主要类型
pub use crate::{
    ast::JsonValue,
    builder::JsonBuilder,
    kind::JsonSyntaxKind,
    language::JsonLanguage,
    lexer::JsonLexer,
    parser::JsonParser,
    serde::{from_value, to_value},
};

pub fn to_string<T: ::serde::Serialize>(value: &T) -> Result<String, String> {
    let json_value = to_value(value).map_err(|e| e.to_string())?;
    Ok(json_value.to_string())
}

pub fn from_str<T: ::serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    let json_value = parse(s)?;
    from_value(json_value).map_err(|e| e.to_string())
}

pub fn parse(json: &str) -> Result<JsonValue, String> {
    use oak_core::{Builder, SourceText, parser::session::ParseSession};
    let language = JsonLanguage::default();
    let builder = JsonBuilder::new(&language);
    let source = SourceText::new(json.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map(|root| root.value).map_err(|e| format!("{:?}", e))
}

pub use oak_macros::json;

#[cfg(feature = "lsp")]
pub use crate::lsp::JsonLanguageService;

#[cfg(feature = "mcp-stdio")]
pub use crate::mcp::serve_json_mcp;
