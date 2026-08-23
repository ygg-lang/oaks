use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};

/// A token type alias for Tcl tokens.
pub type TclToken = Token<TclTokenType>;

impl TokenType for TclTokenType {
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

/// Represents the different types of tokens in the Tcl language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TclTokenType {
    /// Root node of the AST.
    Root,
    /// A command node.
    Command,
    /// A word node.
    Word,
    /// A simple word node.
    SimpleWord,
    /// A variable word node.
    VariableWord,
    /// A script word node.
    ScriptWord,
    /// A braced word node.
    BracedWord,

    /// A numeric literal.
    Number,
    /// A string literal.
    StringLiteral,
    /// An identifier.
    Identifier,

    /// The `if` keyword.
    If,
    /// The `else` keyword.
    Else,
    /// The `elseif` keyword.
    ElseIf,
    /// The `for` keyword.
    For,
    /// The `while` keyword.
    While,
    /// The `foreach` keyword.
    ForEach,
    /// The `proc` keyword.
    Proc,
    /// The `return` keyword.
    Return,
    /// The `break` keyword.
    Break,
    /// The `continue` keyword.
    Continue,
    /// The `set` keyword.
    Set,
    /// The `unset` keyword.
    Unset,
    /// The `global` keyword.
    Global,
    /// The `upvar` keyword.
    Upvar,
    /// The `variable` keyword.
    Variable,

    /// The `+` operator.
    Plus,
    /// The `-` operator.
    Minus,
    /// The `*` operator.
    Star,
    /// The `/` operator.
    Slash,
    /// The `%` operator.
    Percent,
    /// The `==` operator.
    Equal,
    /// The `!=` operator.
    NotEqual,
    /// The `<` operator.
    Less,
    /// The `>` operator.
    Greater,
    /// The `<=` operator.
    LessEqual,
    /// The `>=` operator.
    GreaterEqual,
    /// The `&` operator.
    Ampersand,
    /// The `&&` operator.
    AmpersandAmpersand,
    /// The `|` operator.
    Pipe,
    /// The `||` operator.
    PipePipe,
    /// The `!` operator.
    Exclamation,

    /// The `(` punctuation.
    LeftParen,
    /// The `)` punctuation.
    RightParen,
    /// The `[` punctuation.
    LeftBracket,
    /// The `]` punctuation.
    RightBracket,
    /// The `{` punctuation.
    LeftBrace,
    /// The `}` punctuation.
    RightBrace,
    /// The `;` punctuation.
    Semicolon,
    /// The `,` punctuation.
    Comma,
    /// The `$` punctuation.
    Dollar,

    /// Whitespace token.
    Whitespace,
    /// Newline token.
    Newline,
    /// Comment token.
    Comment,
    /// Error token.
    Error,
    /// End of file token.
    Eof,
}

impl From<TclTokenType> for UniversalElementRole {
    fn from(kind: TclTokenType) -> Self {
        match kind {
            TclTokenType::Root => UniversalElementRole::Root,
            TclTokenType::Command => UniversalElementRole::Expression,
            TclTokenType::Word | TclTokenType::SimpleWord | TclTokenType::VariableWord | TclTokenType::ScriptWord | TclTokenType::BracedWord => UniversalElementRole::Expression,
            TclTokenType::Identifier => UniversalElementRole::Name,
            TclTokenType::Number | TclTokenType::StringLiteral => UniversalElementRole::Value,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::parser::element_type::TclElementType> for TclTokenType {
    fn from(element: crate::parser::element_type::TclElementType) -> Self {
        match element {
            crate::parser::element_type::TclElementType::Root => TclTokenType::Root,
            crate::parser::element_type::TclElementType::Command => TclTokenType::Command,
            crate::parser::element_type::TclElementType::Word => TclTokenType::Word,
            crate::parser::element_type::TclElementType::SimpleWord => TclTokenType::SimpleWord,
            crate::parser::element_type::TclElementType::VariableWord => TclTokenType::VariableWord,
            crate::parser::element_type::TclElementType::ScriptWord => TclTokenType::ScriptWord,
            crate::parser::element_type::TclElementType::BracedWord => TclTokenType::BracedWord,
            crate::parser::element_type::TclElementType::Number => TclTokenType::Number,
            crate::parser::element_type::TclElementType::StringLiteral => TclTokenType::StringLiteral,
            crate::parser::element_type::TclElementType::Identifier => TclTokenType::Identifier,
            crate::parser::element_type::TclElementType::If => TclTokenType::If,
            crate::parser::element_type::TclElementType::Else => TclTokenType::Else,
            crate::parser::element_type::TclElementType::ElseIf => TclTokenType::ElseIf,
            crate::parser::element_type::TclElementType::For => TclTokenType::For,
            crate::parser::element_type::TclElementType::While => TclTokenType::While,
            crate::parser::element_type::TclElementType::ForEach => TclTokenType::ForEach,
            crate::parser::element_type::TclElementType::Proc => TclTokenType::Proc,
            crate::parser::element_type::TclElementType::Return => TclTokenType::Return,
            crate::parser::element_type::TclElementType::Break => TclTokenType::Break,
            crate::parser::element_type::TclElementType::Continue => TclTokenType::Continue,
            crate::parser::element_type::TclElementType::Set => TclTokenType::Set,
            crate::parser::element_type::TclElementType::Unset => TclTokenType::Unset,
            crate::parser::element_type::TclElementType::Global => TclTokenType::Global,
            crate::parser::element_type::TclElementType::Upvar => TclTokenType::Upvar,
            crate::parser::element_type::TclElementType::Variable => TclTokenType::Variable,
            crate::parser::element_type::TclElementType::Plus => TclTokenType::Plus,
            crate::parser::element_type::TclElementType::Minus => TclTokenType::Minus,
            crate::parser::element_type::TclElementType::Star => TclTokenType::Star,
            crate::parser::element_type::TclElementType::Slash => TclTokenType::Slash,
            crate::parser::element_type::TclElementType::Percent => TclTokenType::Percent,
            crate::parser::element_type::TclElementType::Equal => TclTokenType::Equal,
            crate::parser::element_type::TclElementType::NotEqual => TclTokenType::NotEqual,
            crate::parser::element_type::TclElementType::Less => TclTokenType::Less,
            crate::parser::element_type::TclElementType::Greater => TclTokenType::Greater,
            crate::parser::element_type::TclElementType::LessEqual => TclTokenType::LessEqual,
            crate::parser::element_type::TclElementType::GreaterEqual => TclTokenType::GreaterEqual,
            crate::parser::element_type::TclElementType::Ampersand => TclTokenType::Ampersand,
            crate::parser::element_type::TclElementType::AmpersandAmpersand => TclTokenType::AmpersandAmpersand,
            crate::parser::element_type::TclElementType::Pipe => TclTokenType::Pipe,
            crate::parser::element_type::TclElementType::PipePipe => TclTokenType::PipePipe,
            crate::parser::element_type::TclElementType::Exclamation => TclTokenType::Exclamation,
            crate::parser::element_type::TclElementType::LeftParen => TclTokenType::LeftParen,
            crate::parser::element_type::TclElementType::RightParen => TclTokenType::RightParen,
            crate::parser::element_type::TclElementType::LeftBracket => TclTokenType::LeftBracket,
            crate::parser::element_type::TclElementType::RightBracket => TclTokenType::RightBracket,
            crate::parser::element_type::TclElementType::LeftBrace => TclTokenType::LeftBrace,
            crate::parser::element_type::TclElementType::RightBrace => TclTokenType::RightBrace,
            crate::parser::element_type::TclElementType::Semicolon => TclTokenType::Semicolon,
            crate::parser::element_type::TclElementType::Comma => TclTokenType::Comma,
            crate::parser::element_type::TclElementType::Dollar => TclTokenType::Dollar,
            crate::parser::element_type::TclElementType::Whitespace => TclTokenType::Whitespace,
            crate::parser::element_type::TclElementType::Newline => TclTokenType::Newline,
            crate::parser::element_type::TclElementType::Comment => TclTokenType::Comment,
            crate::parser::element_type::TclElementType::Error => TclTokenType::Error,
            crate::parser::element_type::TclElementType::Eof => TclTokenType::Eof,
            _ => TclTokenType::Error,
        }
    }
}
