use crate::kind::DotSyntaxKind;
use oak_core::Language;

/// DOT 语言配置（Graphviz#[derive(Debug, Clone)]
pub struct DotLanguage {
    /// 是否启用严格模式
    pub strict_mode: bool,
    /// 是否允许有向    
    pub allow_digraph: bool,
}

impl Default for DotLanguage {
    fn default() -> Self {
        Self { strict_mode: false, allow_digraph: true }
    }
}

impl Language for DotLanguage {
    type SyntaxKind = DotSyntaxKind;
}
