pub use crate::lexer::ValkyrieKeywords;
use oak_core::{TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    ColonColon,
    ColonEq,
    Semicolon,
    Dot,
    Comma,
    Question,
    Bang,
    At,
    Bolt,
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
    PipeGreater, // |>

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
    StringPart,
    InterpolationStart,   // {
    InterpolationEnd,     // }
    TemplateControlStart, // <%
    TemplateControlEnd,   // %>
    TemplateControlContent,
    TemplateCommentStart, // <#
    TemplateCommentEnd,   // #>
    CharLiteral,
    BoolLiteral,

    // Comments
    LineComment,
    BlockComment,

    // Elements (AST Nodes)
    ValkyrieRoot,
    SourceFile,
    Namespace,
    NamePath,
    UsingStatement,
    Class,
    Flags,
    Enums,
    EffectDefinition,
    Variant,
    Trait,
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
    PathExpression,
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
    ResumeExpression,
    Interpolation,
    Attribute,
    Pattern,
    GenericParameterList,
    GenericArgumentList,

    // Special
    Error,
    Eof,
}

pub type ValkyrieTokenType = ValkyrieSyntaxKind;

impl TokenType for ValkyrieSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Keyword(ValkyrieKeywords::Is) | Self::Keyword(ValkyrieKeywords::In) => UniversalTokenRole::Operator,
            Self::Keyword(_) => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::StringPart | Self::CharLiteral | Self::BoolLiteral => UniversalTokenRole::Literal,
            Self::InterpolationStart | Self::InterpolationEnd | Self::TemplateControlStart | Self::TemplateControlEnd => UniversalTokenRole::Punctuation,
            Self::At
            | Self::Bolt
            | Self::Hash
            | Self::Dollar
            | Self::Percent
            | Self::Caret
            | Self::Ampersand
            | Self::Star
            | Self::Plus
            | Self::Minus
            | Self::Eq
            | Self::LessThan
            | Self::GreaterThan
            | Self::Slash
            | Self::Backslash
            | Self::Pipe
            | Self::Tilde
            | Self::Underscore => UniversalTokenRole::Operator,
            Self::EqEq
            | Self::NotEq
            | Self::LessEq
            | Self::GreaterEq
            | Self::PlusPlus
            | Self::MinusMinus
            | Self::PlusEq
            | Self::MinusEq
            | Self::StarEq
            | Self::SlashEq
            | Self::PercentEq
            | Self::AndAnd
            | Self::OrOr
            | Self::LeftShift
            | Self::RightShift
            | Self::Colon
            | Self::Arrow => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace => UniversalTokenRole::Punctuation,
            Self::Comma | Self::Semicolon | Self::ColonColon | Self::Dot => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }
}
