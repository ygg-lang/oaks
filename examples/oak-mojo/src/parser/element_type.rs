use crate::lexer::MojoTokenType;
use oak_core::UniversalElementRole;

/// Element types for the Mojo language parser.
///
/// This enum represents all possible element types in the Mojo language,
/// including tokens mapped from `MojoTokenType`, statement types, expression types,
/// and special node types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MojoElementType {
    // Tokens (mapped from MojoTokenType)
    /// Function keyword `fn`.
    Fn,
    /// Struct keyword `struct`.
    Struct,
    /// Variable keyword `var`.
    Var,
    /// Let keyword `let`.
    Let,
    /// If keyword `if`.
    If,
    /// Else keyword `else`.
    Else,
    /// While keyword `while`.
    While,
    /// For keyword `for`.
    For,
    /// In keyword `in`.
    In,
    /// Return keyword `return`.
    Return,
    /// Break keyword `break`.
    Break,
    /// Continue keyword `continue`.
    Continue,
    /// Import keyword `import`.
    Import,
    /// From keyword `from`.
    From,
    /// Boolean literal `True`.
    True,
    /// Boolean literal `False`.
    False,
    /// None literal.
    None,
    /// Identifier token.
    Identifier,
    /// Integer literal token.
    Integer,
    /// Float literal token.
    Float,
    /// String literal token.
    String,
    /// Plus operator `+`.
    Plus,
    /// Minus operator `-`.
    Minus,
    /// Star operator `*`.
    Star,
    /// Slash operator `/`.
    Slash,
    /// Percent operator `%`.
    Percent,
    /// Assignment operator `=`.
    Equal,
    /// Equality operator `==`.
    EqualEqual,
    /// Inequality operator `!=`.
    NotEqual,
    /// Less than operator `<`.
    Less,
    /// Less than or equal operator `<=`.
    LessEqual,
    /// Greater than operator `>`.
    Greater,
    /// Greater than or equal operator `>=`.
    GreaterEqual,
    /// Logical and operator `and`.
    And,
    /// Logical or operator `or`.
    Or,
    /// Logical not operator `not`.
    Not,
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Comma `,`.
    Comma,
    /// Dot `.`.
    Dot,
    /// Colon `:`.
    Colon,
    /// Semicolon `;`.
    Semicolon,
    /// Arrow operator `->`.
    Arrow,
    /// Whitespace token.
    Whitespace,
    /// Newline token.
    Newline,
    /// Comment token.
    Comment,
    /// Indent token for significant whitespace.
    Indent,
    /// Dedent token for significant whitespace.
    Dedent,
    /// End of stream marker.
    EndOfStream,

    // Statements
    /// Function definition statement.
    FunctionDef,
    /// Struct definition statement.
    StructDef,
    /// Variable declaration statement.
    VariableDecl,
    /// Assignment statement.
    Assignment,
    /// If statement.
    IfStatement,
    /// While statement.
    WhileStatement,
    /// For statement.
    ForStatement,
    /// Return statement.
    ReturnStatement,
    /// Expression statement.
    ExpressionStatement,

    // Expressions
    /// Binary expression.
    BinaryExpr,
    /// Unary expression.
    UnaryExpr,
    /// Function call expression.
    CallExpr,
    /// Literal expression.
    LiteralExpr,
    /// Identifier expression.
    IdentifierExpr,
    /// Member access expression.
    MemberExpr,
    /// List expression.
    ListExpr,

    // Components
    /// Parameter list.
    ParamList,
    /// Argument list.
    ArgList,
    /// Code block.
    Block,

    // Special
    /// Root node of the AST.
    Root,
    /// Grouping expression.
    Grouping,
    /// Error node.
    Error,
}

impl MojoElementType {
    /// Checks if the node is trivia (whitespace, comments, etc.).
    pub fn is_trivia(&self) -> bool {
        matches!(self, MojoElementType::Whitespace | MojoElementType::Newline | MojoElementType::Comment)
    }
}

impl From<MojoTokenType> for MojoElementType {
    fn from(token: MojoTokenType) -> Self {
        match token {
            MojoTokenType::Fn => MojoElementType::Fn,
            MojoTokenType::Struct => MojoElementType::Struct,
            MojoTokenType::Var => MojoElementType::Var,
            MojoTokenType::Let => MojoElementType::Let,
            MojoTokenType::If => MojoElementType::If,
            MojoTokenType::Else => MojoElementType::Else,
            MojoTokenType::While => MojoElementType::While,
            MojoTokenType::For => MojoElementType::For,
            MojoTokenType::In => MojoElementType::In,
            MojoTokenType::Return => MojoElementType::Return,
            MojoTokenType::Break => MojoElementType::Break,
            MojoTokenType::Continue => MojoElementType::Continue,
            MojoTokenType::Import => MojoElementType::Import,
            MojoTokenType::From => MojoElementType::From,
            MojoTokenType::True => MojoElementType::True,
            MojoTokenType::False => MojoElementType::False,
            MojoTokenType::None => MojoElementType::None,
            MojoTokenType::Identifier => MojoElementType::Identifier,
            MojoTokenType::Integer => MojoElementType::Integer,
            MojoTokenType::Float => MojoElementType::Float,
            MojoTokenType::String => MojoElementType::String,
            MojoTokenType::Plus => MojoElementType::Plus,
            MojoTokenType::Minus => MojoElementType::Minus,
            MojoTokenType::Star => MojoElementType::Star,
            MojoTokenType::Slash => MojoElementType::Slash,
            MojoTokenType::Percent => MojoElementType::Percent,
            MojoTokenType::Equal => MojoElementType::Equal,
            MojoTokenType::EqualEqual => MojoElementType::EqualEqual,
            MojoTokenType::NotEqual => MojoElementType::NotEqual,
            MojoTokenType::Less => MojoElementType::Less,
            MojoTokenType::LessEqual => MojoElementType::LessEqual,
            MojoTokenType::Greater => MojoElementType::Greater,
            MojoTokenType::GreaterEqual => MojoElementType::GreaterEqual,
            MojoTokenType::And => MojoElementType::And,
            MojoTokenType::Or => MojoElementType::Or,
            MojoTokenType::Not => MojoElementType::Not,
            MojoTokenType::LeftParen => MojoElementType::LeftParen,
            MojoTokenType::RightParen => MojoElementType::RightParen,
            MojoTokenType::LeftBracket => MojoElementType::LeftBracket,
            MojoTokenType::RightBracket => MojoElementType::RightBracket,
            MojoTokenType::LeftBrace => MojoElementType::LeftBrace,
            MojoTokenType::RightBrace => MojoElementType::RightBrace,
            MojoTokenType::Comma => MojoElementType::Comma,
            MojoTokenType::Dot => MojoElementType::Dot,
            MojoTokenType::Colon => MojoElementType::Colon,
            MojoTokenType::Semicolon => MojoElementType::Semicolon,
            MojoTokenType::Arrow => MojoElementType::Arrow,
            MojoTokenType::Whitespace => MojoElementType::Whitespace,
            MojoTokenType::Newline => MojoElementType::Newline,
            MojoTokenType::Comment => MojoElementType::Comment,
            MojoTokenType::Indent => MojoElementType::Indent,
            MojoTokenType::Dedent => MojoElementType::Dedent,
            MojoTokenType::EndOfStream => MojoElementType::EndOfStream,
            MojoTokenType::Error => MojoElementType::Error,
        }
    }
}

impl oak_core::ElementType for MojoElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        UniversalElementRole::None
    }
}
