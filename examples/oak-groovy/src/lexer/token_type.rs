//! Token types for the Groovy language.

use oak_core::{Token, TokenType, UniversalTokenRole};

/// Type alias for a Groovy token.
pub type GroovyToken = Token<GroovyTokenType>;

impl GroovyTokenType {
    /// Returns `true` if the token is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::AbstractKeyword
                | Self::AsKeyword
                | Self::AssertKeyword
                | Self::BreakKeyword
                | Self::CaseKeyword
                | Self::CatchKeyword
                | Self::ClassKeyword
                | Self::ConstKeyword
                | Self::ContinueKeyword
                | Self::DefKeyword
                | Self::DefaultKeyword
                | Self::DoKeyword
                | Self::ElseKeyword
                | Self::EnumKeyword
                | Self::ExtendsKeyword
                | Self::FinalKeyword
                | Self::FinallyKeyword
                | Self::ForKeyword
                | Self::GotoKeyword
                | Self::IfKeyword
                | Self::ImplementsKeyword
                | Self::ImportKeyword
                | Self::InKeyword
                | Self::InstanceofKeyword
                | Self::InterfaceKeyword
                | Self::NativeKeyword
                | Self::NewKeyword
                | Self::PackageKeyword
                | Self::PrivateKeyword
                | Self::ProtectedKeyword
                | Self::PublicKeyword
                | Self::ReturnKeyword
                | Self::StaticKeyword
                | Self::StrictfpKeyword
                | Self::SuperKeyword
                | Self::SwitchKeyword
                | Self::SynchronizedKeyword
                | Self::ThisKeyword
                | Self::ThrowKeyword
                | Self::ThrowsKeyword
                | Self::TraitKeyword
                | Self::TransientKeyword
                | Self::TryKeyword
                | Self::VoidKeyword
                | Self::VolatileKeyword
                | Self::WhileKeyword
        )
    }
}

impl TokenType for GroovyTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        if self.is_keyword() {
            return Keyword;
        }
        match self {
            Self::IntLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral | Self::BooleanLiteral | Self::NullLiteral => Literal,
            Self::Identifier => Name,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Power
            | Self::Assign
            | Self::PlusAssign
            | Self::MinusAssign
            | Self::StarAssign
            | Self::SlashAssign
            | Self::PercentAssign
            | Self::PowerAssign
            | Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::Spaceship
            | Self::LogicalAnd
            | Self::LogicalOr
            | Self::LogicalNot
            | Self::BitAnd
            | Self::BitOr
            | Self::BitXor
            | Self::BitNot
            | Self::LeftShift
            | Self::RightShift
            | Self::UnsignedRightShift
            | Self::Increment
            | Self::Decrement
            | Self::Question
            | Self::Elvis
            | Self::SafeNavigation => Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Comma | Self::Period | Self::Semicolon | Self::Colon | Self::At => Punctuation,
            Self::Whitespace | Self::Newline => Whitespace,
            Self::Comment => Comment,
            _ => None,
        }
    }
}

/// Token types for Groovy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum GroovyTokenType {
    /// Root node.
    Root,
    /// Source file.
    SourceFile,
    // --- Literals ---
    /// Integer literal.
    IntLiteral,
    /// Floating point literal.
    FloatLiteral,
    /// String literal.
    StringLiteral,
    /// Character literal.
    CharLiteral,
    /// Boolean literal.
    BooleanLiteral,
    /// Null literal.
    NullLiteral,

    // --- Identifiers ---
    /// Identifier.
    Identifier,

    // --- Keywords ---
    /// `abstract` keyword.
    AbstractKeyword,
    /// `as` keyword.
    AsKeyword,
    /// `assert` keyword.
    AssertKeyword,
    /// `break` keyword.
    BreakKeyword,
    /// `case` keyword.
    CaseKeyword,
    /// `catch` keyword.
    CatchKeyword,
    /// `class` keyword.
    ClassKeyword,
    /// `const` keyword.
    ConstKeyword,
    /// `continue` keyword.
    ContinueKeyword,
    /// `def` keyword.
    DefKeyword,
    /// `default` keyword.
    DefaultKeyword,
    /// `do` keyword.
    DoKeyword,
    /// `else` keyword.
    ElseKeyword,
    /// `enum` keyword.
    EnumKeyword,
    /// `extends` keyword.
    ExtendsKeyword,
    /// `final` keyword.
    FinalKeyword,
    /// `finally` keyword.
    FinallyKeyword,
    /// `for` keyword.
    ForKeyword,
    /// `goto` keyword.
    GotoKeyword,
    /// `if` keyword.
    IfKeyword,
    /// `implements` keyword.
    ImplementsKeyword,
    /// `import` keyword.
    ImportKeyword,
    /// `in` keyword.
    InKeyword,
    /// `instanceof` keyword.
    InstanceofKeyword,
    /// `interface` keyword.
    InterfaceKeyword,
    /// `native` keyword.
    NativeKeyword,
    /// `new` keyword.
    NewKeyword,
    /// `package` keyword.
    PackageKeyword,
    /// `private` keyword.
    PrivateKeyword,
    /// `protected` keyword.
    ProtectedKeyword,
    /// `public` keyword.
    PublicKeyword,
    /// `return` keyword.
    ReturnKeyword,
    /// `static` keyword.
    StaticKeyword,
    /// `strictfp` keyword.
    StrictfpKeyword,
    /// `super` keyword.
    SuperKeyword,
    /// `switch` keyword.
    SwitchKeyword,
    /// `synchronized` keyword.
    SynchronizedKeyword,
    /// `this` keyword.
    ThisKeyword,
    /// `throw` keyword.
    ThrowKeyword,
    /// `throws` keyword.
    ThrowsKeyword,
    /// `trait` keyword.
    TraitKeyword,
    /// `transient` keyword.
    TransientKeyword,
    /// `try` keyword.
    TryKeyword,
    /// `void` keyword.
    VoidKeyword,
    /// `volatile` keyword.
    VolatileKeyword,
    /// `while` keyword.
    WhileKeyword,

    // --- Operators ---
    /// Plus `+`.
    Plus,
    /// Minus `-`.
    Minus,
    /// Star `*`.
    Star,
    /// Slash `/`.
    Slash,
    /// Percent `%`.
    Percent,
    /// Power `**`.
    Power,

    /// Assign `=`.
    Assign,
    /// Plus assign `+=`.
    PlusAssign,
    /// Minus assign `-=`.
    MinusAssign,
    /// Star assign `*=`.
    StarAssign,
    /// Slash assign `/=`.
    SlashAssign,
    /// Percent assign `%=`.
    PercentAssign,
    /// Power assign `**=`.
    PowerAssign,

    /// Equal `==`.
    Equal,
    /// Not equal `!=`.
    NotEqual,
    /// Less than `<`.
    Less,
    /// Greater than `>`.
    Greater,
    /// Less than or equal `<=`.
    LessEqual,
    /// Greater than or equal `>=`.
    GreaterEqual,
    /// Spaceship operator `<=>`.
    Spaceship,

    /// Logical AND `&&`.
    LogicalAnd,
    /// Logical OR `||`.
    LogicalOr, // ||
    /// Logical NOT `!`.
    LogicalNot, // !

    /// Bitwise AND `&`.
    BitAnd, // &
    /// Bitwise OR `|`.
    BitOr, // |
    /// Bitwise XOR `^`.
    BitXor, // ^
    /// Bitwise NOT `~`.
    BitNot, // ~
    /// Left shift `<<`.
    LeftShift, // <<
    /// Right shift `>>`.
    RightShift, // >>
    /// Unsigned right shift `>>>`.
    UnsignedRightShift, // >>>

    /// Increment `++`.
    Increment, // ++
    /// Decrement `--`.
    Decrement, // --

    /// Question mark `?`.
    Question, // ?
    /// Colon `:`.
    Colon, // :
    /// Elvis operator `?:`.
    Elvis, // ?:
    /// Safe navigation operator `?.`.
    SafeNavigation, // ?.

    // --- Delimiters ---
    /// Left parenthesis `(`.
    LeftParen, // (
    /// Right parenthesis `)`.
    RightParen, // )
    /// Left bracket `[`.
    LeftBracket, // [
    /// Right bracket `]`.
    RightBracket, // ]
    /// Left brace `{`.
    LeftBrace, // {
    /// Right brace `}`.
    RightBrace, // }
    /// Comma `,`.
    Comma, // ,
    /// Period `.`.
    Period, // .
    /// Semicolon `;`.
    Semicolon, // ;
    /// At symbol `@`.
    At, // @

    // --- Whitespace and Comments ---
    /// Whitespace.
    Whitespace,
    /// Comment.
    Comment,

    // --- Special ---
    /// Newline.
    Newline,
    /// End of file.
    Eof,
    /// Error token.
    Error,
}
