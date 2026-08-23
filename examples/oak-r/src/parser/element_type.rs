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
                match token {
            crate::lexer::token_type::RTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::RTokenType::Newline => Self::Newline,
            crate::lexer::token_type::RTokenType::Comment => Self::Comment,
            crate::lexer::token_type::RTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::RTokenType::IntegerLiteral => Self::IntegerLiteral,
            crate::lexer::token_type::RTokenType::FloatLiteral => Self::FloatLiteral,
            crate::lexer::token_type::RTokenType::BooleanLiteral => Self::BooleanLiteral,
            crate::lexer::token_type::RTokenType::NullLiteral => Self::NullLiteral,
            crate::lexer::token_type::RTokenType::Inf => Self::Inf,
            crate::lexer::token_type::RTokenType::NaN => Self::NaN,
            crate::lexer::token_type::RTokenType::NA => Self::NA,
            crate::lexer::token_type::RTokenType::NaInteger => Self::NaInteger,
            crate::lexer::token_type::RTokenType::NaReal => Self::NaReal,
            crate::lexer::token_type::RTokenType::NaComplex => Self::NaComplex,
            crate::lexer::token_type::RTokenType::NaCharacter => Self::NaCharacter,
            crate::lexer::token_type::RTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::RTokenType::If => Self::If,
            crate::lexer::token_type::RTokenType::Else => Self::Else,
            crate::lexer::token_type::RTokenType::For => Self::For,
            crate::lexer::token_type::RTokenType::In => Self::In,
            crate::lexer::token_type::RTokenType::While => Self::While,
            crate::lexer::token_type::RTokenType::Repeat => Self::Repeat,
            crate::lexer::token_type::RTokenType::Next => Self::Next,
            crate::lexer::token_type::RTokenType::Break => Self::Break,
            crate::lexer::token_type::RTokenType::Function => Self::Function,
            crate::lexer::token_type::RTokenType::Return => Self::Return,
            crate::lexer::token_type::RTokenType::True => Self::True,
            crate::lexer::token_type::RTokenType::False => Self::False,
            crate::lexer::token_type::RTokenType::Null => Self::Null,
            crate::lexer::token_type::RTokenType::Plus => Self::Plus,
            crate::lexer::token_type::RTokenType::Minus => Self::Minus,
            crate::lexer::token_type::RTokenType::Star => Self::Star,
            crate::lexer::token_type::RTokenType::Slash => Self::Slash,
            crate::lexer::token_type::RTokenType::Percent => Self::Percent,
            crate::lexer::token_type::RTokenType::Caret => Self::Caret,
            crate::lexer::token_type::RTokenType::Equal => Self::Equal,
            crate::lexer::token_type::RTokenType::EqualEqual => Self::EqualEqual,
            crate::lexer::token_type::RTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::RTokenType::Less => Self::Less,
            crate::lexer::token_type::RTokenType::Greater => Self::Greater,
            crate::lexer::token_type::RTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::RTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::RTokenType::And => Self::And,
            crate::lexer::token_type::RTokenType::Or => Self::Or,
            crate::lexer::token_type::RTokenType::Not => Self::Not,
            crate::lexer::token_type::RTokenType::AndAnd => Self::AndAnd,
            crate::lexer::token_type::RTokenType::OrOr => Self::OrOr,
            crate::lexer::token_type::RTokenType::Tilde => Self::Tilde,
            crate::lexer::token_type::RTokenType::LeftArrow => Self::LeftArrow,
            crate::lexer::token_type::RTokenType::RightArrow => Self::RightArrow,
            crate::lexer::token_type::RTokenType::DoubleLeftArrow => Self::DoubleLeftArrow,
            crate::lexer::token_type::RTokenType::DoubleRightArrow => Self::DoubleRightArrow,
            crate::lexer::token_type::RTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::RTokenType::Operator => Self::Operator,
            crate::lexer::token_type::RTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::RTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::RTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::RTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::RTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::RTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::RTokenType::Comma => Self::Comma,
            crate::lexer::token_type::RTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::RTokenType::Colon => Self::Colon,
            crate::lexer::token_type::RTokenType::DoubleColon => Self::DoubleColon,
            crate::lexer::token_type::RTokenType::TripleColon => Self::TripleColon,
            crate::lexer::token_type::RTokenType::Dot => Self::Dot,
            crate::lexer::token_type::RTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::RTokenType::At => Self::At,
            crate::lexer::token_type::RTokenType::Question => Self::Question,
            crate::lexer::token_type::RTokenType::Root => Self::Root,
            crate::lexer::token_type::RTokenType::Assignment => Self::Assignment,
            crate::lexer::token_type::RTokenType::BinaryExpression => Self::BinaryExpression,
            crate::lexer::token_type::RTokenType::UnaryExpression => Self::UnaryExpression,
            crate::lexer::token_type::RTokenType::LiteralExpression => Self::LiteralExpression,
            crate::lexer::token_type::RTokenType::IdentifierExpression => Self::IdentifierExpression,
            crate::lexer::token_type::RTokenType::CallExpression => Self::CallExpression,
            crate::lexer::token_type::RTokenType::GroupingExpression => Self::GroupingExpression,
            crate::lexer::token_type::RTokenType::BlockExpression => Self::BlockExpression,
            crate::lexer::token_type::RTokenType::IfExpression => Self::IfExpression,
            crate::lexer::token_type::RTokenType::WhileExpression => Self::WhileExpression,
            crate::lexer::token_type::RTokenType::ForExpression => Self::ForExpression,
            crate::lexer::token_type::RTokenType::RepeatExpression => Self::RepeatExpression,
            crate::lexer::token_type::RTokenType::FunctionDefinition => Self::FunctionDefinition,
            crate::lexer::token_type::RTokenType::IndexExpression => Self::IndexExpression,
            crate::lexer::token_type::RTokenType::MemberExpression => Self::MemberExpression,
            crate::lexer::token_type::RTokenType::ArgumentList => Self::ArgumentList,
            crate::lexer::token_type::RTokenType::ParameterList => Self::ParameterList,
            crate::lexer::token_type::RTokenType::Error => Self::Error,
            crate::lexer::token_type::RTokenType::Eof => Self::Eof,
            _ => Self::Error,
        }
    }
}
