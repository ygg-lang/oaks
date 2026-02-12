use oak_core::{Token, TokenType, UniversalTokenRole};

/// Token type for Lua.
pub type LuaToken = Token<LuaTokenType>;

impl TokenType for LuaTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::EndOfStream;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::And
            | Self::Break
            | Self::Do
            | Self::Else
            | Self::Elseif
            | Self::End
            | Self::False
            | Self::For
            | Self::Function
            | Self::Goto
            | Self::If
            | Self::In
            | Self::Local
            | Self::Nil
            | Self::Not
            | Self::Or
            | Self::Repeat
            | Self::Return
            | Self::Then
            | Self::True
            | Self::Until
            | Self::While => UniversalTokenRole::Keyword,

            Self::Identifier => UniversalTokenRole::Name,
            Self::Number => UniversalTokenRole::Literal,
            Self::String => UniversalTokenRole::Literal,

            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Caret
            | Self::Hash
            | Self::Ampersand
            | Self::Tilde
            | Self::Pipe
            | Self::LtLt
            | Self::GtGt
            | Self::SlashSlash
            | Self::EqEq
            | Self::TildeEq
            | Self::LtEq
            | Self::GtEq
            | Self::Lt
            | Self::Gt
            | Self::Eq
            | Self::DotDot
            | Self::DotDotDot => UniversalTokenRole::Operator,

            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::ColonColon | Self::Semicolon | Self::Colon | Self::Comma | Self::Dot => UniversalTokenRole::Punctuation,

            Self::Comment => UniversalTokenRole::Comment,
            _ => UniversalTokenRole::None,
        }
    }
}

/// Token types for Lua.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum LuaTokenType {
    /// Root node.
    Root,
    // Keywords
    /// The `and` keyword.
    And,
    /// The `break` keyword.
    Break,
    /// The `do` keyword.
    Do,
    /// The `else` keyword.
    Else,
    /// The `elseif` keyword.
    Elseif,
    /// The `end` keyword.
    End,
    /// The `false` keyword.
    False,
    /// The `for` keyword.
    For,
    /// The `function` keyword.
    Function,
    /// The `goto` keyword.
    Goto,
    /// The `if` keyword.
    If,
    /// The `in` keyword.
    In,
    /// The `local` keyword.
    Local,
    /// The `nil` keyword.
    Nil,
    /// The `not` keyword.
    Not,
    /// The `or` keyword.
    Or,
    /// The `repeat` keyword.
    Repeat,
    /// The `return` keyword.
    Return,
    /// The `then` keyword.
    Then,
    /// The `true` keyword.
    True,
    /// The `until` keyword.
    Until,
    /// The `while` keyword.
    While,

    // Identifiers and literals
    /// An identifier.
    Identifier,
    /// A numeric literal.
    Number,
    /// A string literal.
    String,

    // Operators
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
    /// The `^` operator.
    Caret,
    /// The `#` operator.
    Hash,
    /// The `&` operator.
    Ampersand,
    /// The `~` operator.
    Tilde,
    /// The `|` operator.
    Pipe,
    /// The `<<` operator.
    LtLt,
    /// The `>>` operator.
    GtGt,
    /// The `//` operator.
    SlashSlash,
    /// The `==` operator.
    EqEq,
    /// The `~=` operator.
    TildeEq,
    /// The `<=` operator.
    LtEq,
    /// The `>=` operator.
    GtEq,
    /// The `<` operator.
    Lt,
    /// The `>` operator.
    Gt,
    /// The `=` operator.
    Eq,

    // Delimiters
    /// The `(` punctuation.
    LeftParen,
    /// The `)` punctuation.
    RightParen,
    /// The `{` punctuation.
    LeftBrace,
    /// The `}` punctuation.
    RightBrace,
    /// The `[` punctuation.
    LeftBracket,
    /// The `]` punctuation.
    RightBracket,
    /// The `::` punctuation.
    ColonColon,
    /// The `;` punctuation.
    Semicolon,
    /// The `:` punctuation.
    Colon,
    /// The `,` punctuation.
    Comma,
    /// The `.` punctuation.
    Dot,
    /// The `..` punctuation.
    DotDot,
    /// The `...` punctuation.
    DotDotDot,

    // Whitespace and comments
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// A comment.
    Comment,

    // Special markers
    /// End of stream marker.
    EndOfStream,
    /// Error marker.
    Error,
}
