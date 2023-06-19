use crate::{ast::DockerfileRoot, kind::DockerfileSyntaxKind};
use oak_core::Language;

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
    type SyntaxKind = DockerfileSyntaxKind;
    type TypedRoot = DockerfileRoot;
}
