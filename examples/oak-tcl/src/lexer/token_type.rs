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
        unsafe { std::mem::transmute(element) }
    }
}
