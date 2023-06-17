use crate::{lexer::TwigLexer, syntax::TwigSyntaxKind};
use oak_core::Language;

/// Twig 模板引擎配置
#[derive(Debug, Clone, Copy)]
pub enum TwigMode {
    Template,
    Expression,
    // 其他可能的模式
}

/// Twig 语言定义
#[derive(Debug, Clone)]
pub struct TwigLanguage {
    pub allow_raw_blocks: bool,
    pub allow_custom_tags: bool,
    pub mode: TwigMode,
}

impl Language for TwigLanguage {
    type SyntaxKind = TwigSyntaxKind;
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
}
