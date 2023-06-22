use crate::{ast::TailwindRoot, lexer::TailwindLexer, parser::TailwindParser};
use oak_core::{Language, LanguageCategory};

/// Tailwind 引擎配置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub enum TailwindMode {
    #[default]
    Template,
    Expression,
    // 其他可能的模式
}

/// Tailwind 语言定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct TailwindLanguage {
    pub allow_raw_blocks: bool,
    pub allow_custom_tags: bool,
    pub mode: TailwindMode,
}

impl Language for TailwindLanguage {
    const NAME: &'static str = "tailwind";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::kind::TailwindSyntaxKind;
    type ElementType = crate::kind::TailwindSyntaxKind;
    type TypedRoot = TailwindRoot;
}

impl TailwindLanguage {
    pub fn new() -> Self {
        Self::standard()
    }

    pub fn standard() -> Self {
        Self { allow_raw_blocks: true, allow_custom_tags: false, mode: TailwindMode::Template }
    }

    pub fn lexer(&self) -> TailwindLexer<'_> {
        TailwindLexer::new(self)
    }

    pub fn parser(&self) -> TailwindParser<'_> {
        TailwindParser::new(self)
    }
}
