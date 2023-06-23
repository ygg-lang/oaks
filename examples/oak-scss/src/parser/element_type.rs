use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum ScssElementType {
    // Keywords
    Import,
    Include,
    Mixin,
    Function,
    Return,
    If,
    Else,
    For,
    While,
    Each,
    In,
    True,
    False,
    Null,

    // Operators
    EqEq,
    Ne,
    Le,
    Ge,
    AndAnd,
    OrOr,
    Eq,
    Lt,
    Gt,
    And,
    Or,
    Xor,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bang,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Colon,
    Comma,
    Dot,
    Hash,
    At,
    Dollar,

    // Literals and Identifiers
    Identifier,
    IntegerLiteral,
    StringLiteral,

    // Others
    Whitespace,
    Newline,
    Comment,
    Eof,
    Error,

    // Composite Elements
    SourceFile,
    RuleSet,
    Selector,
    Declaration,
    Property,
    ValueNode,
    Block,
    MixinDeclaration,
    FunctionDeclaration,
    IncludeStatement,
    ImportStatement,
    VariableDeclaration,
    IfStatement,
    ForStatement,
    EachStatement,
    WhileStatement,
}

impl ElementType for ScssElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::RuleSet | Self::MixinDeclaration | Self::FunctionDeclaration => UniversalElementRole::Definition,
            Self::Block => UniversalElementRole::Container,
            Self::Declaration | Self::Property => UniversalElementRole::Attribute,
            Self::Selector => UniversalElementRole::Name,
            Self::ValueNode => UniversalElementRole::Value,
            Self::ImportStatement | Self::IncludeStatement | Self::VariableDeclaration | Self::IfStatement | Self::ForStatement | Self::EachStatement | Self::WhileStatement => UniversalElementRole::Statement,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::ScssTokenType> for ScssElementType {
    fn from(token: crate::lexer::token_type::ScssTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
