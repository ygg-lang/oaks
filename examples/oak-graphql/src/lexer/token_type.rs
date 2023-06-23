use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type GraphQLToken = Token<GraphQLTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GraphQLTokenType {
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

impl GraphQLTokenType {
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

impl TokenType for GraphQLTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
