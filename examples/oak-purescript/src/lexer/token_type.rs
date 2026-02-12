use oak_core::{Token, TokenType, UniversalTokenRole};

/// Type alias for a PureScript token.
pub type PurescriptToken = Token<PurescriptTokenType>;

/// Token types for PureScript.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PurescriptTokenType {
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,
    /// `ado`
    Ado,
    /// `case`
    Case,
    /// `class`
    Class,
    /// `data`
    Data,
    /// `derive`
    Derive,
    /// `do`
    Do,
    /// `else`
    Else,
    /// `false`
    False,
    /// `forall`
    Forall,
    /// `foreign`
    Foreign,
    /// `if`
    If,
    /// `import`
    Import,
    /// `in`
    In,
    /// `infix`
    Infix,
    /// `infixl`
    Infixl,
    /// `infixr`
    Infixr,
    /// `instance`
    Instance,
    /// `let`
    Let,
    /// `module`
    Module,
    /// `newtype`
    Newtype,
    /// `of`
    Of,
    /// `then`
    Then,
    /// `true`
    True,
    /// `type`
    Type,
    /// `where`
    Where,
    /// `->`
    Arrow,
    /// `=>`
    FatArrow,
    /// `\`
    Backslash,
    /// `|`
    Pipe,
    /// `=`
    Equal,
    /// `::`
    ColonColon,
    /// `.`
    Dot,
    /// `..`
    DotDot,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `^`
    Caret,
    /// `==`
    EqualEqual,
    /// `/=`
    NotEqual,
    /// `<`
    Less,
    /// `>`
    Greater,
    /// `<=`
    LessEqual,
    /// `>=`
    GreaterEqual,
    /// `&&`
    And,
    /// `||`
    Or,
    /// `<>`
    Append,
    /// `<<<`
    Compose,
    /// `>>>`
    ComposeFlipped,
    /// `$`
    Apply,
    /// `#`
    ApplyFlipped,
    /// `>>=`
    Bind,
    /// `=<<`
    BindFlipped,
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
    /// `,`
    Comma,
    /// `;`
    Semicolon,
    /// `:`
    Colon,
    /// `?`
    Question,
    /// `!`
    Exclamation,
    /// `@`
    At,
    /// `_`
    Underscore,
    /// `` ` ``
    Tick,
    /// Integer literal.
    IntLiteral,
    /// Floating point literal.
    NumberLiteral,
    /// String literal.
    StringLiteral,
    /// Character literal.
    CharLiteral,
    /// Boolean literal.
    BooleanLiteral,
    /// Lowercase identifier.
    Identifier,
    /// Uppercase identifier.
    UpperIdentifier,
    /// Operator.
    Operator,
    /// Qualified identifier.
    QualifiedIdentifier,
    /// Root node.
    Root,
    /// Source file node.
    SourceFile,
    /// Error token.
    Error,
    /// End of file.
    Eof,
}

impl From<crate::parser::element_type::PurescriptElementType> for PurescriptTokenType {
    fn from(element: crate::parser::element_type::PurescriptElementType) -> Self {
        use crate::parser::element_type::PurescriptElementType as E;
        match element {
            E::Whitespace => Self::Whitespace,
            E::Newline => Self::Newline,
            E::Comment => Self::Comment,
            E::Ado => Self::Ado,
            E::Case => Self::Case,
            E::Class => Self::Class,
            E::Data => Self::Data,
            E::Derive => Self::Derive,
            E::Do => Self::Do,
            E::Else => Self::Else,
            E::False => Self::False,
            E::Forall => Self::Forall,
            E::Foreign => Self::Foreign,
            E::If => Self::If,
            E::Import => Self::Import,
            E::In => Self::In,
            E::Infix => Self::Infix,
            E::Infixl => Self::Infixl,
            E::Infixr => Self::Infixr,
            E::Instance => Self::Instance,
            E::Let => Self::Let,
            E::Module => Self::Module,
            E::Newtype => Self::Newtype,
            E::Of => Self::Of,
            E::Then => Self::Then,
            E::True => Self::True,
            E::Type => Self::Type,
            E::Where => Self::Where,
            E::Arrow => Self::Arrow,
            E::FatArrow => Self::FatArrow,
            E::Backslash => Self::Backslash,
            E::Pipe => Self::Pipe,
            E::Equal => Self::Equal,
            E::ColonColon => Self::ColonColon,
            E::Dot => Self::Dot,
            E::DotDot => Self::DotDot,
            E::Plus => Self::Plus,
            E::Minus => Self::Minus,
            E::Star => Self::Star,
            E::Slash => Self::Slash,
            E::Percent => Self::Percent,
            E::Caret => Self::Caret,
            E::EqualEqual => Self::EqualEqual,
            E::NotEqual => Self::NotEqual,
            E::Less => Self::Less,
            E::Greater => Self::Greater,
            E::LessEqual => Self::LessEqual,
            E::GreaterEqual => Self::GreaterEqual,
            E::And => Self::And,
            E::Or => Self::Or,
            E::Append => Self::Append,
            E::Compose => Self::Compose,
            E::ComposeFlipped => Self::ComposeFlipped,
            E::Apply => Self::Apply,
            E::ApplyFlipped => Self::ApplyFlipped,
            E::Bind => Self::Bind,
            E::BindFlipped => Self::BindFlipped,
            E::LeftParen => Self::LeftParen,
            E::RightParen => Self::RightParen,
            E::LeftBrace => Self::LeftBrace,
            E::RightBrace => Self::RightBrace,
            E::LeftBracket => Self::LeftBracket,
            E::RightBracket => Self::RightBracket,
            E::Comma => Self::Comma,
            E::Semicolon => Self::Semicolon,
            E::Colon => Self::Colon,
            E::Question => Self::Question,
            E::Exclamation => Self::Exclamation,
            E::At => Self::At,
            E::Underscore => Self::Underscore,
            E::Tick => Self::Tick,
            E::IntLiteral => Self::IntLiteral,
            E::NumberLiteral => Self::NumberLiteral,
            E::StringLiteral => Self::StringLiteral,
            E::CharLiteral => Self::CharLiteral,
            E::BooleanLiteral => Self::BooleanLiteral,
            E::Identifier => Self::Identifier,
            E::UpperIdentifier => Self::UpperIdentifier,
            E::Operator => Self::Operator,
            E::QualifiedIdentifier => Self::QualifiedIdentifier,
            E::Root => Self::Root,
            E::SourceFile => Self::SourceFile,
            E::Error => Self::Error,
            E::Eof => Self::Eof,
            _ => Self::Error,
        }
    }
}

impl TokenType for PurescriptTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        match self {
            Self::Whitespace | Self::Newline | Self::Comment => true,
            _ => false,
        }
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
