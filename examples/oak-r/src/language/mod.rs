use crate::{ast::RRoot, kind::RSyntaxKind};
use oak_core::{Language, SyntaxKind};

/// R 语言定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RLanguage;

impl SyntaxKind for RSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::Newline)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Error)
    }
}

impl Language for RLanguage {
    type SyntaxKind = RSyntaxKind;
    type TypedRoot = RRoot;
}

impl Default for RLanguage {
    fn default() -> Self {
        Self
    }
}
