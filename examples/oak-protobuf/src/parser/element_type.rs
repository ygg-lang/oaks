use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProtobufElementType {
    Root,
    // 基础 kind
    Whitespace,
    Newline,
    Comment,
    Error,
    Eof,

    // 语法结构
    SyntaxDef,
    PackageDef,
    ImportDef,
    OptionDef,
    MessageDef,
    EnumDef,
    ServiceDef,
    RpcDef,
    FieldDef,
    MapFieldDef,
    OneofDef,

    // 标记
    Identifier,
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
}

impl ElementType for ProtobufElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::ProtobufTokenType> for ProtobufElementType {
    fn from(token: crate::lexer::token_type::ProtobufTokenType) -> Self {
        use crate::lexer::token_type::ProtobufTokenType as T;
        match token {
            T::Whitespace => Self::Whitespace,
            T::Newline => Self::Newline,
            T::Comment => Self::Comment,
            T::Identifier => Self::Identifier,
            T::StringLiteral => Self::StringLiteral,
            T::NumberLiteral => Self::NumberLiteral,
            T::BooleanLiteral => Self::BooleanLiteral,
            T::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
