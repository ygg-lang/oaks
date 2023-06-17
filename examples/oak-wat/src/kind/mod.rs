use oak_core::SyntaxKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WatSyntaxKind {
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

    // 关键字 - 模块结构
    ModuleKw,
    FuncKw,
    ExportKw,
    ImportKw,
    TypeKw,
    ParamKw,
    ResultKw,
    LocalKw,
    GlobalKw,
    MemoryKw,
    TableKw,
    ElemKw,
    DataKw,
    StartKw,

    // 关键字 - 控制流
    BlockKw,
    LoopKw,
    IfKw,
    ThenKw,
    ElseKw,
    EndKw,
    BrKw,
    BrIfKw,
    BrTableKw,
    ReturnKw,
    CallKw,
    CallIndirectKw,

    // 关键字 - 变量操作
    LocalGetKw,
    LocalSetKw,
    LocalTeeKw,
    GlobalGetKw,
    GlobalSetKw,

    // 关键字 - 内存操作
    I32LoadKw,
    I64LoadKw,
    F32LoadKw,
    F64LoadKw,
    I32Load8SKw,
    I32Load8UKw,
    I32Load16SKw,
    I32Load16UKw,
    I64Load8SKw,
    I64Load8UKw,
    I64Load16SKw,
    I64Load16UKw,
    I64Load32SKw,
    I64Load32UKw,
    I32StoreKw,
    I64StoreKw,
    F32StoreKw,
    F64StoreKw,
    I32Store8Kw,
    I32Store16Kw,
    I64Store8Kw,
    I64Store16Kw,
    I64Store32Kw,
    MemorySizeKw,
    MemoryGrowKw,

    // 关键字 - 常量
    I32ConstKw,
    I64ConstKw,
    F32ConstKw,
    F64ConstKw,

    // 关键字 - 算术运算
    I32AddKw,
    I64AddKw,
    F32AddKw,
    F64AddKw,
    I32SubKw,
    I64SubKw,
    F32SubKw,
    F64SubKw,
    I32MulKw,
    I64MulKw,
    F32MulKw,
    F64MulKw,
    I32DivSKw,
    I32DivUKw,
    I64DivSKw,
    I64DivUKw,
    F32DivKw,
    F64DivKw,
    I32RemSKw,
    I32RemUKw,
    I64RemSKw,
    I64RemUKw,

    // 关键字 - 比较运算
    I32EqKw,
    I64EqKw,
    F32EqKw,
    F64EqKw,
    I32NeKw,
    I64NeKw,
    F32NeKw,
    F64NeKw,
    I32LtSKw,
    I32LtUKw,
    I64LtSKw,
    I64LtUKw,
    F32LtKw,
    F64LtKw,
    I32GtSKw,
    I32GtUKw,
    I64GtSKw,
    I64GtUKw,
    F32GtKw,
    F64GtKw,
    I32LeSKw,
    I32LeUKw,
    I64LeSKw,
    I64LeUKw,
    F32LeKw,
    F64LeKw,
    I32GeSKw,
    I32GeUKw,
    I64GeSKw,
    I64GeUKw,
    F32GeKw,
    F64GeKw,

    // 关键字 - 位运算
    I32AndKw,
    I64AndKw,
    I32OrKw,
    I64OrKw,
    I32XorKw,
    I64XorKw,
    I32ShlKw,
    I64ShlKw,
    I32ShrSKw,
    I32ShrUKw,
    I64ShrSKw,
    I64ShrUKw,
    I32RotlKw,
    I64RotlKw,
    I32RotrKw,
    I64RotrKw,

    // 关键字 - 类型转换
    I32WrapI64Kw,
    I64ExtendI32SKw,
    I64ExtendI32UKw,
    I32TruncF32SKw,
    I32TruncF32UKw,
    I32TruncF64SKw,
    I32TruncF64UKw,
    I64TruncF32SKw,
    I64TruncF32UKw,
    I64TruncF64SKw,
    I64TruncF64UKw,
    F32ConvertI32SKw,
    F32ConvertI32UKw,
    F32ConvertI64SKw,
    F32ConvertI64UKw,
    F64ConvertI32SKw,
    F64ConvertI32UKw,
    F64ConvertI64SKw,
    F64ConvertI64UKw,
    F32DemoteF64Kw,
    F64PromoteF32Kw,

    // 关键字 - 其他指令
    DropKw,
    SelectKw,
    UnreachableKw,
    NopKw,

    // 关键字 - 类型
    I32Kw,
    I64Kw,
    F32Kw,
    F64Kw,
    FuncrefKw,
    ExternrefKw,

    // 关键字 - 其他
    MutKw,
    OffsetKw,
    AlignKw,

    // 标点符号
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Dot,
    Quote,
    Dollar,
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    Colon,
    Question,
    Bang,
    At,
    Hash,
    Percent,
    Caret,
    Ampersand,
    LessThan,
    GreaterThan,
    Backslash,
    Pipe,
    Tilde,
}

impl SyntaxKind for WatSyntaxKind {
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
        !matches!(self, Self::Error)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Error)
    }
}
