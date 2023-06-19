use crate::kind::MarkdownSyntaxKind;
use oak_core::Language;

#[derive(Debug)]
pub struct MarkdownLanguage {
    pub allow_math: bool,
}

impl Language for MarkdownLanguage {
    type SyntaxKind = MarkdownSyntaxKind;
    type TypedRoot = (); // 暂时使用空类型，后续可以定义具体的AST根节点类型
}

impl Default for MarkdownLanguage {
    fn default() -> Self {
        Self { allow_math: false }
    }
}
