use crate::kind::ElixirSyntaxKind;
use oak_core::Language;

/// Elixir 语言配置
#[derive(Debug, Clone)]
pub struct ElixirLanguage {
    /// 是否启用严格模式
    pub strict_mode: bool,
    /// 是否允许 Unicode 标识符
    pub allow_unicode: bool,
}

impl Default for ElixirLanguage {
    fn default() -> Self {
        Self { strict_mode: false, allow_unicode: true }
    }
}

impl Language for ElixirLanguage {
    type SyntaxKind = ElixirSyntaxKind;

    fn error_kind() -> Self::SyntaxKind {
        ElixirSyntaxKind::Error
    }

    fn eof_kind() -> Self::SyntaxKind {
        ElixirSyntaxKind::Eof
    }

    fn is_trivia(kind: Self::SyntaxKind) -> bool {
        kind.is_trivia()
    }
}
