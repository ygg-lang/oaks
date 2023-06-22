use crate::lexer::StylusLexer;
use oak_core::{Language, LanguageCategory};

/// 日期时间格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DateTimeFormat {
    #[default]
    Rfc3339,
    // 其他可能的日期时间格
}

/// Stylus 语言定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StylusLanguage {
    pub allow_multiline_strings: bool,
    pub allow_hex_numbers: bool,
    pub datetime_format: DateTimeFormat,
}

impl Language for StylusLanguage {
    const NAME: &'static str = "stylus";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::StylusSyntaxKind;
    type ElementType = crate::kind::StylusSyntaxKind;
    type TypedRoot = ();
}

impl StylusLanguage {
    pub fn new() -> Self {
        Self::standard()
    }

    pub fn standard() -> Self {
        Self { allow_multiline_strings: true, allow_hex_numbers: false, datetime_format: DateTimeFormat::Rfc3339 }
    }

    pub fn lexer(&self) -> StylusLexer<'_> {
        StylusLexer::new(self)
    }
}
