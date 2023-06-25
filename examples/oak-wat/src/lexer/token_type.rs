use oak_core::{Token, TokenType, UniversalTokenRole};

/// A token in the WebAssembly Text (WAT) language.
pub type WatToken = Token<WatTokenType>;

impl TokenType for WatTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::ModuleKw
            | Self::FuncKw
            | Self::ExportKw
            | Self::ImportKw
            | Self::TypeKw
            | Self::ParamKw
            | Self::ResultKw
            | Self::LocalKw
            | Self::GlobalKw
            | Self::MemoryKw
            | Self::TableKw
            | Self::ElemKw
            | Self::DataKw
            | Self::StartKw
            | Self::BlockKw
            | Self::LoopKw
            | Self::IfKw
            | Self::ThenKw
            | Self::ElseKw
            | Self::EndKw
            | Self::BrKw
            | Self::BrIfKw
            | Self::BrTableKw
            | Self::ReturnKw
            | Self::CallKw
            | Self::CallIndirectKw
            | Self::LocalGetKw
            | Self::LocalSetKw
            | Self::LocalTeeKw
            | Self::GlobalGetKw
            | Self::GlobalSetKw
            | Self::I32LoadKw
            | Self::I64LoadKw
            | Self::F32LoadKw
            | Self::F64LoadKw
            | Self::I32StoreKw
            | Self::I64StoreKw
            | Self::F32StoreKw
            | Self::F64StoreKw
            | Self::MemorySizeKw
            | Self::MemoryGrowKw
            | Self::I32ConstKw
            | Self::I64ConstKw
            | Self::F32ConstKw
            | Self::F64ConstKw
            | Self::I32AddKw
            | Self::I64AddKw
            | Self::F32AddKw
            | Self::F64AddKw
            | Self::I32SubKw
            | Self::I64SubKw
            | Self::F32SubKw
            | Self::F64SubKw
            | Self::I32MulKw
            | Self::I64MulKw
            | Self::F32MulKw
            | Self::F64MulKw
            | Self::I32EqKw
            | Self::I64EqKw
            | Self::F32EqKw
            | Self::F64EqKw
            | Self::I32NeKw
            | Self::I64NeKw
            | Self::F32NeKw
            | Self::F64NeKw
            | Self::DropKw
            | Self::SelectKw
            | Self::UnreachableKw
            | Self::NopKw
            | Self::I32Kw
            | Self::I64Kw
            | Self::F32Kw
            | Self::F64Kw => UniversalTokenRole::Keyword,
            Self::LeftParen | Self::RightParen => UniversalTokenRole::Punctuation,
            Self::Eq => UniversalTokenRole::Operator,
            _ => UniversalTokenRole::None,
        }
    }
}

/// Token types for the WebAssembly Text (WAT) language.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum WatTokenType {
    // Base kinds
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,
    /// Error token.
    Error,
    /// End of stream.
    Eof,
    /// Plain text.
    Text,

    // Literals
    /// Integer literal.
    IntegerLiteral,
    /// Floating-point literal.
    FloatLiteral,
    /// String literal.
    StringLiteral,
    /// Identifier (starting with $).
    Identifier,

    // Keywords - Module structure
    /// `module` keyword.
    ModuleKw,
    /// `func` keyword.
    FuncKw,
    /// `export` keyword.
    ExportKw,
    /// `import` keyword.
    ImportKw,
    /// `type` keyword.
    TypeKw,
    /// `param` keyword.
    ParamKw,
    /// `result` keyword.
    ResultKw,
    /// `local` keyword.
    LocalKw,
    /// `global` keyword.
    GlobalKw,
    /// `memory` keyword.
    MemoryKw,
    /// `table` keyword.
    TableKw,
    /// `elem` keyword.
    ElemKw,
    /// `data` keyword.
    DataKw,
    /// `start` keyword.
    StartKw,

    // Keywords - Control flow
    /// `block` keyword.
    BlockKw,
    /// `loop` keyword.
    LoopKw,
    /// `if` keyword.
    IfKw,
    /// `then` keyword.
    ThenKw,
    /// `else` keyword.
    ElseKw,
    /// `end` keyword.
    EndKw,
    /// `br` keyword.
    BrKw,
    /// `br_if` keyword.
    BrIfKw,
    /// `br_table` keyword.
    BrTableKw,
    /// `return` keyword.
    ReturnKw,
    /// `call` keyword.
    CallKw,
    /// `call_indirect` keyword.
    CallIndirectKw,

    // Keywords - Variable operations
    /// `local.get` keyword.
    LocalGetKw,
    /// `local.set` keyword.
    LocalSetKw,
    /// `local.tee` keyword.
    LocalTeeKw,
    /// `global.get` keyword.
    GlobalGetKw,
    /// `global.set` keyword.
    GlobalSetKw,

    // Keywords - Memory operations
    /// `i32.load` keyword.
    I32LoadKw,
    /// `i64.load` keyword.
    I64LoadKw,
    /// `f32.load` keyword.
    F32LoadKw,
    /// `f64.load` keyword.
    F64LoadKw,
    /// `i32.load8_s` keyword.
    I32Load8SKw,
    /// `i32.load8_u` keyword.
    I32Load8UKw,
    /// `i32.load16_s` keyword.
    I32Load16SKw,
    /// `i32.load16_u` keyword.
    I32Load16UKw,
    /// `i64.load8_s` keyword.
    I64Load8SKw,
    /// `i64.load8_u` keyword.
    I64Load8UKw,
    /// `i64.load16_s` keyword.
    I64Load16SKw,
    /// `i64.load16_u` keyword.
    I64Load16UKw,
    /// `i64.load32_s` keyword.
    I64Load32SKw,
    /// `i64.load32_u` keyword.
    I64Load32UKw,
    /// `i32.store` keyword.
    I32StoreKw,
    /// `i64.store` keyword.
    I64StoreKw,
    /// `f32.store` keyword.
    F32StoreKw,
    /// `f64.store` keyword.
    F64StoreKw,
    /// `i32.store8` keyword.
    I32Store8Kw,
    /// `i32.store16` keyword.
    I32Store16Kw,
    /// `i64.store8` keyword.
    I64Store8Kw,
    /// `i64.store16` keyword.
    I64Store16Kw,
    /// `i64.store32` keyword.
    I64Store32Kw,
    /// `memory.size` keyword.
    MemorySizeKw,
    /// `memory.grow` keyword.
    MemoryGrowKw,

    // Keywords - Constants
    /// `i32.const` keyword.
    I32ConstKw,
    /// `i64.const` keyword.
    I64ConstKw,
    /// `f32.const` keyword.
    F32ConstKw,
    /// `f64.const` keyword.
    F64ConstKw,

    // Keywords - Arithmetic operations
    /// `i32.add` keyword.
    I32AddKw,
    /// `i64.add` keyword.
    I64AddKw,
    /// `f32.add` keyword.
    F32AddKw,
    /// `f64.add` keyword.
    F64AddKw,
    /// `i32.sub` keyword.
    I32SubKw,
    /// `i64.sub` keyword.
    I64SubKw,
    /// `f32.sub` keyword.
    F32SubKw,
    /// `f64.sub` keyword.
    F64SubKw,
    /// `i32.mul` keyword.
    I32MulKw,
    /// `i64.mul` keyword.
    I64MulKw,
    /// `f32.mul` keyword.
    F32MulKw,
    /// `f64.mul` keyword.
    F64MulKw,
    /// `i32.div_s` keyword.
    I32DivSKw,
    /// `i32.div_u` keyword.
    I32DivUKw,
    /// `i64.div_s` keyword.
    I64DivSKw,
    /// `i64.div_u` keyword.
    I64DivUKw,
    /// `f32.div` keyword.
    F32DivKw,
    /// `f64.div` keyword.
    F64DivKw,
    /// `i32.rem_s` keyword.
    I32RemSKw,
    /// `i32.rem_u` keyword.
    I32RemUKw,
    /// `i64.rem_s` keyword.
    I64RemSKw,
    /// `i64.rem_u` keyword.
    I64RemUKw,

    // Keywords - Comparison operations
    /// `i32.eq` keyword.
    I32EqKw,
    /// `i64.eq` keyword.
    I64EqKw,
    /// `f32.eq` keyword.
    F32EqKw,
    /// `f64.eq` keyword.
    F64EqKw,
    /// `i32.ne` keyword.
    I32NeKw,
    /// `i64.ne` keyword.
    I64NeKw,
    /// `f32.ne` keyword.
    F32NeKw,
    /// `f64.ne` keyword.
    F64NeKw,
    /// `i32.lt_s` keyword.
    I32LtSKw,
    /// `i32.lt_u` keyword.
    I32LtUKw,
    /// `i64.lt_s` keyword.
    I64LtSKw,
    /// `i64.lt_u` keyword.
    I64LtUKw,
    /// `f32.lt` keyword.
    F32LtKw,
    /// `f64.lt` keyword.
    F64LtKw,
    /// `i32.gt_s` keyword.
    I32GtSKw,
    /// `i32.gt_u` keyword.
    I32GtUKw,
    /// `i64.gt_s` keyword.
    I64GtSKw,
    /// `i64.gt_u` keyword.
    I64GtUKw,
    /// `f32.gt` keyword.
    F32GtKw,
    /// `f64.gt` keyword.
    F64GtKw,
    /// `i32.le_s` keyword.
    I32LeSKw,
    /// `i32.le_u` keyword.
    I32LeUKw,
    /// `i64.le_s` keyword.
    I64LeSKw,
    /// `i64.le_u` keyword.
    I64LeUKw,
    /// `f32.le` keyword.
    F32LeKw,
    /// `f64.le` keyword.
    F64LeKw,
    /// `i32.ge_s` keyword.
    I32GeSKw,
    /// `i32.ge_u` keyword.
    I32GeUKw,
    /// `i64.ge_s` keyword.
    I64GeSKw,
    /// `i64.ge_u` keyword.
    I64GeUKw,
    /// `f32.ge` keyword.
    F32GeKw,
    /// `f64.ge` keyword.
    F64GeKw,

    // Keywords - Bitwise operations
    /// `i32.and` keyword.
    I32AndKw,
    /// `i64.and` keyword.
    I64AndKw,
    /// `i32.or` keyword.
    I32OrKw,
    /// `i64.or` keyword.
    I64OrKw,
    /// `i32.xor` keyword.
    I32XorKw,
    /// `i64.xor` keyword.
    I64XorKw,
    /// `i32.shl` keyword.
    I32ShlKw,
    /// `i64.shl` keyword.
    I64ShlKw,
    /// `i32.shr_s` keyword.
    I32ShrSKw,
    /// `i32.shr_u` keyword.
    I32ShrUKw,
    /// `i64.shr_s` keyword.
    I64ShrSKw,
    /// `i64.shr_u` keyword.
    I64ShrUKw,
    /// `i32.rotl` keyword.
    I32RotlKw,
    /// `i64.rotl` keyword.
    I64RotlKw,
    /// `i32.rotr` keyword.
    I32RotrKw,
    /// `i64.rotr` keyword.
    I64RotrKw,

    // Keywords - Conversions
    /// `i32.wrap_i64` keyword.
    I32WrapI64Kw,
    /// `i64.extend_i32_s` keyword.
    I64ExtendI32SKw,
    /// `i64.extend_i32_u` keyword.
    I64ExtendI32UKw,
    /// `i32.trunc_f32_s` keyword.
    I32TruncF32SKw,
    /// `i32.trunc_f32_u` keyword.
    I32TruncF32UKw,
    /// `i32.trunc_f64_s` keyword.
    I32TruncF64SKw,
    /// `i32.trunc_f64_u` keyword.
    I32TruncF64UKw,
    /// `i64.trunc_f32_s` keyword.
    I64TruncF32SKw,
    /// `i64.trunc_f32_u` keyword.
    I64TruncF32UKw,
    /// `i64.trunc_f64_s` keyword.
    I64TruncF64SKw,
    /// `i64.trunc_f64_u` keyword.
    I64TruncF64UKw,
    /// `f32.convert_i32_s` keyword.
    F32ConvertI32SKw,
    /// `f32.convert_i32_u` keyword.
    F32ConvertI32UKw,
    /// `f32.convert_i64_s` keyword.
    F32ConvertI64SKw,
    /// `f32.convert_i64_u` keyword.
    F32ConvertI64UKw,
    /// `f64.convert_i32_s` keyword.
    F64ConvertI32SKw,
    /// `f64.convert_i32_u` keyword.
    F64ConvertI32UKw,
    /// `f64.convert_i64_s` keyword.
    F64ConvertI64SKw,
    /// `f64.convert_i64_u` keyword.
    F64ConvertI64UKw,
    /// `f32.demote_f64` keyword.
    F32DemoteF64Kw,
    /// `f64.promote_f32` keyword.
    F64PromoteF32Kw,

    // Keywords - Other instructions
    /// `drop` keyword.
    DropKw,
    /// `select` keyword.
    SelectKw,
    /// `unreachable` keyword.
    UnreachableKw,
    /// `nop` keyword.
    NopKw,

    // Keywords - Types
    /// `i32` keyword.
    I32Kw,
    /// `i64` keyword.
    I64Kw,
    /// `f32` keyword.
    F32Kw,
    /// `f64` keyword.
    F64Kw,
    /// `funcref` keyword.
    FuncrefKw,
    /// `externref` keyword.
    ExternrefKw,

    // Keywords - Other
    /// `mut` keyword.
    MutKw,
    /// `offset` keyword.
    OffsetKw,
    /// `align` keyword.
    AlignKw,

    // Punctuations
    /// `(`.
    LeftParen,
    /// `)`.
    RightParen,
    /// `[`.
    LeftBracket,
    /// `]`.
    RightBracket,
    /// `{`.
    LeftBrace,
    /// `}`.
    RightBrace,
    /// `;`.
    Semicolon,
    /// `,`.
    Comma,
    /// `.`.
    Dot,
    /// `"`.
    Quote,
    /// `$`.
    Dollar,
    /// `+`.
    Plus,
    /// `-`.
    Minus,
    /// `*`.
    Star,
    /// `/`.
    Slash,
    /// `=`.
    Eq,
    /// `:`.
    Colon,
    /// `?`.
    Question,
    /// `!`.
    Bang,
    /// `@`.
    At,
    /// `#`.
    Hash,
    /// `%`.
    Percent,
    /// `^`.
    Caret,
    /// `&`.
    Ampersand,
    /// `<`.
    LessThan,
    /// `>`.
    GreaterThan,
    /// `\`.
    Backslash,
    /// `|`.
    Pipe,
    /// `~`.
    Tilde,
    // Special
    /// Root node.
    Root,
    /// Source file.
    SourceFile,
    /// Module.
    Module,
    /// Item.
    Item,
}
