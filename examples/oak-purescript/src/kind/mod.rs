use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum PurescriptSyntaxKind {
    // Whitespace and comments
    Whitespace,
    Newline,
    Comment,

    // Keywords
    Ado,
    Case,
    Class,
    Data,
    Derive,
    Do,
    Else,
    False,
    Forall,
    Foreign,
    If,
    Import,
    In,
    Infix,
    Infixl,
    Infixr,
    Instance,
    Let,
    Module,
    Newtype,
    Of,
    Then,
    True,
    Type,
    Where,

    // Operators
    Arrow,          // ->
    FatArrow,       // =>
    Backslash,      // \
    Pipe,           // |
    Equal,          // =
    ColonColon,     // ::
    Dot,            // .
    DotDot,         // ..
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    Caret,          // ^
    EqualEqual,     // ==
    NotEqual,       // /=
    Less,           // <
    Greater,        // >
    LessEqual,      // <=
    GreaterEqual,   // >=
    And,            // &&
    Or,             // ||
    Append,         // <>
    Compose,        // <<<
    ComposeFlipped, // >>>
    Apply,          // <$>
    ApplyFlipped,   // <*>
    Bind,           // >>=
    BindFlipped,    // =<<

    // Punctuation
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Semicolon,    // ;
    Colon,        // :
    Question,     // ?
    Exclamation,  // !
    At,           // @
    Underscore,   // _
    Backtick,     // `

    // Literals
    IntLiteral,
    NumberLiteral,
    StringLiteral,
    CharLiteral,
    BooleanLiteral,

    // Identifiers
    Identifier,
    UpperIdentifier,
    Operator,
    QualifiedIdentifier,

    // Special
    Root,
    SourceFile,
    Error,
    Eof,
}

impl TokenType for PurescriptSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for PurescriptSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
