#![doc = include_str!("readme.md")]

use alloc::{string::String, vec::Vec};

/// WAT 根节
#[derive(Clone, Debug)]
pub struct WatRoot {
    pub items: Vec<WatItem>,
}

/// WAT 项目
#[derive(Clone, Debug)]
pub enum WatItem {
    Module(WatModule),
    Function(WatFunction),
    Memory(WatMemory),
    Export(WatExport),
    Import(WatImport),
}

/// WAT 模块
#[derive(Clone, Debug)]
pub struct WatModule {
    pub name: Option<String>,
    pub items: Vec<WatItem>,
}

/// WAT 函数
#[derive(Clone, Debug)]
pub struct WatFunction {
    pub name: Option<String>,
    pub params: Vec<WatParam>,
    pub result: Option<WatType>,
    pub locals: Vec<WatLocal>,
    pub body: Vec<WatInstruction>,
}

/// WAT 参数
#[derive(Clone, Debug)]
pub struct WatParam {
    pub name: Option<String>,
    pub param_type: WatType,
}

/// WAT 局部变
#[derive(Clone, Debug)]
pub struct WatLocal {
    pub name: Option<String>,
    pub local_type: WatType,
}

/// WAT 类型
#[derive(Clone, Debug)]
pub enum WatType {
    I32,
    I64,
    F32,
    F64,
}

/// WAT 内存
#[derive(Clone, Debug)]
pub struct WatMemory {
    pub name: Option<String>,
    pub min: u32,
    pub max: Option<u32>,
}

/// WAT 导出
#[derive(Clone, Debug)]
pub struct WatExport {
    pub name: String,
    pub kind: WatExportKind,
}

/// WAT 导出类型
#[derive(Clone, Debug)]
pub enum WatExportKind {
    Function(String),
    Memory(String),
    Global(String),
    Table(String),
}

/// WAT 导入
#[derive(Clone, Debug)]
pub struct WatImport {
    pub module: String,
    pub name: String,
    pub kind: WatImportKind,
}

/// WAT 导入类型
#[derive(Clone, Debug)]
pub enum WatImportKind {
    Function(WatFunctionType),
    Memory(WatMemoryType),
    Global(WatGlobalType),
    Table(WatTableType),
}

/// WAT 函数类型
#[derive(Clone, Debug)]
pub struct WatFunctionType {
    pub params: Vec<WatType>,
    pub results: Vec<WatType>,
}

/// WAT 内存类型
#[derive(Clone, Debug)]
pub struct WatMemoryType {
    pub min: u32,
    pub max: Option<u32>,
}

/// WAT 全局类型
#[derive(Clone, Debug)]
pub struct WatGlobalType {
    pub value_type: WatType,
    pub mutable: bool,
}

/// WAT 表类
#[derive(Clone, Debug)]
pub struct WatTableType {
    pub element_type: WatType,
    pub min: u32,
    pub max: Option<u32>,
}

/// WAT 指令
#[derive(Clone, Debug)]
pub enum WatInstruction {
    /// 简单指令（nop, drop
    Simple(String),
    /// 带参数的指令（如 i32.const 42
    WithOperand { opcode: String, operand: String },
    /// 控制流指令（if, loop, block
    Control { opcode: String, label: Option<String> },
    /// 函数调用指令（如 call $func
    Call { function: String },
}
