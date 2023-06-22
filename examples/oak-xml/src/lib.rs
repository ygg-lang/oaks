#![feature(new_range_api)]
#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub mod ast;
pub mod builder;
mod kind;
mod language;
mod lexer;
pub mod parser;
pub mod serde;

pub use crate::{
    ast::{XmlNodeExt, XmlValue},
    kind::XmlSyntaxKind,
    language::XmlLanguage,
    lexer::XmlLexer,
    parser::XmlParser,
    serde::{from_value, to_value},
};

pub fn to_string<T: ::serde::Serialize>(value: &T) -> Result<String, String> {
    let xml_value = to_value(value).map_err(String::from)?;
    Ok(xml_value.to_string())
}

pub fn from_str<T: ::serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    let xml_value = parse(s)?;
    from_value(xml_value).map_err(String::from)
}

pub fn parse(xml: &str) -> Result<XmlValue, String> {
    use crate::builder::XmlBuilder;
    use oak_core::{SourceText, builder::Builder, parser::ParseSession};
    let builder = XmlBuilder::new();
    let source = SourceText::new(xml.to_string());
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result.map(|root| root.value).map_err(|e| format!("Parse failed: {:?}, diagnostics: {:?}", e, result.diagnostics))
}
