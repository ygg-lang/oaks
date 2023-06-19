use oak_core::SyntaxKind;
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
    Error,
    Eof,
}

impl SyntaxKind for PurescriptSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Root)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Root)
    }
}
