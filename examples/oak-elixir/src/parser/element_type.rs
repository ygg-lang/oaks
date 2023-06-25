use crate::lexer::token_type::ElixirTokenType;
use oak_core::{ElementType, UniversalElementRole};

/// Element types for Elixir AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum ElixirElementType {
    /// Root element.
    Root,
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,
    /// Identifier.
    Identifier,
    /// Atom.
    Atom,
    /// Variable.
    Variable,
    /// Number.
    Number,
    /// Float.
    Float,
    /// String.
    String,
    /// Character.
    Character,
    /// Sigil.
    Sigil,

    /// `after`
    After,
    /// `and`
    And,
    /// `case`
    Case,
    /// `catch`
    Catch,
    /// `cond`
    Cond,
    /// `def`
    Def,
    /// `defp`
    Defp,
    /// `defmodule`
    Defmodule,
    /// `defstruct`
    Defstruct,
    /// `defprotocol`
    Defprotocol,
    /// `defimpl`
    Defimpl,
    /// `defmacro`
    Defmacro,
    /// `defmacrop`
    Defmacrop,
    /// `do`
    Do,
    /// `else`
    Else,
    /// `elsif`
    Elsif,
    /// `end`
    End,
    /// `false`
    False,
    /// `fn`
    Fn,
    /// `if`
    If,
    /// `in`
    In,
    /// `not`
    Not,
    /// `or`
    Or,
    /// `receive`
    Receive,
    /// `rescue`
    Rescue,
    /// `true`
    True,
    /// `try`
    Try,
    /// `unless`
    Unless,
    /// `when`
    When,
    /// `with`
    With,

    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `.`
    Dot,
    /// `,`
    Comma,
    /// `;`
    Semicolon,
    /// `:`
    Colon,
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `->`
    Arrow,
    /// `|`
    Pipe,

    /// Module definition.
    ModuleDefinition,
    /// Function definition.
    FunctionDefinition,
    /// Call expression.
    CallExpression,
    /// Binary expression.
    BinaryExpression,
    /// Unary expression.
    UnaryExpression,
    /// Literal expression.
    LiteralExpression,
    /// Identifier expression.
    IdentifierExpression,
    /// Attribute expression.
    AttributeExpression,
    /// Access expression.
    AccessExpression,
    /// List literal.
    ListLiteral,
    /// Tuple literal.
    TupleLiteral,
    /// Map literal.
    MapLiteral,
    /// Keyword literal.
    KeywordLiteral,
    /// Block expression.
    BlockExpression,
    /// Match expression.
    MatchExpression,
    /// `@`
    At,
    /// `%`
    Percent,

    /// Error element.
    Error,
}

impl ElementType for ElixirElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<ElixirTokenType> for ElixirElementType {
    fn from(t: ElixirTokenType) -> Self {
        type T = ElixirTokenType;
        match t {
            T::Root => Self::Root,
            T::Whitespace => Self::Whitespace,
            T::Newline => Self::Newline,
            T::Comment => Self::Comment,
            T::Identifier => Self::Identifier,
            T::Atom => Self::Atom,
            T::Variable => Self::Variable,
            T::Number => Self::Number,
            T::Float => Self::Float,
            T::String => Self::String,
            T::Character => Self::Character,
            T::Sigil => Self::Sigil,
            T::After => Self::After,
            T::And => Self::And,
            T::Case => Self::Case,
            T::Catch => Self::Catch,
            T::Cond => Self::Cond,
            T::Def => Self::Def,
            T::Defp => Self::Defp,
            T::Defmodule => Self::Defmodule,
            T::Defstruct => Self::Defstruct,
            T::Defprotocol => Self::Defprotocol,
            T::Defimpl => Self::Defimpl,
            T::Defmacro => Self::Defmacro,
            T::Defmacrop => Self::Defmacrop,
            T::Do => Self::Do,
            T::Else => Self::Else,
            T::Elsif => Self::Elsif,
            T::End => Self::End,
            T::False => Self::False,
            T::Fn => Self::Fn,
            T::If => Self::If,
            T::In => Self::In,
            T::Not => Self::Not,
            T::Or => Self::Or,
            T::Receive => Self::Receive,
            T::Rescue => Self::Rescue,
            T::True => Self::True,
            T::Try => Self::Try,
            T::Unless => Self::Unless,
            T::When => Self::When,
            T::With => Self::With,
            T::Plus => Self::Plus,
            T::Minus => Self::Minus,
            T::Mul => Self::Mul,
            T::Div => Self::Div,
            T::Dot => Self::Dot,
            T::Comma => Self::Comma,
            T::Semicolon => Self::Semicolon,
            T::Colon => Self::Colon,
            T::LeftParen => Self::LeftParen,
            T::RightParen => Self::RightParen,
            T::LeftBrace => Self::LeftBrace,
            T::RightBrace => Self::RightBrace,
            T::LeftBracket => Self::LeftBracket,
            T::RightBracket => Self::RightBracket,
            T::Arrow => Self::Arrow,
            T::Pipe => Self::Pipe,

            T::Eq => Self::MatchExpression,
            T::EqEq => Self::BinaryExpression,
            T::Ne => Self::BinaryExpression,
            T::Lt => Self::BinaryExpression,
            T::Le => Self::BinaryExpression,
            T::Gt => Self::BinaryExpression,
            T::Ge => Self::BinaryExpression,
            T::AndAnd => Self::BinaryExpression,
            T::OrOr => Self::BinaryExpression,
            T::Bang => Self::UnaryExpression,
            T::Concat => Self::BinaryExpression,
            T::PlusPlus => Self::BinaryExpression,
            T::MinusMinus => Self::BinaryExpression,
            T::Pipeline => Self::BinaryExpression,
            T::At => Self::At,
            T::Percent => Self::Percent,
            T::LeftDoubleBracket => Self::LeftBracket,
            T::RightDoubleBracket => Self::RightBracket,

            T::Error => Self::Error,
        }
    }
}
