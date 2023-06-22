use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WitSyntaxKind {
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
    At,        // @
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

impl std::fmt::Display for WitSyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TokenType for WitSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for WitSyntaxKind {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
