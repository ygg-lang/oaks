use oak_core::{TokenType, UniversalElementRole, UniversalTokenRole};

/// 统一JSON 语法种类（包含节点与词法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum JsonSyntaxKind {
    // 节点种类
    Root,
    Value,
    Object,
    Array,
    String,
    Number,
    Boolean,
    Null,
    ObjectEntry,
    ArrayElement,
    ErrorNode,

    // 词法种类
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Colon,        // :
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    NullLiteral,
    BareKey, // For JSON5
    Whitespace,
    Comment,
    Eof,
    Error,
}

impl TokenType for JsonSyntaxKind {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_comment(&self) -> bool {
        matches!(self, JsonSyntaxKind::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, JsonSyntaxKind::Whitespace)
    }

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Comma | Self::Colon => Punctuation,

            Self::StringLiteral | Self::NumberLiteral | Self::BooleanLiteral | Self::NullLiteral => Literal,

            Self::BareKey => Name,
            Self::Whitespace => Whitespace,
            Self::Comment => Comment,
            Self::Error => Error,
            _ => None,
        }
    }
}

impl oak_core::ElementType for JsonSyntaxKind {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::ErrorNode)
    }

    fn role(&self) -> Self::Role {
        use UniversalElementRole::*;
        match self {
            Self::Root => Root,

            // Hierarchy & Scoping
            Self::Object | Self::Array => Container,

            // Flow Control & Logic (Data logic)
            Self::ObjectEntry | Self::ArrayElement => Statement,

            // Atomic Values
            Self::Value | Self::String | Self::Number | Self::Boolean | Self::Null => Value,

            Self::ErrorNode => Error,
            _ => None,
        }
    }
}
