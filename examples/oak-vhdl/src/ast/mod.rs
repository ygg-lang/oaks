#![doc = include_str!("readme.md")]

/// VHDL 根节点
#[derive(Clone, Debug, Default)]
pub struct VhdlRoot {
    pub units: Vec<DesignUnit>,
}

/// 设计单元
#[derive(Clone, Debug)]
pub enum DesignUnit {
    Entity(EntityDeclaration),
    Architecture(ArchitectureBody),
    Package(PackageDeclaration),
}

/// 实体声明
#[derive(Clone, Debug, Default)]
pub struct EntityDeclaration {
    pub name: String,
    pub ports: Vec<PortDeclaration>,
}

/// 端口声明
#[derive(Clone, Debug, Default)]
pub struct PortDeclaration {
    pub name: String,
    pub direction: PortDirection,
    pub data_type: String,
}

/// 端口方向
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum PortDirection {
    #[default]
    In,
    Out,
    Inout,
    Buffer,
    Linkage,
}

/// 结构体/架构体
#[derive(Clone, Debug, Default)]
pub struct ArchitectureBody {
    pub name: String,
    pub entity_name: String,
    pub items: Vec<ArchitectureItem>,
}

/// 架构体项目
#[derive(Clone, Debug)]
pub enum ArchitectureItem {
    Signal(SignalDeclaration),
    Process(ProcessStatement),
    Component(ComponentDeclaration),
}

/// 信号声明
#[derive(Clone, Debug, Default)]
pub struct SignalDeclaration {
    pub name: String,
    pub data_type: String,
}

/// 进程语句
#[derive(Clone, Debug, Default)]
pub struct ProcessStatement {
    pub label: Option<String>,
    pub sensitivity_list: Vec<String>,
    pub body: String,
}

/// 组件声明
#[derive(Clone, Debug, Default)]
pub struct ComponentDeclaration {
    pub name: String,
    pub ports: Vec<PortDeclaration>,
}

/// 包声明
#[derive(Clone, Debug, Default)]
pub struct PackageDeclaration {
    pub name: String,
    pub items: Vec<PackageItem>,
}

/// 包项目
#[derive(Clone, Debug)]
pub enum PackageItem {
    Function(String),
    Type(String),
}
