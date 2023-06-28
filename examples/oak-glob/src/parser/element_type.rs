use oak_core::language::{ElementType, UniversalElementRole};

/// Element types for glob pattern syntax.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobElementType {
    /// Root element.
    Root,
    /// Comment element.
    Comment,
    /// Rule element.
    Rule,
}

impl ElementType for GlobElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Comment => UniversalElementRole::Documentation,
            Self::Rule => UniversalElementRole::Statement,
        }
    }
}

impl From<super::super::lexer::token_type::GlobTokenType> for GlobElementType {
    fn from(token_type: super::super::lexer::token_type::GlobTokenType) -> Self {
        match token_type {
            super::super::lexer::token_type::GlobTokenType::Comment => Self::Comment,
            super::super::lexer::token_type::GlobTokenType::Rule => Self::Rule,
            _ => Self::Rule, // For Whitespace and Eof, default to Rule
        }
    }
}
