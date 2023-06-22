//! Java AST definitions

use core::range::Range;
use serde::{Deserialize, Serialize};

/// Java 程序的根节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JavaRoot {
    /// 编译单元中的项目
    pub items: Vec<Item>,
}

/// Java 程序中的顶级项目
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    /// 类声明
    Class(ClassDeclaration),
    /// 接口声明
    Interface(InterfaceDeclaration),
    /// 包声明
    Package(PackageDeclaration),
    /// 导入声明
    Import(ImportDeclaration),
}

/// 类声明
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassDeclaration {
    /// 类名
    pub name: String,
    /// 源码位置
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 接口声明
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceDeclaration {
    /// 接口名
    pub name: String,
    /// 源码位置
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 包声明
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageDeclaration {
    /// 包名
    pub name: String,
    /// 源码位置
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 导入声明
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportDeclaration {
    /// 导入路径
    pub path: String,
    /// 是否为静态导入
    pub is_static: bool,
    /// 源码位置
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
