use core::range::Range;
use serde::{Deserialize, Serialize};

/// MSIL 抽象语法树的根节点
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MsilRoot {
    /// 指令、类、方法等项目列表
    pub items: Vec<Item>,
}

/// MSIL 中的顶级项目
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Item {
    /// 程序集定义
    Assembly(Assembly),
    /// 模块定义
    Module(String),
    /// 类定义
    Class(Class),
    /// 外部程序集引用
    AssemblyExtern(String),
}

/// 程序集定义
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Assembly {
    pub name: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 类定义
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub methods: Vec<Method>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 方法定义
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub instructions: Vec<Instruction>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 指令
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Instruction {
    pub opcode: String,
    pub operand: Option<String>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

pub struct MsilAssembly {
    pub name: String,
}
