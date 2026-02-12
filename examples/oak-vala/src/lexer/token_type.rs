use oak_core::{Token, TokenType, UniversalTokenRole};

/// Vala token.
pub type ValaToken = Token<ValaTokenType>;

/// Vala token types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum ValaTokenType {
    // Basic
    /// Whitespace.
    Whitespace,
    /// Single-line comment starting with `//`.
    LineComment,
    /// Multi-line block comment.
    BlockComment,
    /// End of file.
    Eof,
    /// Lexical error.
    Error,

    // Literals
    /// Identifier.
    Identifier,
    /// String literal.
    StringLiteral,
    /// Character literal.
    CharLiteral,
    /// Integer literal.
    IntegerLiteral,
    /// Float literal.
    FloatLiteral,

    // Keywords
    /// `abstract` keyword.
    AbstractKw,
    /// `as` keyword.
    AsKw,
    /// `base` keyword.
    BaseKw,
    /// `break` keyword.
    BreakKw,
    /// `case` keyword.
    CaseKw,
    /// `catch` keyword.
    CatchKw,
    /// `class` keyword.
    ClassKw,
    /// `const` keyword.
    ConstKw,
    /// `construct` keyword.
    ConstructKw,
    /// `continue` keyword.
    ContinueKw,
    /// `default` keyword.
    DefaultKw,
    /// `delegate` keyword.
    DelegateKw,
    /// `delete` keyword.
    DeleteKw,
    /// `do` keyword.
    DoKw,
    /// `else` keyword.
    ElseKw,
    /// `enum` keyword.
    EnumKw,
    /// `ensures` keyword.
    EnsuresKw,
    /// `errordomain` keyword.
    ErrordomainKw,
    /// `extern` keyword.
    ExternKw,
    /// `false` keyword.
    FalseKw,
    /// `finally` keyword.
    FinallyKw,
    /// `for` keyword.
    ForKw,
    /// `foreach` keyword.
    ForeachKw,
    /// `get` keyword.
    GetKw,
    /// `if` keyword.
    IfKw,
    /// `in` keyword.
    InKw,
    /// `inline` keyword.
    InlineKw,
    /// `interface` keyword.
    InterfaceKw,
    /// `internal` keyword.
    InternalKw,
    /// `is` keyword.
    IsKw,
    /// `lock` keyword.
    LockKw,
    /// `namespace` keyword.
    NamespaceKw,
    /// `new` keyword.
    NewKw,
    /// `null` keyword.
    NullKw,
    /// `out` keyword.
    OutKw,
    /// `override` keyword.
    OverrideKw,
    /// `owned` keyword.
    OwnedKw,
    /// `private` keyword.
    PrivateKw,
    /// `protected` keyword.
    ProtectedKw,
    /// `public` keyword.
    PublicKw,
    /// `ref` keyword.
    RefKw,
    /// `requires` keyword.
    RequiresKw,
    /// `return` keyword.
    ReturnKw,
    /// `set` keyword.
    SetKw,
    /// `sizeof` keyword.
    SizeofKw,
    /// `static` keyword.
    StaticKw,
    /// `struct` keyword.
    StructKw,
    /// `switch` keyword.
    SwitchKw,
    /// `this` keyword.
    ThisKw,
    /// `throw` keyword.
    ThrowKw,
    /// `throws` keyword.
    ThrowsKw,
    /// `true` keyword.
    TrueKw,
    /// `try` keyword.
    TryKw,
    /// `typeof` keyword.
    TypeofKw,
    /// `unowned` keyword.
    UnownedKw,
    /// `using` keyword.
    UsingKw,
    /// `var` keyword.
    VarKw,
    /// `virtual` keyword.
    VirtualKw,
    /// `void` keyword.
    VoidKw,
    /// `volatile` keyword.
    VolatileKw,
    /// `weak` keyword.
    WeakKw,
    /// `while` keyword.
    WhileKw,
    /// `yield` keyword.
    YieldKw,

    // Basic type keywords
    /// `bool` keyword.
    BoolKw,
    /// `char` keyword.
    CharKw,
    /// `uchar` keyword.
    UcharKw,
    /// `int` keyword.
    IntKw,
    /// `uint` keyword.
    UintKw,
    /// `short` keyword.
    ShortKw,
    /// `ushort` keyword.
    UshortKw,
    /// `long` keyword.
    LongKw,
    /// `ulong` keyword.
    UlongKw,
    /// `int8` keyword.
    Int8Kw,
    /// `uint8` keyword.
    Uint8Kw,
    /// `int16` keyword.
    Int16Kw,
    /// `uint16` keyword.
    Uint16Kw,
    /// `int32` keyword.
    Int32Kw,
    /// `uint32` keyword.
    Uint32Kw,
    /// `int64` keyword.
    Int64Kw,
    /// `uint64` keyword.
    Uint64Kw,
    /// `float` keyword.
    FloatKw,
    /// `double` keyword.
    DoubleKw,
    /// `string` keyword.
    StringKw,

    // Operators
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
    /// Increment `++`.
    PlusPlus,
    /// Decrement `--`.
    MinusMinus,
    /// Assignment `=`.
    Eq,
    /// Plus equal `+=`.
    PlusEq,
    /// Minus equal `-=`.
    MinusEq,
    /// Star equal `*=`.
    StarEq,
    /// Slash equal `/=`.
    SlashEq,
    /// Percent equal `%=`.
    PercentEq,
    /// Equal `==`.
    EqEq,
    /// Not equal `!=`.
    NotEq,
    /// Greater than `>`.
    GreaterThan,
    /// Less than `<`.
    LessThan,
    /// Greater or equal `>=`.
    GreaterEq,
    /// Less or equal `<=`.
    LessEq,
    /// Ampersand `&`.
    Ampersand,
    /// Pipe `|`.
    Pipe,
    /// Caret `^`.
    Caret,
    /// Tilde `~`.
    Tilde,
    /// Logical not `!`.
    Bang,
    /// Logical and `&&`.
    AndAnd,
    /// Logical or `||`.
    OrOr,
    /// Left shift `<<`.
    LeftShift,
    /// Right shift `>>`.
    RightShift,
    /// Left shift equal `<<=`.
    LeftShiftEq,
    /// Right shift equal `>>=`.
    RightShiftEq,
    /// Question mark `?`.
    Question,
    /// Null coalescing `??`.
    QuestionQuestion,
    /// Dot `.`.
    Dot,
    /// Colon `:`.
    Colon,
    /// Double colon `::`.
    ColonColon,
    /// Arrow `->`.
    Arrow,
    /// Lambda arrow `=>`.
    Lambda,

    // Punctuation
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Comma `,`.
    Comma,
    /// Semicolon `;`.
    Semicolon,
    /// Backslash `\`.
    Backslash,
    /// `@` symbol.
    At,
    /// `#` symbol.
    Hash,
    /// `$` symbol.
    Dollar,
}

impl TokenType for ValaTokenType {
    type Role = UniversalTokenRole;

    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral | Self::CharLiteral | Self::IntegerLiteral | Self::FloatLiteral => UniversalTokenRole::Literal,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            _ if self.is_operator() => UniversalTokenRole::Operator,
            _ if self.is_punctuation() => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}

impl ValaTokenType {
    /// Check if it is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::AbstractKw
                | Self::AsKw
                | Self::BaseKw
                | Self::BreakKw
                | Self::CaseKw
                | Self::CatchKw
                | Self::ClassKw
                | Self::ConstKw
                | Self::ConstructKw
                | Self::ContinueKw
                | Self::DefaultKw
                | Self::DelegateKw
                | Self::DeleteKw
                | Self::DoKw
                | Self::ElseKw
                | Self::EnumKw
                | Self::EnsuresKw
                | Self::ErrordomainKw
                | Self::ExternKw
                | Self::FalseKw
                | Self::FinallyKw
                | Self::ForKw
                | Self::ForeachKw
                | Self::GetKw
                | Self::IfKw
                | Self::InKw
                | Self::InlineKw
                | Self::InterfaceKw
                | Self::InternalKw
                | Self::IsKw
                | Self::LockKw
                | Self::NamespaceKw
                | Self::NewKw
                | Self::NullKw
                | Self::OutKw
                | Self::OverrideKw
                | Self::OwnedKw
                | Self::PrivateKw
                | Self::ProtectedKw
                | Self::PublicKw
                | Self::RefKw
                | Self::RequiresKw
                | Self::ReturnKw
                | Self::SetKw
                | Self::SizeofKw
                | Self::StaticKw
                | Self::StructKw
                | Self::SwitchKw
                | Self::ThisKw
                | Self::ThrowKw
                | Self::ThrowsKw
                | Self::TrueKw
                | Self::TryKw
                | Self::TypeofKw
                | Self::UnownedKw
                | Self::UsingKw
                | Self::VarKw
                | Self::VirtualKw
                | Self::VoidKw
                | Self::VolatileKw
                | Self::WeakKw
                | Self::WhileKw
                | Self::YieldKw
                | Self::BoolKw
                | Self::CharKw
                | Self::UcharKw
                | Self::IntKw
                | Self::UintKw
                | Self::ShortKw
                | Self::UshortKw
                | Self::LongKw
                | Self::UlongKw
                | Self::Int8Kw
                | Self::Uint8Kw
                | Self::Int16Kw
                | Self::Uint16Kw
                | Self::Int32Kw
                | Self::Uint32Kw
                | Self::Int64Kw
                | Self::Uint64Kw
                | Self::FloatKw
                | Self::DoubleKw
                | Self::StringKw
        )
    }

    /// Check if it is an operator.
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::PlusPlus
                | Self::MinusMinus
                | Self::Eq
                | Self::PlusEq
                | Self::MinusEq
                | Self::StarEq
                | Self::SlashEq
                | Self::PercentEq
                | Self::EqEq
                | Self::NotEq
                | Self::GreaterThan
                | Self::LessThan
                | Self::GreaterEq
                | Self::LessEq
                | Self::Ampersand
                | Self::Pipe
                | Self::Caret
                | Self::Tilde
                | Self::Bang
                | Self::AndAnd
                | Self::OrOr
                | Self::LeftShift
                | Self::RightShift
                | Self::LeftShiftEq
                | Self::RightShiftEq
                | Self::Question
                | Self::QuestionQuestion
                | Self::Dot
                | Self::Colon
                | Self::ColonColon
                | Self::Arrow
                | Self::Lambda
        )
    }

    /// Check if it is a punctuation.
    pub fn is_punctuation(&self) -> bool {
        matches!(self, Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Comma | Self::Semicolon | Self::Backslash)
    }
}
