use oak_core::{ElementType, Parser, UniversalElementRole};

/// Element types for the R language parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum RElementType {
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,

    /// Comment.
    Comment,

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

    /// Identifier.
    Identifier,

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

    /// `+` operator.
    Plus,
    /// `-` operator.
    Minus,
    /// `*` operator.
    Star,
    /// `/` operator.
    Slash,
    /// `%` operator.
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
    /// `|>` operator.
    Pipe,
    /// Custom operator.
    Operator,

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

    /// Root node.
    Root,

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

    /// Error token.
    Error,
    /// End of file token.
    Eof,
}

impl RElementType {
    /// Returns true if this element type is trivia (whitespace, newline, or comment).
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }
}

impl ElementType for RElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::RTokenType> for RElementType {
    fn from(token: crate::lexer::token_type::RTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
