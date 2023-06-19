//! ActionScript AST 定义

/// ActionScript 语法树的根节点
#[derive(Debug, Clone, PartialEq)]
pub struct ActionScriptRoot {
    /// 源文件中的所有顶级项目
    pub items: Vec<ActionScriptItem>,
}

/// ActionScript 顶级项目
#[derive(Debug, Clone, PartialEq)]
pub enum ActionScriptItem {
    /// 类定义
    Class,
    /// 接口定义
    Interface,
    /// 函数定义
    Function,
    /// 变量声明
    Variable,
    /// 包声明
    Package,
    /// 导入语句
    Import,
}

impl Default for ActionScriptRoot {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}
