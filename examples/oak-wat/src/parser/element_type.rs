use oak_core::{ElementType, UniversalElementRole};

/// Element types for the WebAssembly Text (WAT) format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum WatElementType {
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// A comment.
    Comment,
    /// An error token.
    Error,
    /// End of stream.
    Eof,
    /// Text.
    Text,

    /// An integer literal.
    IntegerLiteral,
    /// A floating-point literal.
    FloatLiteral,
    /// A string literal.
    StringLiteral,
    /// An identifier.
    Identifier,

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

    /// `i32.const` keyword.
    I32ConstKw,
    /// `i64.const` keyword.
    I64ConstKw,
    /// `f32.const` keyword.
    F32ConstKw,
    /// `f64.const` keyword.
    F64ConstKw,

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

    /// `(`.
    LeftParen,
    /// `)`.
    RightParen,

    /// A WebAssembly module.
    Module,
    /// A function.
    Func,
    /// An export.
    Export,
    /// An import.
    Import,
    /// A type definition.
    Type,
    /// A parameter.
    Param,
    /// A result.
    Result,
    /// A local variable.
    Local,
    /// A global variable.
    Global,
    /// Memory definition.
    Memory,
    /// Table definition.
    Table,
    /// Element segment.
    Elem,
    /// Data segment.
    Data,
    /// Start function.
    Start,
    /// A block.
    Block,
    /// A loop.
    Loop,
    /// An if statement.
    If,
    /// An instruction.
    Instruction,
    /// Root node.
    Root,
    /// A generic item.
    Item,
}

impl WatElementType {
    /// Returns the element type for the given keyword.
    pub fn from_keyword(text: &str) -> Option<Self> {
        match text {
            "module" => Some(Self::ModuleKw),
            "func" => Some(Self::FuncKw),
            "export" => Some(Self::ExportKw),
            "import" => Some(Self::ImportKw),
            "type" => Some(Self::TypeKw),
            "param" => Some(Self::ParamKw),
            "result" => Some(Self::ResultKw),
            "local" => Some(Self::LocalKw),
            "global" => Some(Self::GlobalKw),
            "memory" => Some(Self::MemoryKw),
            "table" => Some(Self::TableKw),
            "elem" => Some(Self::ElemKw),
            "data" => Some(Self::DataKw),
            "start" => Some(Self::StartKw),
            "block" => Some(Self::BlockKw),
            "loop" => Some(Self::LoopKw),
            "if" => Some(Self::IfKw),
            "then" => Some(Self::ThenKw),
            "else" => Some(Self::ElseKw),
            "end" => Some(Self::EndKw),
            _ => None,
        }
    }
}

impl ElementType for WatElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            Self::Module => UniversalElementRole::Definition,
            Self::Func => UniversalElementRole::Definition,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::WatTokenType> for WatElementType {
    fn from(token: crate::lexer::token_type::WatTokenType) -> Self {
                match token {
            crate::lexer::token_type::WatTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::WatTokenType::Newline => Self::Newline,
            crate::lexer::token_type::WatTokenType::Comment => Self::Comment,
            crate::lexer::token_type::WatTokenType::Error => Self::Error,
            crate::lexer::token_type::WatTokenType::Eof => Self::Eof,
            crate::lexer::token_type::WatTokenType::Text => Self::Text,
            crate::lexer::token_type::WatTokenType::IntegerLiteral => Self::IntegerLiteral,
            crate::lexer::token_type::WatTokenType::FloatLiteral => Self::FloatLiteral,
            crate::lexer::token_type::WatTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::WatTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::WatTokenType::ModuleKw => Self::ModuleKw,
            crate::lexer::token_type::WatTokenType::FuncKw => Self::FuncKw,
            crate::lexer::token_type::WatTokenType::ExportKw => Self::ExportKw,
            crate::lexer::token_type::WatTokenType::ImportKw => Self::ImportKw,
            crate::lexer::token_type::WatTokenType::TypeKw => Self::TypeKw,
            crate::lexer::token_type::WatTokenType::ParamKw => Self::ParamKw,
            crate::lexer::token_type::WatTokenType::ResultKw => Self::ResultKw,
            crate::lexer::token_type::WatTokenType::LocalKw => Self::LocalKw,
            crate::lexer::token_type::WatTokenType::GlobalKw => Self::GlobalKw,
            crate::lexer::token_type::WatTokenType::MemoryKw => Self::MemoryKw,
            crate::lexer::token_type::WatTokenType::TableKw => Self::TableKw,
            crate::lexer::token_type::WatTokenType::ElemKw => Self::ElemKw,
            crate::lexer::token_type::WatTokenType::DataKw => Self::DataKw,
            crate::lexer::token_type::WatTokenType::StartKw => Self::StartKw,
            crate::lexer::token_type::WatTokenType::BlockKw => Self::BlockKw,
            crate::lexer::token_type::WatTokenType::LoopKw => Self::LoopKw,
            crate::lexer::token_type::WatTokenType::IfKw => Self::IfKw,
            crate::lexer::token_type::WatTokenType::ThenKw => Self::ThenKw,
            crate::lexer::token_type::WatTokenType::ElseKw => Self::ElseKw,
            crate::lexer::token_type::WatTokenType::EndKw => Self::EndKw,
            crate::lexer::token_type::WatTokenType::BrKw => Self::BrKw,
            crate::lexer::token_type::WatTokenType::BrIfKw => Self::BrIfKw,
            crate::lexer::token_type::WatTokenType::BrTableKw => Self::BrTableKw,
            crate::lexer::token_type::WatTokenType::ReturnKw => Self::ReturnKw,
            crate::lexer::token_type::WatTokenType::CallKw => Self::CallKw,
            crate::lexer::token_type::WatTokenType::CallIndirectKw => Self::CallIndirectKw,
            crate::lexer::token_type::WatTokenType::LocalGetKw => Self::LocalGetKw,
            crate::lexer::token_type::WatTokenType::LocalSetKw => Self::LocalSetKw,
            crate::lexer::token_type::WatTokenType::LocalTeeKw => Self::LocalTeeKw,
            crate::lexer::token_type::WatTokenType::GlobalGetKw => Self::GlobalGetKw,
            crate::lexer::token_type::WatTokenType::GlobalSetKw => Self::GlobalSetKw,
            crate::lexer::token_type::WatTokenType::I32LoadKw => Self::I32LoadKw,
            crate::lexer::token_type::WatTokenType::I64LoadKw => Self::I64LoadKw,
            crate::lexer::token_type::WatTokenType::F32LoadKw => Self::F32LoadKw,
            crate::lexer::token_type::WatTokenType::F64LoadKw => Self::F64LoadKw,
            crate::lexer::token_type::WatTokenType::I32Load8SKw => Self::I32Load8SKw,
            crate::lexer::token_type::WatTokenType::I32Load8UKw => Self::I32Load8UKw,
            crate::lexer::token_type::WatTokenType::I32Load16SKw => Self::I32Load16SKw,
            crate::lexer::token_type::WatTokenType::I32Load16UKw => Self::I32Load16UKw,
            crate::lexer::token_type::WatTokenType::I64Load8SKw => Self::I64Load8SKw,
            crate::lexer::token_type::WatTokenType::I64Load8UKw => Self::I64Load8UKw,
            crate::lexer::token_type::WatTokenType::I64Load16SKw => Self::I64Load16SKw,
            crate::lexer::token_type::WatTokenType::I64Load16UKw => Self::I64Load16UKw,
            crate::lexer::token_type::WatTokenType::I64Load32SKw => Self::I64Load32SKw,
            crate::lexer::token_type::WatTokenType::I64Load32UKw => Self::I64Load32UKw,
            crate::lexer::token_type::WatTokenType::I32StoreKw => Self::I32StoreKw,
            crate::lexer::token_type::WatTokenType::I64StoreKw => Self::I64StoreKw,
            crate::lexer::token_type::WatTokenType::F32StoreKw => Self::F32StoreKw,
            crate::lexer::token_type::WatTokenType::F64StoreKw => Self::F64StoreKw,
            crate::lexer::token_type::WatTokenType::I32Store8Kw => Self::I32Store8Kw,
            crate::lexer::token_type::WatTokenType::I32Store16Kw => Self::I32Store16Kw,
            crate::lexer::token_type::WatTokenType::I64Store8Kw => Self::I64Store8Kw,
            crate::lexer::token_type::WatTokenType::I64Store16Kw => Self::I64Store16Kw,
            crate::lexer::token_type::WatTokenType::I64Store32Kw => Self::I64Store32Kw,
            crate::lexer::token_type::WatTokenType::MemorySizeKw => Self::MemorySizeKw,
            crate::lexer::token_type::WatTokenType::MemoryGrowKw => Self::MemoryGrowKw,
            crate::lexer::token_type::WatTokenType::I32ConstKw => Self::I32ConstKw,
            crate::lexer::token_type::WatTokenType::I64ConstKw => Self::I64ConstKw,
            crate::lexer::token_type::WatTokenType::F32ConstKw => Self::F32ConstKw,
            crate::lexer::token_type::WatTokenType::F64ConstKw => Self::F64ConstKw,
            crate::lexer::token_type::WatTokenType::I32AddKw => Self::I32AddKw,
            crate::lexer::token_type::WatTokenType::I64AddKw => Self::I64AddKw,
            crate::lexer::token_type::WatTokenType::F32AddKw => Self::F32AddKw,
            crate::lexer::token_type::WatTokenType::F64AddKw => Self::F64AddKw,
            crate::lexer::token_type::WatTokenType::I32SubKw => Self::I32SubKw,
            crate::lexer::token_type::WatTokenType::I64SubKw => Self::I64SubKw,
            crate::lexer::token_type::WatTokenType::F32SubKw => Self::F32SubKw,
            crate::lexer::token_type::WatTokenType::F64SubKw => Self::F64SubKw,
            crate::lexer::token_type::WatTokenType::I32MulKw => Self::I32MulKw,
            crate::lexer::token_type::WatTokenType::I64MulKw => Self::I64MulKw,
            crate::lexer::token_type::WatTokenType::F32MulKw => Self::F32MulKw,
            crate::lexer::token_type::WatTokenType::F64MulKw => Self::F64MulKw,
            crate::lexer::token_type::WatTokenType::I32DivSKw => Self::I32DivSKw,
            crate::lexer::token_type::WatTokenType::I32DivUKw => Self::I32DivUKw,
            crate::lexer::token_type::WatTokenType::I64DivSKw => Self::I64DivSKw,
            crate::lexer::token_type::WatTokenType::I64DivUKw => Self::I64DivUKw,
            crate::lexer::token_type::WatTokenType::F32DivKw => Self::F32DivKw,
            crate::lexer::token_type::WatTokenType::F64DivKw => Self::F64DivKw,
            crate::lexer::token_type::WatTokenType::I32RemSKw => Self::I32RemSKw,
            crate::lexer::token_type::WatTokenType::I32RemUKw => Self::I32RemUKw,
            crate::lexer::token_type::WatTokenType::I64RemSKw => Self::I64RemSKw,
            crate::lexer::token_type::WatTokenType::I64RemUKw => Self::I64RemUKw,
            crate::lexer::token_type::WatTokenType::I32EqKw => Self::I32EqKw,
            crate::lexer::token_type::WatTokenType::I64EqKw => Self::I64EqKw,
            crate::lexer::token_type::WatTokenType::F32EqKw => Self::F32EqKw,
            crate::lexer::token_type::WatTokenType::F64EqKw => Self::F64EqKw,
            crate::lexer::token_type::WatTokenType::I32NeKw => Self::I32NeKw,
            crate::lexer::token_type::WatTokenType::I64NeKw => Self::I64NeKw,
            crate::lexer::token_type::WatTokenType::F32NeKw => Self::F32NeKw,
            crate::lexer::token_type::WatTokenType::F64NeKw => Self::F64NeKw,
            crate::lexer::token_type::WatTokenType::I32LtSKw => Self::I32LtSKw,
            crate::lexer::token_type::WatTokenType::I32LtUKw => Self::I32LtUKw,
            crate::lexer::token_type::WatTokenType::I64LtSKw => Self::I64LtSKw,
            crate::lexer::token_type::WatTokenType::I64LtUKw => Self::I64LtUKw,
            crate::lexer::token_type::WatTokenType::F32LtKw => Self::F32LtKw,
            crate::lexer::token_type::WatTokenType::F64LtKw => Self::F64LtKw,
            crate::lexer::token_type::WatTokenType::I32GtSKw => Self::I32GtSKw,
            crate::lexer::token_type::WatTokenType::I32GtUKw => Self::I32GtUKw,
            crate::lexer::token_type::WatTokenType::I64GtSKw => Self::I64GtSKw,
            crate::lexer::token_type::WatTokenType::I64GtUKw => Self::I64GtUKw,
            crate::lexer::token_type::WatTokenType::F32GtKw => Self::F32GtKw,
            crate::lexer::token_type::WatTokenType::F64GtKw => Self::F64GtKw,
            crate::lexer::token_type::WatTokenType::I32LeSKw => Self::I32LeSKw,
            crate::lexer::token_type::WatTokenType::I32LeUKw => Self::I32LeUKw,
            crate::lexer::token_type::WatTokenType::I64LeSKw => Self::I64LeSKw,
            crate::lexer::token_type::WatTokenType::I64LeUKw => Self::I64LeUKw,
            crate::lexer::token_type::WatTokenType::F32LeKw => Self::F32LeKw,
            crate::lexer::token_type::WatTokenType::F64LeKw => Self::F64LeKw,
            crate::lexer::token_type::WatTokenType::I32GeSKw => Self::I32GeSKw,
            crate::lexer::token_type::WatTokenType::I32GeUKw => Self::I32GeUKw,
            crate::lexer::token_type::WatTokenType::I64GeSKw => Self::I64GeSKw,
            crate::lexer::token_type::WatTokenType::I64GeUKw => Self::I64GeUKw,
            crate::lexer::token_type::WatTokenType::F32GeKw => Self::F32GeKw,
            crate::lexer::token_type::WatTokenType::F64GeKw => Self::F64GeKw,
            crate::lexer::token_type::WatTokenType::I32AndKw => Self::I32AndKw,
            crate::lexer::token_type::WatTokenType::I64AndKw => Self::I64AndKw,
            crate::lexer::token_type::WatTokenType::I32OrKw => Self::I32OrKw,
            crate::lexer::token_type::WatTokenType::I64OrKw => Self::I64OrKw,
            crate::lexer::token_type::WatTokenType::I32XorKw => Self::I32XorKw,
            crate::lexer::token_type::WatTokenType::I64XorKw => Self::I64XorKw,
            crate::lexer::token_type::WatTokenType::I32ShlKw => Self::I32ShlKw,
            crate::lexer::token_type::WatTokenType::I64ShlKw => Self::I64ShlKw,
            crate::lexer::token_type::WatTokenType::I32ShrSKw => Self::I32ShrSKw,
            crate::lexer::token_type::WatTokenType::I32ShrUKw => Self::I32ShrUKw,
            crate::lexer::token_type::WatTokenType::I64ShrSKw => Self::I64ShrSKw,
            crate::lexer::token_type::WatTokenType::I64ShrUKw => Self::I64ShrUKw,
            crate::lexer::token_type::WatTokenType::I32RotlKw => Self::I32RotlKw,
            crate::lexer::token_type::WatTokenType::I64RotlKw => Self::I64RotlKw,
            crate::lexer::token_type::WatTokenType::I32RotrKw => Self::I32RotrKw,
            crate::lexer::token_type::WatTokenType::I64RotrKw => Self::I64RotrKw,
            crate::lexer::token_type::WatTokenType::I32WrapI64Kw => Self::I32WrapI64Kw,
            crate::lexer::token_type::WatTokenType::I64ExtendI32SKw => Self::I64ExtendI32SKw,
            crate::lexer::token_type::WatTokenType::I64ExtendI32UKw => Self::I64ExtendI32UKw,
            crate::lexer::token_type::WatTokenType::I32TruncF32SKw => Self::I32TruncF32SKw,
            crate::lexer::token_type::WatTokenType::I32TruncF32UKw => Self::I32TruncF32UKw,
            crate::lexer::token_type::WatTokenType::I32TruncF64SKw => Self::I32TruncF64SKw,
            crate::lexer::token_type::WatTokenType::I32TruncF64UKw => Self::I32TruncF64UKw,
            crate::lexer::token_type::WatTokenType::I64TruncF32SKw => Self::I64TruncF32SKw,
            crate::lexer::token_type::WatTokenType::I64TruncF32UKw => Self::I64TruncF32UKw,
            crate::lexer::token_type::WatTokenType::I64TruncF64SKw => Self::I64TruncF64SKw,
            crate::lexer::token_type::WatTokenType::I64TruncF64UKw => Self::I64TruncF64UKw,
            crate::lexer::token_type::WatTokenType::F32ConvertI32SKw => Self::F32ConvertI32SKw,
            crate::lexer::token_type::WatTokenType::F32ConvertI32UKw => Self::F32ConvertI32UKw,
            crate::lexer::token_type::WatTokenType::F32ConvertI64SKw => Self::F32ConvertI64SKw,
            crate::lexer::token_type::WatTokenType::F32ConvertI64UKw => Self::F32ConvertI64UKw,
            crate::lexer::token_type::WatTokenType::F64ConvertI32SKw => Self::F64ConvertI32SKw,
            crate::lexer::token_type::WatTokenType::F64ConvertI32UKw => Self::F64ConvertI32UKw,
            crate::lexer::token_type::WatTokenType::F64ConvertI64SKw => Self::F64ConvertI64SKw,
            crate::lexer::token_type::WatTokenType::F64ConvertI64UKw => Self::F64ConvertI64UKw,
            crate::lexer::token_type::WatTokenType::F32DemoteF64Kw => Self::F32DemoteF64Kw,
            crate::lexer::token_type::WatTokenType::F64PromoteF32Kw => Self::F64PromoteF32Kw,
            crate::lexer::token_type::WatTokenType::DropKw => Self::DropKw,
            crate::lexer::token_type::WatTokenType::SelectKw => Self::SelectKw,
            crate::lexer::token_type::WatTokenType::UnreachableKw => Self::UnreachableKw,
            crate::lexer::token_type::WatTokenType::NopKw => Self::NopKw,
            crate::lexer::token_type::WatTokenType::I32Kw => Self::I32Kw,
            crate::lexer::token_type::WatTokenType::I64Kw => Self::I64Kw,
            crate::lexer::token_type::WatTokenType::F32Kw => Self::F32Kw,
            crate::lexer::token_type::WatTokenType::F64Kw => Self::F64Kw,
            crate::lexer::token_type::WatTokenType::FuncrefKw => Self::FuncrefKw,
            crate::lexer::token_type::WatTokenType::ExternrefKw => Self::ExternrefKw,
            crate::lexer::token_type::WatTokenType::MutKw => Self::MutKw,
            crate::lexer::token_type::WatTokenType::OffsetKw => Self::OffsetKw,
            crate::lexer::token_type::WatTokenType::AlignKw => Self::AlignKw,
            crate::lexer::token_type::WatTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::WatTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::WatTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::WatTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::WatTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::WatTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::WatTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::WatTokenType::Comma => Self::Comma,
            crate::lexer::token_type::WatTokenType::Dot => Self::Dot,
            crate::lexer::token_type::WatTokenType::Quote => Self::Quote,
            crate::lexer::token_type::WatTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::WatTokenType::Plus => Self::Plus,
            crate::lexer::token_type::WatTokenType::Minus => Self::Minus,
            crate::lexer::token_type::WatTokenType::Star => Self::Star,
            crate::lexer::token_type::WatTokenType::Slash => Self::Slash,
            crate::lexer::token_type::WatTokenType::Eq => Self::Eq,
            crate::lexer::token_type::WatTokenType::Colon => Self::Colon,
            crate::lexer::token_type::WatTokenType::Question => Self::Question,
            crate::lexer::token_type::WatTokenType::Bang => Self::Bang,
            crate::lexer::token_type::WatTokenType::At => Self::At,
            crate::lexer::token_type::WatTokenType::Hash => Self::Hash,
            crate::lexer::token_type::WatTokenType::Percent => Self::Percent,
            crate::lexer::token_type::WatTokenType::Caret => Self::Caret,
            crate::lexer::token_type::WatTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::WatTokenType::LessThan => Self::LessThan,
            crate::lexer::token_type::WatTokenType::GreaterThan => Self::GreaterThan,
            crate::lexer::token_type::WatTokenType::Backslash => Self::Backslash,
            crate::lexer::token_type::WatTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::WatTokenType::Tilde => Self::Tilde,
            crate::lexer::token_type::WatTokenType::Root => Self::Root,
            crate::lexer::token_type::WatTokenType::SourceFile => Self::SourceFile,
            crate::lexer::token_type::WatTokenType::Module => Self::Module,
            crate::lexer::token_type::WatTokenType::Item => Self::Item,
            _ => Self::Error,
        }
    }
}
