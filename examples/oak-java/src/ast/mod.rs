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
    /// 成员
    pub members: Vec<Member>,
    /// 源码位置
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 类成员
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Member {
    /// 方法声明
    Method(MethodDeclaration),
    /// 字段声明 (暂不支持)
    Field(FieldDeclaration),
}

/// 方法声明
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MethodDeclaration {
    /// 方法名
    pub name: String,
    /// 返回类型
    pub return_type: String,
    /// 参数列表
    pub parameters: Vec<Parameter>,
    /// 方法体
    pub body: Vec<Statement>,
    /// 是否为静态
    pub is_static: bool,
    /// 源码位置
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    /// 参数名
    pub name: String,
    /// 参数类型
    pub r#type: String,
}

/// 字段声明
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDeclaration {
    pub name: String,
    pub r#type: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    /// 表达式语句
    Expression(Expression),
    /// 返回语句
    Return(Option<Expression>),
    /// 块语句
    Block(Vec<Statement>),
}

/// 表达式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// 字面量
    Literal(Literal),
    /// 变量引用
    Identifier(String),
    /// 方法调用
    MethodCall(MethodCall),
    /// 字段访问
    FieldAccess(FieldAccess),
}

/// 字面量
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Integer(i64),
    String(String),
    Boolean(bool),
}

/// 字段访问
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldAccess {
    /// 访问目标
    pub target: Box<Expression>,
    /// 字段名
    pub name: String,
}

/// 方法调用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MethodCall {
    /// 调用目标 (可选，如 System.out)
    pub target: Option<Box<Expression>>,
    /// 方法名
    pub name: String,
    /// 参数
    pub arguments: Vec<Expression>,
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
