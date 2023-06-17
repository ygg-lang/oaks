#![doc = include_str!("readme.md")]

use alloc::{string::String, vec::Vec};

/// Wolfram 根节
#[derive(Clone, Debug)]
pub struct WolframRoot {
    pub items: Vec<WolframItem>,
}

/// Wolfram 项目
#[derive(Clone, Debug)]
pub enum WolframItem {
    Module(WolframModule),
    Function(WolframFunction),
    Memory(WolframMemory),
    Export(WolframExport),
    Import(WolframImport),
}

/// Wolfram 模块
#[derive(Clone, Debug)]
pub struct WolframModule {
    pub name: Option<String>,
    pub items: Vec<WolframItem>,
}

/// Wolfram 函数
#[derive(Clone, Debug)]
pub struct WolframFunction {
    pub name: Option<String>,
    pub params: Vec<WolframParam>,
    pub result: Option<WolframType>,
    pub locals: Vec<WolframLocal>,
    pub body: Vec<WolframInstruction>,
}

/// Wolfram 参数
#[derive(Clone, Debug)]
pub struct WolframParam {
    pub name: Option<String>,
    pub param_type: WolframType,
}

/// Wolfram 局部变
#[derive(Clone, Debug)]
pub struct WolframLocal {
    pub name: Option<String>,
    pub local_type: WolframType,
}

/// Wolfram 类型
#[derive(Clone, Debug)]
pub enum WolframType {
    I32,
    I64,
    F32,
    F64,
}

/// Wolfram 内存
#[derive(Clone, Debug)]
pub struct WolframMemory {
    pub name: Option<String>,
    pub min: u32,
    pub max: Option<u32>,
}

/// Wolfram 导出
#[derive(Clone, Debug)]
pub struct WolframExport {
    pub name: String,
    pub kind: WolframExportKind,
}

/// Wolfram 导出类型
#[derive(Clone, Debug)]
pub enum WolframExportKind {
    Function(String),
    Memory(String),
    Global(String),
    Table(String),
}

/// Wolfram 导入
#[derive(Clone, Debug)]
pub struct WolframImport {
    pub module: String,
    pub name: String,
    pub kind: WolframImportKind,
}

/// Wolfram 导入类型
#[derive(Clone, Debug)]
pub enum WolframImportKind {
    Function(WolframFunctionType),
    Memory(WolframMemoryType),
    Global(WolframGlobalType),
    Table(WolframTableType),
}

/// Wolfram 函数类型
#[derive(Clone, Debug)]
pub struct WolframFunctionType {
    pub params: Vec<WolframType>,
    pub results: Vec<WolframType>,
}

/// Wolfram 内存类型
#[derive(Clone, Debug)]
pub struct WolframMemoryType {
    pub min: u32,
    pub max: Option<u32>,
}

/// Wolfram 全局类型
#[derive(Clone, Debug)]
pub struct WolframGlobalType {
    pub value_type: WolframType,
    pub mutable: bool,
}

/// Wolfram 表类
#[derive(Clone, Debug)]
pub struct WolframTableType {
    pub element_type: WolframType,
    pub min: u32,
    pub max: Option<u32>,
}

/// Wolfram 指令
#[derive(Clone, Debug)]
pub enum WolframInstruction {
    /// 简单指令（nop, drop
    Simple(String),
    /// 带参数的指令（如 i32.const 42
    WithOperand { opcode: String, operand: String },
    /// 控制流指令（if, loop, block
    Control { opcode: String, label: Option<String> },
    /// 函数调用指令（如 call $func
    Call { function: String },
}
