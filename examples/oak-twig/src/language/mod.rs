use crate::{ast::TwigRoot, lexer::TwigLexer, parser::TwigParser};
use oak_core::{Language, LanguageCategory};

/// Twig 模板引擎配置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub enum TwigMode {
    #[default]
    Template,
    Expression,
    // 其他可能的模式
}

/// Twig 语言定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct TwigLanguage {
    pub allow_raw_blocks: bool,
    pub allow_custom_tags: bool,
    pub mode: TwigMode,
}

impl Language for TwigLanguage {
    const NAME: &'static str = "twig";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::kind::TwigSyntaxKind;
    type ElementType = crate::kind::TwigSyntaxKind;
    type TypedRoot = TwigRoot;
}

impl TwigLanguage {
    pub fn new() -> Self {
        Self::standard()
    }

    pub fn standard() -> Self {
        Self { allow_raw_blocks: true, allow_custom_tags: false, mode: TwigMode::Template }
    }

    pub fn lexer(&self) -> TwigLexer<'_> {
        TwigLexer::new(self)
    }

    pub fn parser(&self) -> TwigParser<'_> {
        TwigParser::new(self)
    }
}
