//! MSIL (Microsoft Intermediate Language) 抽象语法(AST) 模块
//!
//! 这个模块定义MSIL 汇编语言的抽象语法树结构，用于表示解析后MSIL 代码
//! AST 节点对应MSIL 汇编语言中的各种构造，如程序集、模块、类、方法等

use alloc::{string::String, vec::Vec};

/// MSIL 程序的根节点
///
/// 表示一个完整的 MSIL 程序，包含程序中的所有类
///
/// # 示例
///
/// ```rust
/// # use oak_msil::ast::MsilRoot;
///
/// let root = MsilRoot { classes: vec![] };
/// ```
#[derive(Clone, Debug)]
pub struct MsilRoot {
    /// 程序中的所有类
    pub classes: Vec<MsilClass>,
}

/// MSIL 类声
///
/// 表示 MSIL 程序中的类定
///
/// # 示例
///
/// ```rust
/// # use oak_msil::ast::MsilClass;
///
/// let class = MsilClass { name: "MyClass".to_string(), methods: vec![] };
/// ```
#[derive(Clone, Debug)]
pub struct MsilClass {
    /// 类名
    pub name: String,
    /// 类中的方
    pub methods: Vec<MsilMethod>,
}

/// MSIL 方法声明
///
/// 表示类中的方法定
///
/// # 示例
///
/// ```rust
/// # use oak_msil::ast::MsilMethod;
///
/// let method = MsilMethod { name: "MyMethod".to_string() };
/// ```
#[derive(Clone, Debug)]
pub struct MsilMethod {
    /// 方法
    pub name: String,
}
