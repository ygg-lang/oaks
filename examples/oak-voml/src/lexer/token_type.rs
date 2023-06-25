//! Voml token types.

use oak_core::{Token, TokenType, UniversalTokenRole};

/// A token in the Voml language.
pub type VomlToken = Token<VomlTokenType>;

/// Enum representing all possible token types in Voml.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum VomlTokenType {
    // Keywords
    /// `module` keyword.
    ModuleKw,
    /// `import` keyword.
    ImportKw,
    /// `pub` keyword.
    PubKw,
    /// `fn` keyword.
    FnKw,
    /// `struct` keyword.
    StructKw,
    /// `interface` keyword.
    InterfaceKw,
    /// `enum` keyword.
    EnumKw,
    /// `type` keyword.
    TypeKw,
    /// `const` keyword.
    ConstKw,
    /// `mut` keyword.
    MutKw,
    /// `shared` keyword.
    SharedKw,
    /// `volatile` keyword.
    VolatileKw,
    /// `unsafe` keyword.
    UnsafeKw,
    /// `if` keyword.
    IfKw,
    /// `else` keyword.
    ElseKw,
    /// `for` keyword.
    ForKw,
    /// `in` keyword.
    InKw,
    /// `match` keyword.
    MatchKw,
    /// `or` keyword.
    OrKw,
    /// `return` keyword.
    ReturnKw,
    /// `break` keyword.
    BreakKw,
    /// `continue` keyword.
    ContinueKw,
    /// `goto` keyword.
    GotoKw,
    /// `defer` keyword.
    DeferKw,
    /// `go` keyword.
    GoKw,
    /// `select` keyword.
    SelectKw,
    /// `lock` keyword.
    LockKw,
    /// `rlock` keyword.
    RlockKw,
    /// `as` keyword.
    AsKw,
    /// `is` keyword.
    IsKw,
    /// `sizeof` keyword.
    SizeofKw,
    /// `typeof` keyword.
    TypeofKw,
    /// `offsetof` keyword.
    OffsetofKw,
    /// `assert` keyword.
    AssertKw,
    /// `panic` keyword.
    PanicKw,
    /// `eprintln` keyword.
    EprintlnKw,
    /// `println` keyword.
    PrintlnKw,
    /// `print` keyword.
    PrintKw,
    /// `eprint` keyword.
    EprintKw,
    /// `bool` keyword.
    BoolKw,
    /// `i8` keyword.
    I8Kw,
    /// `i16` keyword.
    I16Kw,
    /// `i32` keyword.
    I32Kw,
    /// `i64` keyword.
    I64Kw,
    /// `u8` keyword.
    U8Kw,
    /// `u16` keyword.
    U16Kw,
    /// `u32` keyword.
    U32Kw,
    /// `u64` keyword.
    U64Kw,
    /// `int` keyword.
    IntKw,
    /// `uint` keyword.
    UintKw,
    /// `f32` keyword.
    F32Kw,
    /// `f64` keyword.
    F64Kw,
    /// `string` keyword.
    StringKw,
    /// `rune` keyword.
    RuneKw,
    /// `byte` keyword.
    ByteKw,
    /// `voidptr` keyword.
    VoidptrKw,
    /// `char` keyword.
    CharKw,
    /// Boolean literal.
    BoolLiteral,

    // Basic kinds
    /// Identifier.
    Identifier,
    /// Numeric literal.
    Number,
    /// String literal.
    String,
    /// Whitespace.
    Whitespace,
    /// Comment.
    Comment,
    /// Error token.
    Error,
    /// End of stream.
    Eof,

    // Literals used by the lexer
    /// String literal.
    StringLiteral,
    /// Character literal.
    CharLiteral,
    /// Floating point literal.
    FloatLiteral,
    /// Integer literal.
    IntegerLiteral,

    // Operators and punctuation
    /// `+` operator.
    Plus,
    /// `+=` operator.
    PlusEq,
    /// `++` operator.
    PlusPlus,
    /// `-` operator.
    Minus,
    /// `-=` operator.
    MinusEq,
    /// `--` operator.
    MinusMinus,
    /// `->` operator.
    Arrow,
    /// `*` operator.
    Star,
    /// `*=` operator.
    StarEq,
    /// `/` operator.
    Slash,
    /// `/=` operator.
    SlashEq,
    /// `%` operator.
    Percent,
    /// `%=` operator.
    PercentEq,
    /// `&` operator.
    Ampersand,
    /// `&=` operator.
    AmpersandEq,
    /// `&&` operator.
    AndAnd,
    /// `|` operator.
    Pipe,
    /// `|=` operator.
    PipeEq,
    /// `||` operator.
    OrOr,
    /// `^` operator.
    Caret,
    /// `^=` operator.
    CaretEq,
    /// `=` operator.
    Eq,
    /// `==` operator.
    EqEq,
    /// `=>` operator.
    FatArrow,
    /// `!` operator.
    Bang,
    /// `!=` operator.
    Ne,
    /// `<` operator.
    LessThan,
    /// `<=` operator.
    Le,
    /// `<<` operator.
    LeftShift,
    /// `<<=` operator.
    LeftShiftEq,
    /// `>` operator.
    GreaterThan,
    /// `>=` operator.
    Ge,
    /// `>>` operator.
    RightShift,
    /// `>>=` operator.
    RightShiftEq,
    /// `.` symbol.
    Dot,
    /// `..` symbol.
    DotDot,
    /// `...` symbol.
    DotDotDot,
    /// `,` symbol.
    Comma,
    /// `:` symbol.
    Colon,
    /// `;` symbol.
    Semicolon,
    /// `(` symbol.
    LeftParen,
    /// `)` symbol.
    RightParen,
    /// `[` symbol.
    LeftBracket,
    /// `]` symbol.
    RightBracket,
    /// `{` symbol.
    LeftBrace,
    /// `}` symbol.
    RightBrace,
    /// `?` symbol.
    Question,
    /// `~` symbol.
    Tilde,
}

impl TokenType for VomlTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            Self::ModuleKw
            | Self::ImportKw
            | Self::PubKw
            | Self::FnKw
            | Self::StructKw
            | Self::InterfaceKw
            | Self::EnumKw
            | Self::TypeKw
            | Self::ConstKw
            | Self::MutKw
            | Self::SharedKw
            | Self::VolatileKw
            | Self::UnsafeKw
            | Self::IfKw
            | Self::ElseKw
            | Self::ForKw
            | Self::InKw
            | Self::MatchKw
            | Self::OrKw
            | Self::ReturnKw
            | Self::BreakKw
            | Self::ContinueKw
            | Self::GotoKw
            | Self::DeferKw
            | Self::GoKw
            | Self::SelectKw
            | Self::LockKw
            | Self::RlockKw
            | Self::AsKw
            | Self::IsKw
            | Self::SizeofKw
            | Self::TypeofKw
            | Self::OffsetofKw
            | Self::AssertKw
            | Self::PanicKw
            | Self::EprintlnKw
            | Self::PrintlnKw
            | Self::PrintKw
            | Self::EprintKw
            | Self::BoolKw
            | Self::I8Kw
            | Self::I16Kw
            | Self::I32Kw
            | Self::I64Kw
            | Self::U8Kw
            | Self::U16Kw
            | Self::U32Kw
            | Self::U64Kw
            | Self::IntKw
            | Self::UintKw
            | Self::F32Kw
            | Self::F64Kw
            | Self::StringKw
            | Self::RuneKw
            | Self::ByteKw
            | Self::VoidptrKw
            | Self::CharKw => UniversalTokenRole::Keyword,
            Self::BoolLiteral | Self::StringLiteral | Self::CharLiteral | Self::FloatLiteral | Self::IntegerLiteral => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Number => UniversalTokenRole::Literal,
            Self::String => UniversalTokenRole::Literal,
            Self::Plus
            | Self::PlusEq
            | Self::PlusPlus
            | Self::Minus
            | Self::MinusEq
            | Self::MinusMinus
            | Self::Arrow
            | Self::Star
            | Self::StarEq
            | Self::Slash
            | Self::SlashEq
            | Self::Percent
            | Self::PercentEq
            | Self::Ampersand
            | Self::AmpersandEq
            | Self::AndAnd
            | Self::Pipe
            | Self::PipeEq
            | Self::OrOr
            | Self::Caret
            | Self::CaretEq
            | Self::Eq
            | Self::EqEq
            | Self::FatArrow
            | Self::Bang
            | Self::Ne
            | Self::LessThan
            | Self::Le
            | Self::LeftShift
            | Self::LeftShiftEq
            | Self::GreaterThan
            | Self::Ge
            | Self::RightShift
            | Self::RightShiftEq
            | Self::Dot
            | Self::DotDot
            | Self::DotDotDot
            | Self::Question
            | Self::Tilde => UniversalTokenRole::Operator,
            Self::Comma | Self::Colon | Self::Semicolon => UniversalTokenRole::Punctuation,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace => UniversalTokenRole::Punctuation,
        }
    }
}
