use crate::lexer::token_type::ValaTokenType;
use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum ValaElementType {
    SourceFile,
    Root,
    Namespace,
    Class,
    Interface,
    Struct,
    Enum,
    Method,
    Field,
    Property,
    Parameter,
    Block,
    Statement,
    Expression,
    Type,
    Error,

    // Token-derived elements
    Token(ValaTokenType),
}

impl From<ValaTokenType> for ValaElementType {
    fn from(token: ValaTokenType) -> Self {
        Self::Token(token)
    }
}

impl ElementType for ValaElementType {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::SourceFile | Self::Root)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile | Self::Root => UniversalElementRole::Root,
            Self::Namespace | Self::Class | Self::Interface | Self::Struct | Self::Enum | Self::Method | Self::Field | Self::Property => UniversalElementRole::Definition,
            Self::Block | Self::Parameter => UniversalElementRole::Container,
            Self::Statement => UniversalElementRole::Statement,
            Self::Expression | Self::Type => UniversalElementRole::Expression,
            Self::Error => UniversalElementRole::Error,
            Self::Token(_) => UniversalElementRole::None,
        }
    }
}

impl oak_core::language::TokenType for ValaElementType {
    type Role = oak_core::UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Token(ValaTokenType::Eof);

    fn is_ignored(&self) -> bool {
        match self {
            Self::Token(t) => {
                use oak_core::TokenType;
                t.is_ignored()
            }
            _ => false,
        }
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Token(t) => {
                use oak_core::TokenType;
                t.role()
            }
            _ => oak_core::UniversalTokenRole::None,
        }
    }
}
