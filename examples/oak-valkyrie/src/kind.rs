pub use crate::lexer::ValkyrieKeywords;
use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum ValkyrieSyntaxKind {
    // Tokens
    Whitespace,
    Newline,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Colon,
    Semicolon,
    Dot,
    Comma,
    Question,
    Bang,
    At,
    Hash,
    Dollar,
    Percent,
    Caret,
    Ampersand,
    Star,
    Plus,
    Minus,
    Eq,
    LessThan,
    GreaterThan,
    Slash,
    Backslash,
    Pipe,
    Tilde,
    Underscore,

    // Compound Operators
    EqEq,
    NotEq,
    LessEq,
    GreaterEq,
    PlusPlus,
    MinusMinus,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    AndAnd,
    OrOr,
    LeftShift,
    RightShift,
    Arrow,

    // Additional operators
    CaretEq,
    AndEq,
    OrEq,
    ShlEq,
    ShrEq,
    Lt,
    Gt,
    Ne,
    Le,
    Ge,
    Shl,
    Shr,
    Not,
    And,
    Or,

    // Keywords
    Keyword(ValkyrieKeywords),

    // Identifiers
    Identifier,

    // Literals
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BoolLiteral,

    // Comments
    LineComment,
    BlockComment,

    // Elements (AST Nodes)
    ValkyrieRoot, // Added this as it's used in parse.rs
    SourceFile,
    Namespace,
    Class,
    Widget,
    ApplyBlock,
    Micro,
    Mezzo,
    Type,
    ParameterList,
    Parameter,
    BlockExpression,
    LetStatement,
    ExpressionStatement,
    IdentifierExpression,
    LiteralExpression,
    BooleanLiteral,
    AnonymousClass,
    ObjectExpression,
    ParenthesizedExpression,
    UnaryExpression,
    BinaryExpression,
    CallExpression,
    FieldExpression,
    IndexExpression,
    IfExpression,
    MatchExpression,
    MatchArm,
    LoopExpression,
    ReturnExpression,
    BreakExpression,
    ContinueExpression,
    YieldExpression,
    RaiseExpression,
    CatchExpression,

    // Special
    Error,
    Eof,
}

impl TokenType for ValkyrieSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Keyword(_) => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral | Self::BoolLiteral => UniversalTokenRole::Literal,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Eq
            | Self::EqEq
            | Self::NotEq
            | Self::LessThan
            | Self::GreaterThan
            | Self::LessEq
            | Self::GreaterEq
            | Self::AndAnd
            | Self::OrOr
            | Self::Bang
            | Self::Ampersand
            | Self::Pipe
            | Self::Caret
            | Self::Tilde
            | Self::LeftShift
            | Self::RightShift
            | Self::Arrow => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace => UniversalTokenRole::Punctuation,
            Self::Comma | Self::Semicolon | Self::Colon | Self::Dot => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for ValkyrieSyntaxKind {
    type Role = oak_core::UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile | Self::ValkyrieRoot => UniversalElementRole::Root,
            Self::Namespace => UniversalElementRole::Container,
            Self::Class => UniversalElementRole::Definition,
            Self::Widget => UniversalElementRole::Definition,
            Self::Micro => UniversalElementRole::Definition,
            Self::Mezzo => UniversalElementRole::Definition,
            Self::ParameterList => UniversalElementRole::Container,
            Self::Parameter => UniversalElementRole::Binding,
            Self::BlockExpression => UniversalElementRole::Expression,
            Self::LetStatement => UniversalElementRole::Statement,
            Self::ExpressionStatement => UniversalElementRole::Statement,
            Self::IdentifierExpression => UniversalElementRole::Expression,
            Self::LiteralExpression => UniversalElementRole::Expression,
            Self::BooleanLiteral => UniversalElementRole::Expression,
            Self::AnonymousClass => UniversalElementRole::Expression,
            Self::ApplyBlock | Self::ObjectExpression => UniversalElementRole::Expression,
            Self::ParenthesizedExpression => UniversalElementRole::Expression,
            Self::UnaryExpression => UniversalElementRole::Expression,
            Self::BinaryExpression => UniversalElementRole::Expression,
            Self::CallExpression => UniversalElementRole::Call,
            Self::FieldExpression => UniversalElementRole::Expression,
            Self::IndexExpression => UniversalElementRole::Expression,
            Self::IfExpression => UniversalElementRole::Expression,
            Self::MatchExpression => UniversalElementRole::Expression,
            Self::MatchArm => UniversalElementRole::Container,
            Self::LoopExpression => UniversalElementRole::Expression,
            Self::ReturnExpression => UniversalElementRole::Expression,
            Self::BreakExpression => UniversalElementRole::Expression,
            Self::ContinueExpression => UniversalElementRole::Expression,
            Self::YieldExpression => UniversalElementRole::Expression,
            Self::RaiseExpression => UniversalElementRole::Expression,
            Self::CatchExpression => UniversalElementRole::Expression,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::SourceFile | Self::ValkyrieRoot)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}

impl From<ValkyrieSyntaxKind> for UniversalTokenRole {
    fn from(kind: ValkyrieSyntaxKind) -> Self {
        <ValkyrieSyntaxKind as TokenType>::role(&kind)
    }
}
