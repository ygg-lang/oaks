use oak_core::source::{SourceBuffer, ToSource};

/// Wat 根节点
#[derive(Clone, Debug)]
pub struct WatRoot {
    pub items: Vec<WatItem>,
}

impl ToSource for WatRoot {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        for item in &self.items {
            item.to_source(buffer);
            buffer.push("\n");
        }
    }
}

/// Wat 项目
#[derive(Clone, Debug)]
pub enum WatItem {
    Module(WatModule),
}

impl ToSource for WatItem {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            WatItem::Module(m) => m.to_source(buffer),
        }
    }
}

/// Wat 模块
#[derive(Clone, Debug)]
pub struct WatModule {
    pub name: Option<String>,
    pub items: Vec<WatModuleField>,
}

impl ToSource for WatModule {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(module");
        if let Some(name) = &self.name {
            buffer.push(" ");
            buffer.push(name);
        }
        for item in &self.items {
            buffer.push("\n  ");
            item.to_source(buffer);
        }
        buffer.push(")");
    }
}

/// Wat 模块字段
#[derive(Clone, Debug)]
pub enum WatModuleField {
    Func(WatFunc),
    Import(WatImport),
    Export(WatExport),
    Type(WatType),
    Table(WatTable),
    Memory(WatMemory),
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

/// Wat 函数
#[derive(Clone, Debug)]
pub struct WatFunc {
    pub name: Option<String>,
    pub params: Vec<WatParam>,
    pub results: Vec<WatResult>,
    pub locals: Vec<WatLocal>,
    pub body: Vec<WatInstruction>,
}

impl ToSource for WatFunc {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(func");
        if let Some(name) = &self.name {
            buffer.push(" ");
            buffer.push(name);
        }
        for param in &self.params {
            buffer.push(" ");
            param.to_source(buffer);
        }
        for result in &self.results {
            buffer.push(" ");
            result.to_source(buffer);
        }
        for local in &self.locals {
            buffer.push(" ");
            local.to_source(buffer);
        }
        for instr in &self.body {
            buffer.push("\n    ");
            instr.to_source(buffer);
        }
        buffer.push(")");
    }
}

/// Wat 参数
#[derive(Clone, Debug)]
pub struct WatParam {
    pub name: Option<String>,
    pub ty: WatTypeKind,
}

impl ToSource for WatParam {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(param");
        if let Some(name) = &self.name {
            buffer.push(" ");
            buffer.push(name);
        }
        buffer.push(" ");
        self.ty.to_source(buffer);
        buffer.push(")");
    }
}

/// Wat 结果
#[derive(Clone, Debug)]
pub struct WatResult {
    pub ty: WatTypeKind,
}

impl ToSource for WatResult {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(result ");
        self.ty.to_source(buffer);
        buffer.push(")");
    }
}

/// Wat 局部变量
#[derive(Clone, Debug)]
pub struct WatLocal {
    pub name: Option<String>,
    pub ty: WatTypeKind,
}

impl ToSource for WatLocal {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(local");
        if let Some(name) = &self.name {
            buffer.push(" ");
            buffer.push(name);
        }
        buffer.push(" ");
        self.ty.to_source(buffer);
        buffer.push(")");
    }
}

/// Wat 类型种类
#[derive(Clone, Debug)]
pub enum WatTypeKind {
    I32,
    I64,
    F32,
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

/// Wat 指令 (简化版)
#[derive(Clone, Debug)]
pub struct WatInstruction {
    pub name: String,
}

impl ToSource for WatInstruction {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(&self.name);
    }
}

/// Wat 导入
#[derive(Clone, Debug)]
pub struct WatImport {
    pub module: String,
    pub field: String,
}

impl ToSource for WatImport {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(import \"");
        buffer.push(&self.module);
        buffer.push("\" \"");
        buffer.push(&self.field);
        buffer.push("\")");
    }
}

/// Wat 导出
#[derive(Clone, Debug)]
pub struct WatExport {
    pub name: String,
}

impl ToSource for WatExport {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(export \"");
        buffer.push(&self.name);
        buffer.push("\")");
    }
}

/// Wat 类型定义
#[derive(Clone, Debug)]
pub struct WatType {
    pub name: Option<String>,
}

impl ToSource for WatType {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(type");
        if let Some(name) = &self.name {
            buffer.push(" ");
            buffer.push(name);
        }
        buffer.push(")");
    }
}

/// Wat 表
#[derive(Clone, Debug)]
pub struct WatTable {
    pub name: Option<String>,
}

impl ToSource for WatTable {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(table");
        if let Some(name) = &self.name {
            buffer.push(" ");
            buffer.push(name);
        }
        buffer.push(")");
    }
}

/// Wat 内存
#[derive(Clone, Debug)]
pub struct WatMemory {
    pub name: Option<String>,
}

impl ToSource for WatMemory {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(memory");
        if let Some(name) = &self.name {
            buffer.push(" ");
            buffer.push(name);
        }
        buffer.push(")");
    }
}

/// Wat 全局变量
#[derive(Clone, Debug)]
pub struct WatGlobal {
    pub name: Option<String>,
}

impl ToSource for WatGlobal {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("(global");
        if let Some(name) = &self.name {
            buffer.push(" ");
            buffer.push(name);
        }
        buffer.push(")");
    }
}
