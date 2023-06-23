#![doc = include_str!("readme.md")]
//! Objective-C AST 定义

/// Objective-C 语法树的根节点
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectiveCRoot {
    /// 源文件中的所有顶级项目
    pub items: Vec<ObjectiveCItem>,
}

/// Objective-C 顶级项目
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectiveCItem {
    /// 接口定义 (@interface)
    Interface,
    /// 实现定义 (@implementation)
    Implementation,
    /// 协议定义 (@protocol)
    Protocol,
    /// 函数定义
    Function,
    /// 变量声明
    Variable,
    /// 导入语句 (#import/#include)
    Import,
}

impl Default for ObjectiveCRoot {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}
