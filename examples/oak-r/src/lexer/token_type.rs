use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};

pub type RToken = Token<RTokenType>;

impl RTokenType {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }
}

impl TokenType for RTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

use crate::language::RLanguage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
/// R language token types.
pub enum RTokenType {
    // Whitespace and newlines
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,

    // Comments
    /// Comment.
    Comment,

    // Literals
    /// String literal.
    StringLiteral,
    /// Integer literal.
    IntegerLiteral,
    /// Float literal.
    FloatLiteral,
    /// Boolean literal.
    BooleanLiteral,
    /// Null literal.
    NullLiteral,
    /// Infinity.
    Inf,
    /// Not a Number.
    NaN,
    /// Not Available.
    NA,
    /// Not Available (Integer).
    NaInteger,
    /// Not Available (Real).
    NaReal,
    /// Not Available (Complex).
    NaComplex,
    /// Not Available (Character).
    NaCharacter,

    // Identifiers
    /// Identifier.
    Identifier,

    // Keywords
    /// `if` keyword.
    If,
    /// `else` keyword.
    Else,
    /// `for` keyword.
    For,
    /// `in` keyword.
    In,
    /// `while` keyword.
    While,
    /// `repeat` keyword.
    Repeat,
    /// `next` keyword.
    Next,
    /// `break` keyword.
    Break,
    /// `function` keyword.
    Function,
    /// `return` keyword.
    Return,
    /// `TRUE` keyword.
    True,
    /// `FALSE` keyword.
    False,
    /// `NULL` keyword.
    Null,

    // Operators
    /// `+` operator.
    Plus,
    /// `-` operator.
    Minus,
    /// `*` operator.
    Star,
    /// `/` operator.
    Slash,
    /// `%` operator (e.g. `%%` or `%*%`).
    Percent,
    /// `^` operator.
    Caret,
    /// `=` operator.
    Equal,
    /// `==` operator.
    EqualEqual,
    /// `!=` operator.
    NotEqual,
    /// `<` operator.
    Less,
    /// `>` operator.
    Greater,
    /// `<=` operator.
    LessEqual,
    /// `>=` operator.
    GreaterEqual,
    /// `&` operator.
    And,
    /// `|` operator.
    Or,
    /// `!` operator.
    Not,
    /// `&&` operator.
    AndAnd,
    /// `||` operator.
    OrOr,
    /// `~` operator.
    Tilde,
    /// `<-` operator.
    LeftArrow,
    /// `->` operator.
    RightArrow,
    /// `<<-` operator.
    DoubleLeftArrow,
    /// `->>` operator.
    DoubleRightArrow,
    /// `|` or `|>` operator.
    Pipe,
    /// Custom operator.
    Operator,

    // Punctuation
    /// `(` symbol.
    LeftParen,
    /// `)` symbol.
    RightParen,
    /// `[` symbol.
    LeftBracket,
    /// `]` symbol.
    RightBracket,
    /// `{` symbol.
    LeftBrace,
    /// `}` symbol.
    RightBrace,
    /// `,` symbol.
    Comma,
    /// `;` symbol.
    Semicolon,
    /// `:` symbol.
    Colon,
    /// `::` symbol.
    DoubleColon,
    /// `:::` symbol.
    TripleColon,
    /// `.` symbol.
    Dot,
    /// `$` symbol.
    Dollar,
    /// `@` symbol.
    At,
    /// `?` symbol.
    Question,

    // Root node
    /// Root node.
    Root,

    // Expressions
    /// Assignment expression.
    Assignment,
    /// Binary expression.
    BinaryExpression,
    /// Unary expression.
    UnaryExpression,
    /// Literal expression.
    LiteralExpression,
    /// Identifier expression.
    IdentifierExpression,
    /// Call expression.
    CallExpression,
    /// Grouping expression.
    GroupingExpression,
    /// Block expression.
    BlockExpression,
    /// If expression.
    IfExpression,
    /// While expression.
    WhileExpression,
    /// For expression.
    ForExpression,
    /// Repeat expression.
    RepeatExpression,
    /// Function definition.
    FunctionDefinition,
    /// Index expression.
    IndexExpression,
    /// Member expression.
    MemberExpression,
    /// Argument list.
    ArgumentList,
    /// Parameter list.
    ParameterList,

    // Error and EOF
    /// Error token.
    Error,
    /// End of file token.
    Eof,
}

pub type RNode<'a> = oak_core::tree::RedNode<'a, RLanguage>;
