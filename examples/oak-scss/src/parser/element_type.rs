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
        unsafe { std::mem::transmute(token) }
    }
}
