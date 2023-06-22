use oak_core::{Language, LanguageCategory};

/// DOT 语言配置（Graphviz）
#[derive(Debug, Clone, Copy)]
pub struct DotLanguage {
    /// 是否启用严格模式
    pub strict_mode: bool,
    /// 是否允许有向图
    pub allow_digraph: bool,
}

impl Default for DotLanguage {
    fn default() -> Self {
        Self { strict_mode: false, allow_digraph: true }
    }
}

impl Language for DotLanguage {
    const NAME: &'static str = "dot";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::DotSyntaxKind;
    type ElementType = crate::kind::DotSyntaxKind;
    type TypedRoot = ();
}
