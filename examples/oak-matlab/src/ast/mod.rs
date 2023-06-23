#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// MATLAB 抽象语法树的根节点
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MatlabRoot {
    /// 脚本或函数中的项目列表
    pub items: Vec<Item>,
}

/// MATLAB 中的顶级项目
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Item {
    /// 函数定义
    Function(Function),
    /// 类定义
    Class(Class),
    /// 语句
    Statement(Statement),
}

/// 函数定义
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Function {
    /// 函数名
    pub name: String,
    /// 输入参数
    pub inputs: Vec<String>,
    /// 输出参数
    pub outputs: Vec<String>,
    /// 函数体
    pub body: Vec<Statement>,
    /// 源代码范围
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 类定义
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Class {
    /// 类名
    pub name: String,
    /// 基类
    pub superclasses: Vec<String>,
    /// 属性块
    pub properties: Vec<Property>,
    /// 方法块
    pub methods: Vec<Function>,
    /// 源代码范围
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 属性
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Property {
    /// 属性名
    pub name: String,
    /// 默认值
    pub default_value: Option<String>,
    /// 源代码范围
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 语句
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Statement {
    /// 赋值语句
    Assignment {
        target: String,
        value: String,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 表达式语句
    Expression {
        value: String,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// If 语句
    If {
        condition: String,
        body: Vec<Statement>,
        else_ifs: Vec<(String, Vec<Statement>)>,
        else_body: Option<Vec<Statement>>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// For 循环
    For {
        variable: String,
        range: String,
        body: Vec<Statement>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// While 循环
    While {
        condition: String,
        body: Vec<Statement>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

pub struct MatlabScript {
    pub items: Vec<Item>,
}

impl MatlabScript {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}
