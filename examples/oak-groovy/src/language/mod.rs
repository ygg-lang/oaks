use crate::kind::GroovySyntaxKind;
use oak_core::{Language, lexer::common::CommentConfig};

/// Groovy 语言配置
#[derive(Debug, Clone)]
pub struct GroovyLanguage {
    pub comments: CommentConfig,
}

impl Default for GroovyLanguage {
    fn default() -> Self {
        Self { comments: CommentConfig { line_markers: &[], block_markers: &[], nested_blocks: false } }
    }
}

impl Language for GroovyLanguage {
    type SyntaxKind = GroovySyntaxKind;
}

impl GroovyLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}
