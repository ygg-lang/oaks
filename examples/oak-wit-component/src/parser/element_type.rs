use oak_core::{ElementType, Parser, UniversalElementRole};

/// Element types for the WIT Component parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WitElementType {
    /// Root node.
    Root,
    /// Whitespace trivia token.
    Whitespace,
    /// Newline trivia token.
    Newline,
    /// Comment trivia token.
    Comment,
    /// Error token.
    Error,
    /// End of file token.
    Eof,
    /// Text token.
    Text,

    /// Integer literal token.
    IntegerLiteral,
    /// Float literal token.
    FloatLiteral,
    /// String literal token.
    StringLiteral,
    /// Identifier token.
    Identifier,

    /// The `world` keyword.
    WorldKw,
    /// The `interface` keyword.
    InterfaceKw,
    /// The `package` keyword.
    PackageKw,
    /// The `component` keyword.
    ComponentKw,
    /// The `instance` keyword.
    InstanceKw,
    /// The `module` keyword.
    ModuleKw,
    /// The `core` keyword.
    CoreKw,
    /// The `func` keyword.
    FuncKw,
    /// The `type` keyword.
    TypeKw,
    /// The `record` keyword.
    RecordKw,
    /// The `variant` keyword.
    VariantKw,
    /// The `enum` keyword.
    EnumKw,
    /// The `flags` keyword.
    FlagsKw,
    /// The `union` keyword.
    UnionKw,
    /// The `tuple` keyword.
    TupleKw,
    /// The `list` keyword.
    ListKw,
    /// The `option` keyword.
    OptionKw,
    /// The `result` keyword.
    ResultKw,
    /// The `resource` keyword.
    ResourceKw,

    /// The `import` keyword.
    ImportKw,
    /// The `export` keyword.
    ExportKw,
    /// The `use` keyword.
    UseKw,
    /// The `include` keyword.
    IncludeKw,
    /// The `with` keyword.
    WithKw,

    /// The `static` keyword.
    StaticKw,
    /// The `constructor` keyword.
    ConstructorKw,
    /// The `method` keyword.
    MethodKw,

    /// The `bool` type keyword.
    BoolKw,
    /// The `u8` type keyword.
    U8Kw,
    /// The `u16` type keyword.
    U16Kw,
    /// The `u32` type keyword.
    U32Kw,
    /// The `u64` type keyword.
    U64Kw,
    /// The `s8` type keyword.
    S8Kw,
    /// The `s16` type keyword.
    S16Kw,
    /// The `s32` type keyword.
    S32Kw,
    /// The `s64` type keyword.
    S64Kw,
    /// The `f32` type keyword.
    F32Kw,
    /// The `f64` type keyword.
    F64Kw,
    /// The `char` type keyword.
    CharKw,
    /// The `string` type keyword.
    StringKw,

    /// The arrow operator `->`.
    Arrow,
    /// The fat arrow operator `=>`.
    FatArrow,
    /// The assign operator `=`.
    Assign,
    /// The colon operator `:`.
    Colon,
    /// The semicolon operator `;`.
    Semicolon,
    /// The comma operator `,`.
    Comma,
    /// The dot operator `.`.
    Dot,
    /// The question operator `?`.
    Question,
    /// The at operator `@`.
    At,
    /// The hash operator `#`.
    Hash,
    /// The dollar operator `$`.
    Dollar,
    /// The percent operator `%`.
    Percent,
    /// The ampersand operator `&`.
    Ampersand,
    /// The star operator `*`.
    Star,
    /// The plus operator `+`.
    Plus,
    /// The minus operator `-`.
    Minus,
    /// The slash operator `/`.
    Slash,
    /// The less-than operator `<`.
    Lt,
    /// The greater-than operator `>`.
    Gt,
    /// The pipe operator `|`.
    Pipe,
    /// The caret operator `^`.
    Caret,
    /// The tilde operator `~`.
    Tilde,
    /// The bang operator `!`.
    Bang,

    /// The left parenthesis `(`.
    LeftParen,
    /// The right parenthesis `)`.
    RightParen,
    /// The left brace `{`.
    LeftBrace,
    /// The right brace `}`.
    RightBrace,
    /// The left bracket `[`.
    LeftBracket,
    /// The right bracket `]`.
    RightBracket,
}

impl std::fmt::Display for WitElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ElementType for WitElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::WitTokenType> for WitElementType {
    fn from(token: crate::lexer::token_type::WitTokenType) -> Self {
        use crate::lexer::token_type::WitTokenType as T;
        match token {
            T::Root => Self::Root,
            T::Whitespace => Self::Whitespace,
            T::Newline => Self::Newline,
            T::Comment => Self::Comment,
            T::Error => Self::Error,
            T::Eof => Self::Eof,
            T::Text => Self::Text,
            T::IntegerLiteral => Self::IntegerLiteral,
            T::FloatLiteral => Self::FloatLiteral,
            T::StringLiteral => Self::StringLiteral,
            T::Identifier => Self::Identifier,
            T::WorldKw => Self::WorldKw,
            T::InterfaceKw => Self::InterfaceKw,
            T::PackageKw => Self::PackageKw,
            T::ComponentKw => Self::ComponentKw,
            T::InstanceKw => Self::InstanceKw,
            T::ModuleKw => Self::ModuleKw,
            T::CoreKw => Self::CoreKw,
            T::FuncKw => Self::FuncKw,
            T::TypeKw => Self::TypeKw,
            T::RecordKw => Self::RecordKw,
            T::VariantKw => Self::VariantKw,
            T::EnumKw => Self::EnumKw,
            T::FlagsKw => Self::FlagsKw,
            T::UnionKw => Self::UnionKw,
            T::TupleKw => Self::TupleKw,
            T::ListKw => Self::ListKw,
            T::OptionKw => Self::OptionKw,
            T::ResultKw => Self::ResultKw,
            T::ResourceKw => Self::ResourceKw,
            T::ImportKw => Self::ImportKw,
            T::ExportKw => Self::ExportKw,
            T::UseKw => Self::UseKw,
            T::IncludeKw => Self::IncludeKw,
            T::WithKw => Self::WithKw,
            T::StaticKw => Self::StaticKw,
            T::ConstructorKw => Self::ConstructorKw,
            T::MethodKw => Self::MethodKw,
            T::BoolKw => Self::BoolKw,
            T::U8Kw => Self::U8Kw,
            T::U16Kw => Self::U16Kw,
            T::U32Kw => Self::U32Kw,
            T::U64Kw => Self::U64Kw,
            T::S8Kw => Self::S8Kw,
            T::S16Kw => Self::S16Kw,
            T::S32Kw => Self::S32Kw,
            T::S64Kw => Self::S64Kw,
            T::F32Kw => Self::F32Kw,
            T::F64Kw => Self::F64Kw,
            T::CharKw => Self::CharKw,
            T::StringKw => Self::StringKw,
            T::Arrow => Self::Arrow,
            T::FatArrow => Self::FatArrow,
            T::Assign => Self::Assign,
            T::Colon => Self::Colon,
            T::Semicolon => Self::Semicolon,
            T::Comma => Self::Comma,
            T::Dot => Self::Dot,
            T::Question => Self::Question,
            T::At => Self::At,
            T::Hash => Self::Hash,
            T::Dollar => Self::Dollar,
            T::Percent => Self::Percent,
            T::Ampersand => Self::Ampersand,
            T::Star => Self::Star,
            T::Plus => Self::Plus,
            T::Minus => Self::Minus,
            T::Slash => Self::Slash,
            T::Lt => Self::Lt,
            T::Gt => Self::Gt,
            T::Pipe => Self::Pipe,
            T::Caret => Self::Caret,
            T::Tilde => Self::Tilde,
            T::Bang => Self::Bang,
            T::LeftParen => Self::LeftParen,
            T::RightParen => Self::RightParen,
            T::LeftBrace => Self::LeftBrace,
            T::RightBrace => Self::RightBrace,
            T::LeftBracket => Self::LeftBracket,
            T::RightBracket => Self::RightBracket,
        }
    }
}
