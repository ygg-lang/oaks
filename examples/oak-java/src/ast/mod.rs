#![doc = include_str!("readme.md")]
//! Java AST definitions

use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Java 程序的根节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JavaRoot {
    /// 编译单元中的项目
    pub items: Vec<Item>,
}

/// Java 程序中的顶级项目
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Item {
    /// 类声明
    Class(ClassDeclaration),
    /// 接口声明
    Interface(InterfaceDeclaration),
    /// 结构体声明
    Struct(StructDeclaration),
    /// 枚举声明
    Enum(EnumDeclaration),
    /// 记录声明
    Record(RecordDeclaration),
    /// 包声明
    Package(PackageDeclaration),
    /// 导入声明
    Import(ImportDeclaration),
}

/// 类声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClassDeclaration {
    /// 类名
    pub name: String,
    /// 修饰符
    pub modifiers: Vec<String>,
    /// 父类
    pub extends: Option<String>,
    /// 实现的接口
    pub implements: Vec<String>,
    /// 成员
    pub members: Vec<Member>,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 类成员
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Member {
    /// 方法声明
    Method(MethodDeclaration),
    /// 字段声明
    Field(FieldDeclaration),
    /// 构造函数
    Constructor(ConstructorDeclaration),
}

/// 构造函数声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConstructorDeclaration {
    /// 修饰符
    pub modifiers: Vec<String>,
    /// 名称 (应与类名一致)
    pub name: String,
    /// 参数列表
    pub parameters: Vec<Parameter>,
    /// 方法体
    pub body: Vec<Statement>,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 方法声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MethodDeclaration {
    /// 方法名
    pub name: String,
    /// 修饰符
    pub modifiers: Vec<String>,
    /// 返回类型
    pub return_type: String,
    /// 参数列表
    pub parameters: Vec<Parameter>,
    /// 方法体
    pub body: Vec<Statement>,
    /// 抛出的异常
    pub throws: Vec<String>,
    /// 是否为静态
    pub is_static: bool,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 参数
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Parameter {
    /// 参数名
    pub name: String,
    /// 参数类型
    pub r#type: String,
}

/// 字段声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FieldDeclaration {
    pub name: String,
    pub r#type: String,
    pub modifiers: Vec<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Statement {
    /// 表达式语句
    Expression(Expression),
    /// 返回语句
    Return(Option<Expression>),
    /// 块语句
    Block(Vec<Statement>),
    /// Try 语句
    Try(TryStatement),
    /// Throw 语句
    Throw(Expression),
    /// 条件判断
    If { condition: Expression, then_branch: Box<Statement>, else_branch: Option<Box<Statement>> },
    /// While 循环
    While { condition: Expression, body: Box<Statement> },
    /// Do-While 循环
    DoWhile { condition: Expression, body: Box<Statement> },
    /// For 循环
    For { init: Option<Box<Statement>>, condition: Option<Expression>, update: Option<Expression>, body: Box<Statement> },
    /// For-Each 循环
    ForEach { item_type: String, item_name: String, iterable: Expression, body: Box<Statement> },
    /// Switch 语句
    Switch { selector: Expression, cases: Vec<SwitchCase>, default: Option<Vec<Statement>> },
    /// Break 语句
    Break,
    /// Continue 语句
    Continue,
    /// 变量声明语句
    LocalVariable { r#type: String, name: String, initializer: Option<Expression> },
}

/// Switch 分支
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SwitchCase {
    pub label: Expression,
    pub body: Vec<Statement>,
}

/// Try 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TryStatement {
    pub block: Vec<Statement>,
    pub catches: Vec<CatchClause>,
    pub finally: Option<Vec<Statement>>,
}

/// Catch 子句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CatchClause {
    pub parameter: Parameter,
    pub block: Vec<Statement>,
}

/// 表达式
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Expression {
    /// 字面量
    Literal(Literal),
    /// 变量引用
    Identifier(String),
    /// 方法调用
    MethodCall(MethodCall),
    /// 字段访问
    FieldAccess(FieldAccess),
    /// 数组访问
    ArrayAccess(ArrayAccess),
    /// 数组创建
    ArrayCreation(ArrayCreation),
    /// New 表达式
    New(NewExpression),
    /// This 表达式
    This,
    /// Super 表达式
    Super,
    /// 二元运算
    Binary { left: Box<Expression>, op: String, right: Box<Expression> },
    /// 一元运算
    Unary { op: String, expression: Box<Expression> },
    /// 赋值运算
    Assignment { left: Box<Expression>, op: String, right: Box<Expression> },
    /// 自增自减运算
    Update { expression: Box<Expression>, op: String, is_prefix: bool },
    /// 三元运算
    Ternary { condition: Box<Expression>, then_branch: Box<Expression>, else_branch: Box<Expression> },
    /// 类型转换
    Cast { target_type: String, expression: Box<Expression> },
}

/// New 表达式
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NewExpression {
    pub r#type: String,
    pub arguments: Vec<Expression>,
}

/// 字面量
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

/// 字段访问
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FieldAccess {
    /// 访问目标
    pub target: Box<Expression>,
    /// 字段名
    pub name: String,
}

/// 方法调用
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MethodCall {
    /// 调用目标 (可选，如 System.out)
    pub target: Option<Box<Expression>>,
    /// 方法名
    pub name: String,
    /// 参数
    pub arguments: Vec<Expression>,
}

/// 数组访问
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ArrayAccess {
    pub target: Box<Expression>,
    pub index: Box<Expression>,
}

/// 数组创建
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ArrayCreation {
    pub element_type: String,
    pub dimensions: Vec<Expression>,
}

/// 接口声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InterfaceDeclaration {
    /// 接口名
    pub name: String,
    /// 修饰符
    pub modifiers: Vec<String>,
    /// 继承的接口
    pub extends: Vec<String>,
    /// 成员
    pub members: Vec<Member>,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 结构体声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StructDeclaration {
    /// 结构体名
    pub name: String,
    /// 修饰符
    pub modifiers: Vec<String>,
    /// 实现的接口
    pub implements: Vec<String>,
    /// 成员
    pub members: Vec<Member>,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 枚举声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnumDeclaration {
    /// 枚举名
    pub name: String,
    /// 修饰符
    pub modifiers: Vec<String>,
    /// 枚举项
    pub variants: Vec<String>,
    /// 成员
    pub members: Vec<Member>,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 记录声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RecordDeclaration {
    /// 记录名
    pub name: String,
    /// 修饰符
    pub modifiers: Vec<String>,
    /// 参数列表 (C# record 有主构造函数)
    pub parameters: Vec<Parameter>,
    /// 实现的接口
    pub implements: Vec<String>,
    /// 成员
    pub members: Vec<Member>,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 包声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PackageDeclaration {
    /// 包名
    pub name: String,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 导入声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImportDeclaration {
    /// 导入路径
    pub path: String,
    /// 是否为静态导入
    pub is_static: bool,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
