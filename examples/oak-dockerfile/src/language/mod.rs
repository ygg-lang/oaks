#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Dockerfile 语言配置
#[derive(Debug, Clone, Copy)]
pub struct DockerfileLanguage {
    /// 是否启用严格模式
    pub strict_mode: bool,
    /// 是否允许多阶段构
    pub multi_stage: bool,
}

impl Default for DockerfileLanguage {
    fn default() -> Self {
        Self { strict_mode: false, multi_stage: true }
    }
}

impl Language for DockerfileLanguage {
    const NAME: &'static str = "dockerfile";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::DockerfileTokenType;
    type ElementType = crate::parser::element_type::DockerfileElementType;
    type TypedRoot = ();
}
