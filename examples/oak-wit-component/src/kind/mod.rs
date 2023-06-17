use oak_core::SyntaxKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WitSyntaxKind {
    // 基础 kind
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

impl SyntaxKind for WitSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        true // WIT 组件语法主要是 token 类型
    }

    fn is_element_type(&self) -> bool {
        false // WIT 组件语法主要是 token 类型
    }
}
