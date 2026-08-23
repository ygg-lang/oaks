use oak_core::{ElementType, UniversalElementRole};

/// Element types for Elm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ElmElementType {
    /// The root node of the AST.
    Root,
    /// A module declaration.
    Module,
    /// An import statement.
    Import,
    /// A custom type declaration.
    TypeDeclaration,
    /// A type alias declaration.
    TypeAlias,
    /// A function declaration.
    FunctionDeclaration,
    /// A generic expression.
    Expression,
    /// A literal value.
    Literal,
    /// An identifier.
    Identifier,
    /// A binary expression.
    BinaryExpression,
    /// A unary expression.
    UnaryExpression,
    /// An `if` expression.
    IfExpression,
    /// A `case` expression.
    CaseExpression,
    /// A `let` expression.
    LetExpression,
    /// A tuple expression.
    TupleExpression,
    /// A list expression.
    ListExpression,
    /// A record expression.
    RecordExpression,
    /// A field access expression.
    FieldExpression,
    /// A lambda expression.
    LambdaExpression,
    /// A type signature.
    TypeSignature,
    /// A value declaration.
    ValueDeclaration,
    /// A pattern.
    Pattern,
    /// An error element.
    Error,
}

impl ElementType for ElmElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Module => UniversalElementRole::Definition,
            Self::Import => UniversalElementRole::Statement,
            Self::FunctionDeclaration => UniversalElementRole::Definition,
            Self::Expression => UniversalElementRole::Expression,
            Self::Identifier => UniversalElementRole::Name,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::ElmTokenType> for ElmElementType {
    fn from(token: crate::lexer::token_type::ElmTokenType) -> Self {
        match token {
            crate::lexer::token_type::ElmTokenType::Root => Self::Root,
            crate::lexer::token_type::ElmTokenType::Identifier => Self::Identifier,
            _ =>         match token {
            crate::lexer::token_type::ElmTokenType::Root => Self::Root,
            crate::lexer::token_type::ElmTokenType::Whitespace => Self::Root,
            crate::lexer::token_type::ElmTokenType::Newline => Self::Root,
            crate::lexer::token_type::ElmTokenType::Comment => Self::Root,
            crate::lexer::token_type::ElmTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::ElmTokenType::Number => Self::Root,
            crate::lexer::token_type::ElmTokenType::Float => Self::Root,
            crate::lexer::token_type::ElmTokenType::String => Self::Root,
            crate::lexer::token_type::ElmTokenType::Char => Self::Root,
            crate::lexer::token_type::ElmTokenType::If => Self::Root,
            crate::lexer::token_type::ElmTokenType::Then => Self::Root,
            crate::lexer::token_type::ElmTokenType::Else => Self::Root,
            crate::lexer::token_type::ElmTokenType::Case => Self::Root,
            crate::lexer::token_type::ElmTokenType::Of => Self::Root,
            crate::lexer::token_type::ElmTokenType::Let => Self::Root,
            crate::lexer::token_type::ElmTokenType::In => Self::Root,
            crate::lexer::token_type::ElmTokenType::Type => Self::Root,
            crate::lexer::token_type::ElmTokenType::Alias => Self::Root,
            crate::lexer::token_type::ElmTokenType::Module => Self::Module,
            crate::lexer::token_type::ElmTokenType::Where => Self::Root,
            crate::lexer::token_type::ElmTokenType::Import => Self::Import,
            crate::lexer::token_type::ElmTokenType::Exposing => Self::Root,
            crate::lexer::token_type::ElmTokenType::As => Self::Root,
            crate::lexer::token_type::ElmTokenType::Port => Self::Root,
            crate::lexer::token_type::ElmTokenType::Plus => Self::Root,
            crate::lexer::token_type::ElmTokenType::Minus => Self::Root,
            crate::lexer::token_type::ElmTokenType::Star => Self::Root,
            crate::lexer::token_type::ElmTokenType::Slash => Self::Root,
            crate::lexer::token_type::ElmTokenType::DoubleSlash => Self::Root,
            crate::lexer::token_type::ElmTokenType::Caret => Self::Root,
            crate::lexer::token_type::ElmTokenType::Percent => Self::Root,
            crate::lexer::token_type::ElmTokenType::Equal => Self::Root,
            crate::lexer::token_type::ElmTokenType::EqualEqual => Self::Root,
            crate::lexer::token_type::ElmTokenType::NotEqual => Self::Root,
            crate::lexer::token_type::ElmTokenType::Less => Self::Root,
            crate::lexer::token_type::ElmTokenType::Greater => Self::Root,
            crate::lexer::token_type::ElmTokenType::LessEqual => Self::Root,
            crate::lexer::token_type::ElmTokenType::GreaterEqual => Self::Root,
            crate::lexer::token_type::ElmTokenType::DoubleAmpersand => Self::Root,
            crate::lexer::token_type::ElmTokenType::DoublePipe => Self::Root,
            crate::lexer::token_type::ElmTokenType::DoublePlus => Self::Root,
            crate::lexer::token_type::ElmTokenType::DoubleLess => Self::Root,
            crate::lexer::token_type::ElmTokenType::DoubleGreater => Self::Root,
            crate::lexer::token_type::ElmTokenType::Arrow => Self::Root,
            crate::lexer::token_type::ElmTokenType::Pipe => Self::Root,
            crate::lexer::token_type::ElmTokenType::PipeGreater => Self::Root,
            crate::lexer::token_type::ElmTokenType::Dot => Self::Root,
            crate::lexer::token_type::ElmTokenType::DoubleDot => Self::Root,
            crate::lexer::token_type::ElmTokenType::TripleDot => Self::Root,
            crate::lexer::token_type::ElmTokenType::Comma => Self::Root,
            crate::lexer::token_type::ElmTokenType::Colon => Self::Root,
            crate::lexer::token_type::ElmTokenType::Semicolon => Self::Root,
            crate::lexer::token_type::ElmTokenType::LeftParen => Self::Root,
            crate::lexer::token_type::ElmTokenType::RightParen => Self::Root,
            crate::lexer::token_type::ElmTokenType::LeftBracket => Self::Root,
            crate::lexer::token_type::ElmTokenType::RightBracket => Self::Root,
            crate::lexer::token_type::ElmTokenType::LeftBrace => Self::Root,
            crate::lexer::token_type::ElmTokenType::RightBrace => Self::Root,
            crate::lexer::token_type::ElmTokenType::Backslash => Self::Root,
            crate::lexer::token_type::ElmTokenType::Bar => Self::Root,
            crate::lexer::token_type::ElmTokenType::Error => Self::Root,
            crate::lexer::token_type::ElmTokenType::Eof => Self::Root,
            _ => Self::Root,
        },
        }
    }
}
