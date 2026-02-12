#![doc = include_str!("readme.md")]
use oak_core::source::{SourceBuffer, ToSource};

/// Root node of the WAT AST.
#[derive(Clone, Debug)]
pub struct WatRoot {
    /// Items in the WAT file.
    pub items: Vec<WatItem>,
}

impl ToSource for WatRoot {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        for item in &self.items {
            item.to_source(buffer);
            buffer.push("\n")
        }
    }
}

/// An item in a WAT file.
#[derive(Clone, Debug)]
pub enum WatItem {
    /// A module definition.
    Module(WatModule),
}

impl ToSource for WatItem {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            WatItem::Module(m) => m.to_source(buffer),
        }
    }
}

/// A WebAssembly module.
#[derive(Clone, Debug)]
pub struct WatModule {
    /// Optional name of the module.
    pub name: Option<String>,
    /// Fields within the module.
    pub items: Vec<WatModuleField>,
}

impl ToSource for WatModule {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(module");
        if let Some(name) = &self.name {
            buffer.push(" ");
            buffer.push(name)
        }
        for item in &self.items {
            buffer.push("\n  ");
            item.to_source(buffer)
        }
        buffer.push(")")
    }
}

/// A field within a WebAssembly module.
#[derive(Clone, Debug)]
pub enum WatModuleField {
    /// A function definition.
    Func(WatFunc),
    /// An import definition.
    Import(WatImport),
    /// An export definition.
    Export(WatExport),
    /// A type definition.
    Type(WatType),
    /// A table definition.
    Table(WatTable),
    /// A memory definition.
    Memory(WatMemory),
    /// A global variable definition.
    Global(WatGlobal),
}

impl ToSource for WatModuleField {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            WatModuleField::Func(f) => f.to_source(buffer),
            WatModuleField::Import(i) => i.to_source(buffer),
            WatModuleField::Export(e) => e.to_source(buffer),
            WatModuleField::Type(t) => t.to_source(buffer),
            WatModuleField::Table(t) => t.to_source(buffer),
            WatModuleField::Memory(m) => m.to_source(buffer),
            WatModuleField::Global(g) => g.to_source(buffer),
        }
    }
}

/// A WebAssembly function.
#[derive(Clone, Debug)]
pub struct WatFunc {
    /// Optional name of the function.
    pub name: Option<String>,
    /// Parameters of the function.
    pub params: Vec<WatParam>,
    /// Result types of the function.
    pub results: Vec<WatResult>,
    /// Local variables of the function.
    pub locals: Vec<WatLocal>,
    /// Instructions in the function body.
    pub body: Vec<WatInstruction>,
}

impl ToSource for WatFunc {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(func");
        if let Some(name) = &self.name {
            buffer.push(" ");
            buffer.push(name)
        }
        for param in &self.params {
            buffer.push(" ");
            param.to_source(buffer)
        }
        for result in &self.results {
            buffer.push(" ");
            result.to_source(buffer)
        }
        for local in &self.locals {
            buffer.push(" ");
            local.to_source(buffer)
        }
        for instr in &self.body {
            buffer.push("\n    ");
            instr.to_source(buffer)
        }
        buffer.push(")")
    }
}

/// A parameter in a WAT function.
#[derive(Clone, Debug)]
pub struct WatParam {
    /// Optional name of the parameter.
    pub name: Option<String>,
    /// Type of the parameter.
    pub ty: WatTypeKind,
}

impl ToSource for WatParam {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(param");
        if let Some(name) = &self.name {
            buffer.push(" ");
            buffer.push(name)
        }
        buffer.push(" ");
        self.ty.to_source(buffer);
        buffer.push(")")
    }
}

/// A result type in a WAT function.
#[derive(Clone, Debug)]
pub struct WatResult {
    /// Type of the result.
    pub ty: WatTypeKind,
}

impl ToSource for WatResult {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(result ");
        self.ty.to_source(buffer);
        buffer.push(")")
    }
}

/// A local variable in a WAT function.
#[derive(Clone, Debug)]
pub struct WatLocal {
    /// Optional name of the local variable.
    pub name: Option<String>,
    /// Type of the local variable.
    pub ty: WatTypeKind,
}

impl ToSource for WatLocal {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(local");
        if let Some(name) = &self.name {
            buffer.push(" ");
            buffer.push(name)
        }
        buffer.push(" ");
        self.ty.to_source(buffer);
        buffer.push(")")
    }
}

/// Supported types in WAT.
#[derive(Clone, Debug)]
pub enum WatTypeKind {
    /// 32-bit integer.
    I32,
    /// 64-bit integer.
    I64,
    /// 32-bit float.
    F32,
    /// 64-bit float.
    F64,
}

impl ToSource for WatTypeKind {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            WatTypeKind::I32 => buffer.push("i32"),
            WatTypeKind::I64 => buffer.push("i64"),
            WatTypeKind::F32 => buffer.push("f32"),
            WatTypeKind::F64 => buffer.push("f64"),
        }
    }
}

/// A WebAssembly instruction.
#[derive(Clone, Debug)]
pub enum WatInstruction {
    /// unreachable
    Unreachable,
    /// nop
    Nop,
    /// drop
    Drop,
    /// select
    Select,
    /// return
    Return,
    /// local.get
    LocalGet(String),
    /// local.set
    LocalSet(String),
    /// local.tee
    LocalTee(String),
    /// global.get
    GlobalGet(String),
    /// global.set
    GlobalSet(String),
    /// i32.const
    I32Const(i32),
    /// i64.const
    I64Const(i64),
    /// f32.const
    F32Const(f32),
    /// f64.const
    F64Const(f64),
    /// i32.add
    I32Add,
    /// i32.sub
    I32Sub,
    /// i32.mul
    I32Mul,
    /// i64.add
    I64Add,
    /// i64.sub
    I64Sub,
    /// i64.mul
    I64Mul,
    /// Other instruction
    Other(String, Vec<String>),
}

impl ToSource for WatInstruction {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            WatInstruction::Unreachable => buffer.push("unreachable"),
            WatInstruction::Nop => buffer.push("nop"),
            WatInstruction::Drop => buffer.push("drop"),
            WatInstruction::Select => buffer.push("select"),
            WatInstruction::Return => buffer.push("return"),
            WatInstruction::LocalGet(id) => {
                buffer.push("local.get ");
                buffer.push(id);
            }
            WatInstruction::LocalSet(id) => {
                buffer.push("local.set ");
                buffer.push(id);
            }
            WatInstruction::LocalTee(id) => {
                buffer.push("local.tee ");
                buffer.push(id);
            }
            WatInstruction::GlobalGet(id) => {
                buffer.push("global.get ");
                buffer.push(id);
            }
            WatInstruction::GlobalSet(id) => {
                buffer.push("global.set ");
                buffer.push(id);
            }
            WatInstruction::I32Const(val) => {
                buffer.push("i32.const ");
                buffer.push(&val.to_string());
            }
            WatInstruction::I64Const(val) => {
                buffer.push("i64.const ");
                buffer.push(&val.to_string());
            }
            WatInstruction::F32Const(val) => {
                buffer.push("f32.const ");
                buffer.push(&val.to_string());
            }
            WatInstruction::F64Const(val) => {
                buffer.push("f64.const ");
                buffer.push(&val.to_string());
            }
            WatInstruction::I32Add => buffer.push("i32.add"),
            WatInstruction::I32Sub => buffer.push("i32.sub"),
            WatInstruction::I32Mul => buffer.push("i32.mul"),
            WatInstruction::I64Add => buffer.push("i64.add"),
            WatInstruction::I64Sub => buffer.push("i64.sub"),
            WatInstruction::I64Mul => buffer.push("i64.mul"),
            WatInstruction::Other(name, args) => {
                buffer.push(name);
                for arg in args {
                    buffer.push(" ");
                    buffer.push(arg);
                }
            }
        }
    }
}

/// A WebAssembly import.
#[derive(Clone, Debug)]
pub struct WatImport {
    /// Module name being imported from.
    pub module: String,
    /// Field name being imported.
    pub name: String,
    /// Kind of import (e.g., "func", "memory").
    pub kind: String,
}

impl ToSource for WatImport {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(import \"");
        buffer.push(&self.module);
        buffer.push("\" \"");
        buffer.push(&self.name);
        buffer.push("\" (");
        buffer.push(&self.kind);
        buffer.push("))")
    }
}

/// A WebAssembly export.
#[derive(Clone, Debug)]
pub struct WatExport {
    /// Name being exported as.
    pub name: String,
    /// Kind of export (e.g., "func", "memory").
    pub kind: String,
    /// ID of the exported item (index or name).
    pub id: String,
}

impl ToSource for WatExport {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(export \"");
        buffer.push(&self.name);
        buffer.push("\" (");
        buffer.push(&self.kind);
        buffer.push(" ");
        buffer.push(&self.id);
        buffer.push("))")
    }
}

/// A WebAssembly type definition.
#[derive(Clone, Debug)]
pub struct WatType {
    /// Optional name of the type.
    pub id: Option<String>,
}

impl ToSource for WatType {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(type");
        if let Some(id) = &self.id {
            buffer.push(" ");
            buffer.push(id)
        }
        buffer.push(")")
    }
}

/// A WebAssembly table definition.
#[derive(Clone, Debug)]
pub struct WatTable {
    /// Optional ID of the table.
    pub id: Option<String>,
    /// Span of the table.
    pub span: oak_core::Range<usize>,
}

impl ToSource for WatTable {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(table");
        if let Some(id) = &self.id {
            buffer.push(" ");
            buffer.push(id)
        }
        buffer.push(")")
    }
}

/// A WebAssembly memory definition.
#[derive(Clone, Debug)]
pub struct WatMemory {
    /// Optional ID of the memory.
    pub id: Option<String>,
    /// Span of the memory.
    pub span: oak_core::Range<usize>,
}

impl ToSource for WatMemory {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(memory");
        if let Some(id) = &self.id {
            buffer.push(" ");
            buffer.push(id)
        }
        buffer.push(")")
    }
}

/// A WebAssembly global variable definition.
#[derive(Clone, Debug)]
pub struct WatGlobal {
    /// Optional name of the global variable.
    pub id: Option<String>,
    /// Type of the global variable.
    pub ty: WatTypeKind,
    /// Whether the global variable is mutable.
    pub mutable: bool,
}

impl ToSource for WatGlobal {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(global");
        if let Some(id) = &self.id {
            buffer.push(" ");
            buffer.push(id)
        }
        buffer.push(" ");
        if self.mutable {
            buffer.push("(mut ");
            self.ty.to_source(buffer);
            buffer.push(")");
        }
        else {
            self.ty.to_source(buffer);
        }
        buffer.push(")")
    }
}
