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
        unsafe { std::mem::transmute(token) }
    }
}
