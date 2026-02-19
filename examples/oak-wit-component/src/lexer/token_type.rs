use oak_core::{Token, TokenType, UniversalTokenRole};

/// A token produced by the WIT lexer, combining a token type with its source location.
pub type WitToken = Token<WitTokenType>;

/// Token types recognized by the WIT (WebAssembly Interface Types) lexer.
///
/// This enum defines all possible token types that can be produced when
/// lexing WIT component definition files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WitTokenType {
    /// Root node token.
    Root,
    /// Whitespace token.
    Whitespace,
    /// Newline token.
    Newline,
    /// Comment token.
    Comment,
    /// Error token for unrecognized input.
    Error,
    /// End of file token.
    Eof,
    /// Text token for arbitrary text content.
    Text,

    /// Integer literal token.
    IntegerLiteral,
    /// Float literal token.
    FloatLiteral,
    /// String literal token.
    StringLiteral,
    /// Identifier token.
    Identifier,

    /// `world` keyword.
    WorldKw,
    /// `interface` keyword.
    InterfaceKw,
    /// `package` keyword.
    PackageKw,
    /// `component` keyword.
    ComponentKw,
    /// `instance` keyword.
    InstanceKw,
    /// `module` keyword.
    ModuleKw,
    /// `core` keyword.
    CoreKw,
    /// `func` keyword.
    FuncKw,
    /// `type` keyword.
    TypeKw,
    /// `record` keyword.
    RecordKw,
    /// `variant` keyword.
    VariantKw,
    /// `enum` keyword.
    EnumKw,
    /// `flags` keyword.
    FlagsKw,
    /// `union` keyword.
    UnionKw,
    /// `tuple` keyword.
    TupleKw,
    /// `list` keyword.
    ListKw,
    /// `option` keyword.
    OptionKw,
    /// `result` keyword.
    ResultKw,
    /// `resource` keyword.
    ResourceKw,

    /// `import` keyword.
    ImportKw,
    /// `export` keyword.
    ExportKw,
    /// `use` keyword.
    UseKw,
    /// `include` keyword.
    IncludeKw,
    /// `with` keyword.
    WithKw,

    /// `static` keyword.
    StaticKw,
    /// `constructor` keyword.
    ConstructorKw,
    /// `method` keyword.
    MethodKw,

    /// `bool` type keyword.
    BoolKw,
    /// `u8` type keyword.
    U8Kw,
    /// `u16` type keyword.
    U16Kw,
    /// `u32` type keyword.
    U32Kw,
    /// `u64` type keyword.
    U64Kw,
    /// `s8` type keyword.
    S8Kw,
    /// `s16` type keyword.
    S16Kw,
    /// `s32` type keyword.
    S32Kw,
    /// `s64` type keyword.
    S64Kw,
    /// `f32` type keyword.
    F32Kw,
    /// `f64` type keyword.
    F64Kw,
    /// `char` type keyword.
    CharKw,
    /// `string` type keyword.
    StringKw,

    /// Arrow operator `->`.
    Arrow,
    /// Fat arrow operator `=>`.
    FatArrow,
    /// Assignment operator `=`.
    Assign,
    /// Colon operator `:`.
    Colon,
    /// Semicolon operator `;`.
    Semicolon,
    /// Comma operator `,`.
    Comma,
    /// Dot operator `.`.
    Dot,
    /// Question operator `?`.
    Question,
    /// At operator `@`.
    At,
    /// Hash operator `#`.
    Hash,
    /// Dollar operator `$`.
    Dollar,
    /// Percent operator `%`.
    Percent,
    /// Ampersand operator `&`.
    Ampersand,
    /// Star operator `*`.
    Star,
    /// Plus operator `+`.
    Plus,
    /// Minus operator `-`.
    Minus,
    /// Slash operator `/`.
    Slash,
    /// Less-than operator `<`.
    Lt,
    /// Greater-than operator `>`.
    Gt,
    /// Pipe operator `|`.
    Pipe,
    /// Caret operator `^`.
    Caret,
    /// Tilde operator `~`.
    Tilde,
    /// Bang operator `!`.
    Bang,

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
}

impl WitTokenType {
    /// Returns `true` if this token type is a WIT keyword.
    ///
    /// Keywords include structural keywords (world, interface, package, etc.),
    /// import/export keywords, type-related keywords, and basic type keywords.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::WorldKw
                | Self::InterfaceKw
                | Self::PackageKw
                | Self::ComponentKw
                | Self::InstanceKw
                | Self::ModuleKw
                | Self::CoreKw
                | Self::FuncKw
                | Self::TypeKw
                | Self::RecordKw
                | Self::VariantKw
                | Self::EnumKw
                | Self::FlagsKw
                | Self::UnionKw
                | Self::TupleKw
                | Self::ListKw
                | Self::OptionKw
                | Self::ResultKw
                | Self::ResourceKw
                | Self::ImportKw
                | Self::ExportKw
                | Self::UseKw
                | Self::IncludeKw
                | Self::WithKw
                | Self::StaticKw
                | Self::ConstructorKw
                | Self::MethodKw
                | Self::BoolKw
                | Self::U8Kw
                | Self::U16Kw
                | Self::U32Kw
                | Self::U64Kw
                | Self::S8Kw
                | Self::S16Kw
                | Self::S32Kw
                | Self::S64Kw
                | Self::F32Kw
                | Self::F64Kw
                | Self::CharKw
                | Self::StringKw
        )
    }
}

impl std::fmt::Display for WitTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TokenType for WitTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            _ => UniversalTokenRole::None,
        }
    }
}
