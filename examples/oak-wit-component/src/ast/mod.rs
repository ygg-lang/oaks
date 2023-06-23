#![doc = include_str!("readme.md")]
/// WIT 根节点
#[derive(Clone, Debug)]
pub struct WitRoot {
    pub items: Vec<WitItem>,
}

/// WIT 项目
#[derive(Clone, Debug)]
pub enum WitItem {
    Package(WitPackage),
    World(WitWorld),
    Interface(WitInterface),
}

/// WIT 包
#[derive(Clone, Debug)]
pub struct WitPackage {
    pub name: String,
}

/// WIT World
#[derive(Clone, Debug)]
pub struct WitWorld {
    pub name: String,
    pub items: Vec<WitWorldItem>,
}

/// WIT World 项目
#[derive(Clone, Debug)]
pub enum WitWorldItem {
    Import(WitImport),
    Export(WitExport),
    Include(WitInclude),
}

/// WIT 接口
#[derive(Clone, Debug)]
pub struct WitInterface {
    pub name: String,
    pub items: Vec<WitInterfaceItem>,
}

/// WIT 接口项目
#[derive(Clone, Debug)]
pub enum WitInterfaceItem {
    Type(WitType),
    Func(WitFunc),
}

/// WIT 函数
#[derive(Clone, Debug)]
pub struct WitFunc {
    pub name: String,
    pub params: Vec<WitParam>,
    pub result: Option<WitTypeKind>,
}

/// WIT 参数
#[derive(Clone, Debug)]
pub struct WitParam {
    pub name: String,
    pub ty: WitTypeKind,
}

/// WIT 类型
#[derive(Clone, Debug)]
pub struct WitType {
    pub name: String,
    pub kind: WitTypeKind,
}

/// WIT 类型种类
#[derive(Clone, Debug)]
pub enum WitTypeKind {
    Bool,
    U32,
    String,
    // ...
}

/// WIT 导入
#[derive(Clone, Debug)]
pub struct WitImport {
    pub name: String,
}

/// WIT 导出
#[derive(Clone, Debug)]
pub struct WitExport {
    pub name: String,
}

/// WIT 包含
#[derive(Clone, Debug)]
pub struct WitInclude {
    pub name: String,
}
