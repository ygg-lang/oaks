#![doc = include_str!("readme.md")]
//! F# AST definitions

use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// F# 程序的根节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FSharpRoot {
    /// 编译单元中的项目
    pub items: Vec<Item>,
}

/// F# 程序中的顶级项目
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Item {
    /// 命名空间声明
    Namespace(NamespaceDeclaration),
    /// 模块声明
    Module(ModuleDeclaration),
    /// 开放指令 (open)
    Open(OpenDirective),
    /// 绑定 (let)
    Binding(Binding),
}

/// 命名空间声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NamespaceDeclaration {
    /// 命名空间名
    pub name: String,
    /// 成员
    pub items: Vec<Item>,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 模块声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModuleDeclaration {
    /// 模块名
    pub name: String,
    /// 是否为顶级模块
    pub is_top_level: bool,
    /// 成员
    pub items: Vec<Item>,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 开放指令 (open)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OpenDirective {
    /// 导入路径
    pub path: String,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 绑定 (let)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Binding {
    /// 绑定名
    pub name: String,
    /// 是否为递归绑定 (rec)
    pub is_rec: bool,
    /// 参数列表
    pub parameters: Vec<String>,
    /// 绑定的表达式
    pub expression: Expression,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// F# 表达式
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Expression {
    /// 字面量或标识符
    Simple(String),
    /// If 表达式
    If { condition: Box<Expression>, then_branch: Box<Expression>, else_branch: Option<Box<Expression>> },
}
