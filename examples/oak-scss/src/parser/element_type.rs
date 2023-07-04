use oak_core::{ElementType, UniversalElementRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum ScssElementType {
    // Keywords
    /// '@import' keyword.
    Import,
    /// '@include' keyword.
    Include,
    /// '@mixin' keyword.
    Mixin,
    /// '@function' keyword.
    Function,
    /// '@return' keyword.
    Return,
    /// '@if' keyword.
    If,
    /// '@else' keyword.
    Else,
    /// '@for' keyword.
    For,
    /// '@while' keyword.
    While,
    /// '@each' keyword.
    Each,
    /// 'in' keyword.
    In,
    /// 'true' keyword.
    True,
    /// 'false' keyword.
    False,
    /// 'null' keyword.
    Null,

    // Operators
    /// '==' operator.
    EqEq,
    /// '!=' operator.
    Ne,
    /// '<=' operator.
    Le,
    /// '>=' operator.
    Ge,
    /// '&&' operator.
    AndAnd,
    /// '||' operator.
    OrOr,
    /// '=' operator.
    Eq,
    /// '<' operator.
    Lt,
    /// '>' operator.
    Gt,
    /// 'and' operator.
    And,
    /// 'or' operator.
    Or,
    /// 'xor' operator.
    Xor,
    /// '+' operator.
    Plus,
    /// '-' operator.
    Minus,
    /// '*' operator.
    Star,
    /// '/' operator.
    Slash,
    /// '%' operator.
    Percent,
    /// '!' operator.
    Bang,

    // Punctuation
    /// '(' punctuation.
    LeftParen,
    /// ')' punctuation.
    RightParen,
    /// '{' punctuation.
    LeftBrace,
    /// '}' punctuation.
    RightBrace,
    /// '[' punctuation.
    LeftBracket,
    /// ']' punctuation.
    RightBracket,
    /// ';' punctuation.
    Semicolon,
    /// ':' punctuation.
    Colon,
    /// ',' punctuation.
    Comma,
    /// '.' punctuation.
    Dot,
    /// '#' punctuation.
    Hash,
    /// '@' punctuation.
    At,
    /// '$' punctuation.
    Dollar,

    // Literals and Identifiers
    /// Identifier.
    Identifier,
    /// Integer literal.
    IntegerLiteral,
    /// String literal.
    StringLiteral,

    // Others
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,
    /// End of file.
    Eof,
    /// Error token.
    Error,

    // Composite Elements
    /// Source file.
    SourceFile,
    /// Rule set.
    RuleSet,
    /// Selector.
    Selector,
    /// Declaration.
    Declaration,
    /// Property.
    Property,
    /// Value node.
    ValueNode,
    /// Block.
    Block,
    /// Mixin declaration.
    MixinDeclaration,
    /// Function declaration.
    FunctionDeclaration,
    /// Include statement.
    IncludeStatement,
    /// Import statement.
    ImportStatement,
    /// Variable declaration.
    VariableDeclaration,
    /// If statement.
    IfStatement,
    /// For statement.
    ForStatement,
    /// Each statement.
    EachStatement,
    /// While statement.
    WhileStatement,
    /// Return statement.
    ReturnStatement,
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
                match token {
            crate::lexer::token_type::ScssTokenType::Import => Self::Import,
            crate::lexer::token_type::ScssTokenType::Include => Self::Include,
            crate::lexer::token_type::ScssTokenType::Mixin => Self::Mixin,
            crate::lexer::token_type::ScssTokenType::Function => Self::Function,
            crate::lexer::token_type::ScssTokenType::Return => Self::Return,
            crate::lexer::token_type::ScssTokenType::If => Self::If,
            crate::lexer::token_type::ScssTokenType::Else => Self::Else,
            crate::lexer::token_type::ScssTokenType::For => Self::For,
            crate::lexer::token_type::ScssTokenType::While => Self::While,
            crate::lexer::token_type::ScssTokenType::Each => Self::Each,
            crate::lexer::token_type::ScssTokenType::In => Self::In,
            crate::lexer::token_type::ScssTokenType::True => Self::True,
            crate::lexer::token_type::ScssTokenType::False => Self::False,
            crate::lexer::token_type::ScssTokenType::Null => Self::Null,
            crate::lexer::token_type::ScssTokenType::EqEq => Self::EqEq,
            crate::lexer::token_type::ScssTokenType::Ne => Self::Ne,
            crate::lexer::token_type::ScssTokenType::Le => Self::Le,
            crate::lexer::token_type::ScssTokenType::Ge => Self::Ge,
            crate::lexer::token_type::ScssTokenType::AndAnd => Self::AndAnd,
            crate::lexer::token_type::ScssTokenType::OrOr => Self::OrOr,
            crate::lexer::token_type::ScssTokenType::Eq => Self::Eq,
            crate::lexer::token_type::ScssTokenType::Lt => Self::Lt,
            crate::lexer::token_type::ScssTokenType::Gt => Self::Gt,
            crate::lexer::token_type::ScssTokenType::And => Self::And,
            crate::lexer::token_type::ScssTokenType::Or => Self::Or,
            crate::lexer::token_type::ScssTokenType::Xor => Self::Xor,
            crate::lexer::token_type::ScssTokenType::Plus => Self::Plus,
            crate::lexer::token_type::ScssTokenType::Minus => Self::Minus,
            crate::lexer::token_type::ScssTokenType::Star => Self::Star,
            crate::lexer::token_type::ScssTokenType::Slash => Self::Slash,
            crate::lexer::token_type::ScssTokenType::Percent => Self::Percent,
            crate::lexer::token_type::ScssTokenType::Bang => Self::Bang,
            crate::lexer::token_type::ScssTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::ScssTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::ScssTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::ScssTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::ScssTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::ScssTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::ScssTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::ScssTokenType::Colon => Self::Colon,
            crate::lexer::token_type::ScssTokenType::Comma => Self::Comma,
            crate::lexer::token_type::ScssTokenType::Dot => Self::Dot,
            crate::lexer::token_type::ScssTokenType::Hash => Self::Hash,
            crate::lexer::token_type::ScssTokenType::At => Self::At,
            crate::lexer::token_type::ScssTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::ScssTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::ScssTokenType::IntegerLiteral => Self::IntegerLiteral,
            crate::lexer::token_type::ScssTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::ScssTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::ScssTokenType::Newline => Self::Newline,
            crate::lexer::token_type::ScssTokenType::Comment => Self::Comment,
            crate::lexer::token_type::ScssTokenType::Eof => Self::Eof,
            crate::lexer::token_type::ScssTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
