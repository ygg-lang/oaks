use oak_core::UniversalTokenRole;

/// Token types for the APL language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AplTokenType {
    /// Whitespace.
    Whitespace,
    /// A newline.
    Newline,
    /// A comment (⍝).
    Comment,

    /// A string literal.
    StringLiteral,
    /// A number literal.
    NumberLiteral,
    /// An identifier.
    Identifier,

    // Core Symbols
    /// Left arrow (←).
    LeftArrow,
    /// Right arrow (→).
    RightArrow,
    /// Diamond (⋄).
    Diamond,
    /// Quad (⎕).
    Quad,
    /// Quote quad (⍞).
    QuoteQuad,
    /// Rho (⍴).
    Rho,
    /// Iota (⍳).
    Iota,
    /// Epsilon (∊).
    Epsilon,
    /// Up arrow (↑).
    UpArrow,
    /// Down arrow (↓).
    DownArrow,
    /// Del (∇).
    Del,
    /// Delta (∆).
    Delta,
    /// Alpha (⍺).
    Alpha,
    /// Omega (⍵).
    Omega,
    /// Zilde (⍬).
    Zilde,

    // Operators/Functions
    /// Plus (+).
    Plus,
    /// Minus (-).
    Minus,
    /// Times (×).
    Times,
    /// Divide (÷).
    Divide,
    /// Star (*).
    Star,
    /// Log (⍟).
    Log,
    /// Circle (○).
    Circle,
    /// Or (∨).
    Or,
    /// And (∧).
    And,
    /// Not (∼).
    Not,
    /// Nor (⍱).
    Nor,
    /// Nand (⍲).
    Nand,
    /// Equal (=).
    Equal,
    /// Not equal (≠).
    NotEqual,
    /// Less than (<).
    LessThan,
    /// Less equal (≤).
    LessEqual,
    /// Greater equal (≥).
    GreaterEqual,
    /// Greater than (>).
    GreaterThan,
    /// Up stile (⌈).
    UpStile,
    /// Down stile (⌊).
    DownStile,
    /// Bar (|).
    Bar,
    /// Tilde (∼).
    Tilde,
    /// Question (?).
    Question,
    /// Factorial (!).
    Factorial,

    // Operators (Higher Order)
    /// Slash (/).
    Slash,
    /// Backslash (\).
    Backslash,
    /// Slash bar (⌿).
    SlashBar,
    /// Backslash bar (⍀).
    BackslashBar,
    /// Dot (.).
    Dot,
    /// Jot (∘).
    Jot,
    /// Diaeresis (¨).
    Diaeresis,
    /// Power (⍣).
    Power,
    /// Rank (⍤).
    Rank,
    /// Tally (≢).
    Tally,

    // Structural
    /// Left parenthesis (().
    LeftParen,
    /// Right parenthesis ()).
    RightParen,
    /// Left bracket ([).
    LeftBracket,
    /// Right bracket (]).
    RightBracket,
    /// Left brace ({).
    LeftBrace,
    /// Right brace (}).
    RightBrace,
    /// Semicolon (;).
    Semicolon,

    /// End of stream.
    Eof,
    /// An error token.
    Error,
}

impl AplTokenType {
    /// Returns true if this token is an identifier.
    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier | Self::Alpha | Self::Omega)
    }

    /// Returns true if this token is a literal.
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::StringLiteral | Self::NumberLiteral | Self::Zilde)
    }
}

/// Type alias for `AplTokenType`.
pub type TokenType = AplTokenType;

impl oak_core::TokenType for AplTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            Self::Identifier | Self::Alpha | Self::Omega => Name,
            Self::StringLiteral | Self::NumberLiteral | Self::Zilde => Literal,
            Self::Comment => Comment,
            Self::Whitespace | Self::Newline => Whitespace,
            Self::Error => Error,
            Self::Eof => Eof,

            // Core symbols and operators
            Self::LeftArrow
            | Self::RightArrow
            | Self::Plus
            | Self::Minus
            | Self::Times
            | Self::Divide
            | Self::Star
            | Self::Log
            | Self::Circle
            | Self::Or
            | Self::And
            | Self::Not
            | Self::Nor
            | Self::Nand
            | Self::Equal
            | Self::NotEqual
            | Self::LessThan
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::GreaterThan
            | Self::UpStile
            | Self::DownStile
            | Self::Bar
            | Self::Tilde
            | Self::Question
            | Self::Factorial
            | Self::Slash
            | Self::Backslash
            | Self::SlashBar
            | Self::BackslashBar
            | Self::Dot
            | Self::Jot
            | Self::Diaeresis
            | Self::Power
            | Self::Rank
            | Self::Tally
            | Self::Rho
            | Self::Iota
            | Self::Epsilon
            | Self::UpArrow
            | Self::DownArrow => Operator,

            Self::Diamond | Self::Quad | Self::QuoteQuad | Self::Del | Self::Delta | Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Semicolon => Punctuation,
        }
    }
}
