use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WitElementType {
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
