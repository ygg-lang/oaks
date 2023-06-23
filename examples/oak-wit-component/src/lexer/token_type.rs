use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type WitToken = Token<WitTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WitTokenType {
    // 基础 kind
    Root,
    Whitespace,
    Newline,
    Comment,
    Error,
    Eof,
    Text,

    // 字面量
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    Identifier,

    // WIT Component 关键字 - 基本结构
    WorldKw,
    InterfaceKw,
    PackageKw,
    ComponentKw,
    InstanceKw,
    ModuleKw,
    CoreKw,
    FuncKw,
    TypeKw,
    RecordKw,
    VariantKw,
    EnumKw,
    FlagsKw,
    UnionKw,
    TupleKw,
    ListKw,
    OptionKw,
    ResultKw,
    ResourceKw,

    // 导入导出
    ImportKw,
    ExportKw,
    UseKw,
    IncludeKw,
    WithKw,

    // 类型相关
    StaticKw,
    ConstructorKw,
    MethodKw,

    // 基本类型
    BoolKw,
    U8Kw,
    U16Kw,
    U32Kw,
    U64Kw,
    S8Kw,
    S16Kw,
    S32Kw,
    S64Kw,
    F32Kw,
    F64Kw,
    CharKw,
    StringKw,

    // 操作符
    Arrow,     // ->
    FatArrow,  // =>
    Assign,    // =
    Colon,     // :
    Semicolon, // ;
    Comma,     // ,
    Dot,       // .
    Question,  // ?
    At,        // ↯
    Hash,      // #
    Dollar,    // $
    Percent,   // %
    Ampersand, // &
    Star,      // *
    Plus,      // +
    Minus,     // -
    Slash,     // /
    Lt,        // <
    Gt,        // >
    Pipe,      // |
    Caret,     // ^
    Tilde,     // ~
    Bang,      // !

    // 标点符号
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
}

impl WitTokenType {
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
