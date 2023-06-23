#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Swift 源文件的根节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SwiftRoot {
    pub program: Program,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Swift 程序
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Statement {
    /// 函数定义
    FunctionDef { name: String, parameters: Vec<Parameter>, return_type: Option<Type>, body: Vec<Statement> },
    /// 变量声明
    VariableDecl { is_mutable: bool, name: String, type_annotation: Option<Type>, value: Option<Expression> },
    /// 表达式语句
    Expression(Expression),
    /// 返回语句
    Return(Option<Expression>),
    /// 条件语句
    If { test: Expression, body: Vec<Statement>, orelse: Option<Vec<Statement>> },
    /// While 循环
    While { test: Expression, body: Vec<Statement> },
    /// For 循环
    For { variable: String, iterable: Expression, body: Vec<Statement> },
    /// 代码块
    Block(Vec<Statement>),
}

/// 表达式
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Expression {
    /// 二元运算
    Binary { left: Box<Expression>, operator: String, right: Box<Expression> },
    /// 一元运算
    Unary { operator: String, operand: Box<Expression> },
    /// 函数调用
    Call { callee: Box<Expression>, arguments: Vec<Expression> },
    /// 成员访问
    Member { object: Box<Expression>, member: String },
    /// 标识符
    Identifier(String),
    /// 字面量
    Literal(Literal),
}

/// 字面量
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Literal {
    Number(String),
    String(String),
    Boolean(bool),
    Nil,
}

/// 参数
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Type,
}

/// 类型
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Type {
    pub name: String,
}
