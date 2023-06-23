#![doc = include_str!("readme.md")]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// VHDL 根节点
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VhdlRoot {
    pub units: Vec<DesignUnit>,
}

/// 设计单元
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DesignUnit {
    Entity(EntityDeclaration),
    Architecture(ArchitectureBody),
    Package(PackageDeclaration),
}

/// 实体声明
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EntityDeclaration {
    pub name: String,
    pub ports: Vec<PortDeclaration>,
}

/// 端口声明
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PortDeclaration {
    pub name: String,
    pub direction: PortDirection,
    pub data_type: String,
}

/// 端口方向
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ArchitectureBody {
    pub name: String,
    pub entity_name: String,
    pub items: Vec<ArchitectureItem>,
}

/// 架构体项目
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ArchitectureItem {
    Signal(SignalDeclaration),
    Process(ProcessStatement),
    Component(ComponentDeclaration),
}

/// 信号声明
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SignalDeclaration {
    pub name: String,
    pub data_type: String,
}

/// 进程语句
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProcessStatement {
    pub label: Option<String>,
    pub sensitivity_list: Vec<String>,
    pub body: String,
}

/// 组件声明
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComponentDeclaration {
    pub name: String,
    pub ports: Vec<PortDeclaration>,
}

/// 包声明
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PackageDeclaration {
    pub name: String,
    pub items: Vec<PackageItem>,
}

/// 包项目
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PackageItem {
    Function(String),
    Type(String),
}
