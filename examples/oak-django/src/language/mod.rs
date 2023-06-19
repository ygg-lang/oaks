use crate::{ast::DjangoRoot, kind::DjangoSyntaxKind};
use oak_core::Language;

/// Django 模板语言配置
#[derive(Debug, Clone)]
pub struct DjangoLanguage {
    /// 是否启用严格模式
    pub strict_mode: bool,
    /// 是否允许自定义标签
    pub allow_custom_tags: bool,
}

impl Default for DjangoLanguage {
    fn default() -> Self {
        Self { strict_mode: false, allow_custom_tags: true }
    }
}

impl Language for DjangoLanguage {
    type SyntaxKind = DjangoSyntaxKind;
    type TypedRoot = DjangoRoot;
}
