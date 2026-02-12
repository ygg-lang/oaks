#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// Language configuration for Dockerfile.
#[derive(Debug, Clone, Copy)]
pub struct DockerfileLanguage {
    /// Whether to enable strict mode.
    pub strict_mode: bool,
    /// Whether to allow multi-stage builds.
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
    type TypedRoot = crate::ast::DockerfileRoot;
}
