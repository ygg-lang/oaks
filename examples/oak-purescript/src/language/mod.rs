use crate::kind::PurescriptSyntaxKind;
use oak_core::Language;

/// PureScript 语言实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurescriptLanguage {
    /// 语言配置
    pub config: (),
}

impl Default for PurescriptLanguage {
    fn default() -> Self {
        Self { config: () }
    }
}

impl Language for PurescriptLanguage {
    type SyntaxKind = PurescriptSyntaxKind;
    type TypedRoot = crate::ast::SourceFile;
}
