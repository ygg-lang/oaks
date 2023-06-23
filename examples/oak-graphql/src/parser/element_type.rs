use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GraphQLElementType {
    StringLiteral,
    IntLiteral,
    FloatLiteral,
    BooleanLiteral,
    NullLiteral,
    Name,
    QueryKeyword,
    MutationKeyword,
    SubscriptionKeyword,
    FragmentKeyword,
    OnKeyword,
    TypeKeyword,
    InterfaceKeyword,
    UnionKeyword,
    ScalarKeyword,
    EnumKeyword,
    InputKeyword,
    ExtendKeyword,
    SchemaKeyword,
    DirectiveKeyword,
    ImplementsKeyword,
    RepeatsKeyword,
    Spread,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    Semicolon,
    Pipe,
    Ampersand,
    Equals,
    Exclamation,
    At,
    Dollar,
    Whitespace,
    Comment,
    SourceFile,
    Newline,
    Eof,
    Error,
}

impl GraphQLElementType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::QueryKeyword
                | Self::MutationKeyword
                | Self::SubscriptionKeyword
                | Self::FragmentKeyword
                | Self::OnKeyword
                | Self::TypeKeyword
                | Self::InterfaceKeyword
                | Self::UnionKeyword
                | Self::ScalarKeyword
                | Self::EnumKeyword
                | Self::InputKeyword
                | Self::ExtendKeyword
                | Self::SchemaKeyword
                | Self::DirectiveKeyword
                | Self::ImplementsKeyword
                | Self::RepeatsKeyword
        )
    }
}

impl ElementType for GraphQLElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::GraphQLTokenType> for GraphQLElementType {
    fn from(token: crate::lexer::token_type::GraphQLTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
