use crate::{lexer::TomlLexer, parser::TomlParser};
use oak_core::{Language, LanguageCategory};

/// 日期时间格式
#[derive(Debug, Clone, Copy)]
pub enum DateTimeFormat {
    Rfc3339,
    // 其他可能的日期时间格
}

/// TOML 语言定义
#[derive(Debug, Clone, Copy)]
pub struct TomlLanguage {
    pub allow_multiline_strings: bool,
    pub allow_hex_numbers: bool,
    pub datetime_format: DateTimeFormat,
}

impl Language for TomlLanguage {
    const NAME: &'static str = "toml";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = crate::kind::TomlSyntaxKind;
    type ElementType = crate::kind::TomlSyntaxKind;
    type TypedRoot = crate::ast::TomlRoot;
}

impl Default for TomlLanguage {
    fn default() -> Self {
        Self::standard()
    }
}

impl TomlLanguage {
    pub fn new() -> Self {
        Self::standard()
    }

    pub fn standard() -> Self {
        Self { allow_multiline_strings: true, allow_hex_numbers: false, datetime_format: DateTimeFormat::Rfc3339 }
    }

    pub fn lexer(&self) -> TomlLexer {
        TomlLexer::new(self)
    }

    pub fn parser(&self) -> TomlParser<'_> {
        TomlParser::new(self)
    }
}
