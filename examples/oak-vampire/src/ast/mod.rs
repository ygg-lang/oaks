#![doc = include_str!("readme.md")]

use alloc::{string::String, vec::Vec};

/// Valkyrie 根节#[derive(Clone, Debug)]
pub struct ValkyrieRoot {
    pub items: Vec<ValkyrieItem>,
}

/// Valkyrie 项目
#[derive(Clone, Debug)]
pub enum ValkyrieItem {
    Module(ValkyrieModule),
    Function(ValkyrieFunction),
    Memory(ValkyrieMemory),
    Export(ValkyrieExport),
    Import(ValkyrieImport),
}

/// Valkyrie 模块
#[derive(Clone, Debug)]
pub struct ValkyrieModule {
    pub name: Option<String>,
    pub items: Vec<ValkyrieItem>,
}

/// Valkyrie 函数
#[derive(Clone, Debug)]
pub struct ValkyrieFunction {
    pub name: Option<String>,
    pub params: Vec<ValkyrieParam>,
    pub result: Option<ValkyrieType>,
    pub locals: Vec<ValkyrieLocal>,
    pub body: Vec<ValkyrieInstruction>,
}

/// Valkyrie 参数
#[derive(Clone, Debug)]
pub struct ValkyrieParam {
    pub name: Option<String>,
    pub param_type: ValkyrieType,
}

/// Valkyrie 局部变量
#[derive(Clone, Debug)]
pub struct ValkyrieLocal {
    pub name: Option<String>,
    pub local_type: ValkyrieType,
}

/// Valkyrie 类型
#[derive(Clone, Debug)]
pub enum ValkyrieType {
    I32,
    I64,
    F32,
    F64,
}

/// Valkyrie 内存
#[derive(Clone, Debug)]
pub struct ValkyrieMemory {
    pub name: Option<String>,
    pub min: u32,
    pub max: Option<u32>,
}

/// Valkyrie 导出
#[derive(Clone, Debug)]
pub struct ValkyrieExport {
    pub name: String,
    pub kind: ValkyrieExportKind,
}

/// Valkyrie 导出类型
#[derive(Clone, Debug)]
pub enum ValkyrieExportKind {
    Function(String),
    Memory(String),
    Global(String),
    Table(String),
}

/// Valkyrie 导入
#[derive(Clone, Debug)]
pub struct ValkyrieImport {
    pub module: String,
    pub name: String,
    pub kind: ValkyrieImportKind,
}

/// Valkyrie 导入类型
#[derive(Clone, Debug)]
pub enum ValkyrieImportKind {
    Function(ValkyrieFunctionType),
    Memory(ValkyrieMemoryType),
    Global(ValkyrieGlobalType),
    Table(ValkyrieTableType),
}

/// Valkyrie 函数类型
#[derive(Clone, Debug)]
pub struct ValkyrieFunctionType {
    pub params: Vec<ValkyrieType>,
    pub results: Vec<ValkyrieType>,
}

/// Valkyrie 内存类型
#[derive(Clone, Debug)]
pub struct ValkyrieMemoryType {
    pub min: u32,
    pub max: Option<u32>,
}

/// Valkyrie 全局类型
#[derive(Clone, Debug)]
pub struct ValkyrieGlobalType {
    pub value_type: ValkyrieType,
    pub mutable: bool,
}

/// Valkyrie 表类型
#[derive(Clone, Debug)]
pub struct ValkyrieTableType {
    pub element_type: ValkyrieType,
    pub min: u32,
    pub max: Option<u32>,
}

/// Valkyrie 指令
#[derive(Clone, Debug)]
pub enum ValkyrieInstruction {
    /// 简单指令（nop, drop）
    Simple(String),
    /// 带参数的指令（如 i32.const 42）
    WithOperand { opcode: String, operand: String },
    /// 控制流指令（if, loop, block）
    Control { opcode: String, label: Option<String> },
    /// 函数调用指令（如 call $func）
    Call { function: String },
}
