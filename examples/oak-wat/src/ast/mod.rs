/// Wat 根节点
#[derive(Clone, Debug)]
pub struct WatRoot {
    pub items: Vec<WatItem>,
}

/// Wat 项目
#[derive(Clone, Debug)]
pub enum WatItem {
    Module(WatModule),
}

/// Wat 模块
#[derive(Clone, Debug)]
pub struct WatModule {
    pub name: Option<String>,
    pub items: Vec<WatModuleField>,
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

/// Wat 函数
#[derive(Clone, Debug)]
pub struct WatFunc {
    pub name: Option<String>,
    pub params: Vec<WatParam>,
    pub results: Vec<WatResult>,
    pub locals: Vec<WatLocal>,
    pub body: Vec<WatInstruction>,
}

/// Wat 参数
#[derive(Clone, Debug)]
pub struct WatParam {
    pub name: Option<String>,
    pub ty: WatTypeKind,
}

/// Wat 结果
#[derive(Clone, Debug)]
pub struct WatResult {
    pub ty: WatTypeKind,
}

/// Wat 局部变量
#[derive(Clone, Debug)]
pub struct WatLocal {
    pub name: Option<String>,
    pub ty: WatTypeKind,
}

/// Wat 类型种类
#[derive(Clone, Debug)]
pub enum WatTypeKind {
    I32,
    I64,
    F32,
    F64,
}

/// Wat 指令 (简化版)
#[derive(Clone, Debug)]
pub struct WatInstruction {
    pub name: String,
}

/// Wat 导入
#[derive(Clone, Debug)]
pub struct WatImport {
    pub module: String,
    pub field: String,
}

/// Wat 导出
#[derive(Clone, Debug)]
pub struct WatExport {
    pub name: String,
}

/// Wat 类型定义
#[derive(Clone, Debug)]
pub struct WatType {
    pub name: Option<String>,
}

/// Wat 表
#[derive(Clone, Debug)]
pub struct WatTable {
    pub name: Option<String>,
}

/// Wat 内存
#[derive(Clone, Debug)]
pub struct WatMemory {
    pub name: Option<String>,
}

/// Wat 全局变量
#[derive(Clone, Debug)]
pub struct WatGlobal {
    pub name: Option<String>,
}
