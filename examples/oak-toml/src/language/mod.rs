use crate::{lexer::TomlLexer, syntax::TomlSyntaxKind};
use oak_core::Language;

/// 日期时间格式
#[derive(Debug, Clone, Copy)]
pub enum DateTimeFormat {
    Rfc3339,
    // 其他可能的日期时间格
}

/// TOML 语言定义
#[derive(Debug, Clone)]
pub struct TomlLanguage {
    pub allow_multiline_strings: bool,
    pub allow_hex_numbers: bool,
    pub datetime_format: DateTimeFormat,
}

impl Language for TomlLanguage {
    type SyntaxKind = TomlSyntaxKind;
}

impl TomlLanguage {
    pub fn new() -> Self {
        Self::standard()
    }

    pub fn standard() -> Self {
        Self { allow_multiline_strings: true, allow_hex_numbers: false, datetime_format: DateTimeFormat::Rfc3339 }
    }

    pub fn lexer(&self) -> TomlLexer<'_> {
        TomlLexer::new(self)
    }
}
