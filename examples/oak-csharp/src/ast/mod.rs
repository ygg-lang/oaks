#![doc = include_str!("readme.md")]
//! C# AST definitions

use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// C# 程序的根节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CSharpRoot {
    /// 编译单元中的项目
    pub items: Vec<Item>,
}

/// C# 程序中的顶级项目
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Item {
    /// 命名空间声明
    Namespace(NamespaceDeclaration),
    /// Using 指令
    Using(UsingDirective),
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
    /// 委托声明
    Delegate(DelegateDeclaration),
}

/// 命名空间声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NamespaceDeclaration {
    /// 命名空间名
    pub name: String,
    /// 特性
    pub attributes: Vec<Attribute>,
    /// 成员
    pub items: Vec<Item>,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Using 指令
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UsingDirective {
    /// 导入路径
    pub path: String,
    /// 是否为静态导入
    pub is_static: bool,
    /// 别名
    pub alias: Option<String>,
    /// 全局
    pub is_global: bool,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 类声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClassDeclaration {
    /// 类名
    pub name: String,
    /// 特性
    pub attributes: Vec<Attribute>,
    /// 修饰符
    pub modifiers: Vec<String>,
    /// 基类和接口
    pub base_types: Vec<String>,
    /// 泛型参数
    pub type_parameters: Vec<TypeParameter>,
    /// 泛型约束
    pub constraints: Vec<TypeParameterConstraint>,
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
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<String>,
    pub members: Vec<Member>,
    pub type_parameters: Vec<TypeParameter>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 接口声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InterfaceDeclaration {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<String>,
    pub members: Vec<Member>,
    pub type_parameters: Vec<TypeParameter>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 枚举声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnumDeclaration {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<String>,
    pub members: Vec<EnumMember>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 枚举成员
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnumMember {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub value: Option<Expression>,
}

/// 记录声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RecordDeclaration {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<String>,
    pub members: Vec<Member>,
    pub type_parameters: Vec<TypeParameter>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 委托声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DelegateDeclaration {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<String>,
    pub return_type: String,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 成员
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Member {
    /// 方法声明
    Method(MethodDeclaration),
    /// 字段声明
    Field(FieldDeclaration),
    /// 属性声明
    Property(PropertyDeclaration),
    /// 索引器声明
    Indexer(IndexerDeclaration),
    /// 构造函数
    Constructor(MethodDeclaration),
    /// 事件声明
    Event(EventDeclaration),
}

/// 方法声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MethodDeclaration {
    /// 方法名
    pub name: String,
    /// 特性
    pub attributes: Vec<Attribute>,
    /// 修饰符
    pub modifiers: Vec<String>,
    /// 返回类型
    pub return_type: String,
    /// 泛型参数
    pub type_parameters: Vec<TypeParameter>,
    /// 参数列表
    pub parameters: Vec<Parameter>,
    /// 方法体
    pub body: Option<Vec<Statement>>,
    /// 是否为异步
    pub is_async: bool,
    /// 源码位置
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 属性声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PropertyDeclaration {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub r#type: String,
    pub modifiers: Vec<String>,
    pub get_accessor: Option<Accessor>,
    pub set_accessor: Option<Accessor>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 访问器 (get/set)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Accessor {
    pub attributes: Vec<Attribute>,
    pub body: Option<Vec<Statement>>,
    pub modifiers: Vec<String>,
}

/// 索引器声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IndexerDeclaration {
    pub attributes: Vec<Attribute>,
    pub r#type: String,
    pub parameters: Vec<Parameter>,
    pub get_accessor: Option<Accessor>,
    pub set_accessor: Option<Accessor>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 事件声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventDeclaration {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub r#type: String,
    pub modifiers: Vec<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 参数
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Parameter {
    /// 特性
    pub attributes: Vec<Attribute>,
    /// 参数名
    pub name: String,
    /// 参数类型
    pub r#type: String,
    /// 修饰符 (ref, out, params)
    pub modifiers: Vec<String>,
    /// 默认值
    pub default_value: Option<Expression>,
}

/// 字段声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FieldDeclaration {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub r#type: String,
    pub modifiers: Vec<String>,
    pub initializer: Option<Expression>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 特性
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Attribute {
    pub name: String,
    pub arguments: Vec<Expression>,
}

/// 泛型参数
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeParameter {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub variance: Option<String>, // in, out
}

/// 泛型约束
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeParameterConstraint {
    pub parameter_name: String,
    pub constraints: Vec<String>,
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
    /// 条件判断
    If { condition: Expression, then_branch: Box<Statement>, else_branch: Option<Box<Statement>> },
    /// While 循环
    While { condition: Expression, body: Box<Statement> },
    /// For 循环
    For { init: Option<Box<Statement>>, condition: Option<Expression>, update: Option<Expression>, body: Box<Statement> },
    /// Foreach 循环
    Foreach { item_type: String, item_name: String, iterable: Expression, body: Box<Statement> },
    /// 变量声明语句
    LocalVariable { r#type: String, name: String, initializer: Option<Expression> },
    /// Break
    Break,
    /// Continue
    Continue,
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
    /// 字段/属性访问
    MemberAccess(MemberAccess),
    /// 索引访问
    ElementAccess(ElementAccess),
    /// New 表达式
    New(NewExpression),
    /// This 表达式
    This,
    /// Base 表达式
    Base,
    /// 二元运算
    Binary { left: Box<Expression>, op: String, right: Box<Expression> },
    /// 一元运算
    Unary { op: String, expression: Box<Expression> },
    /// 赋值运算
    Assignment { left: Box<Expression>, op: String, right: Box<Expression> },
    /// Await 表达式
    Await(Box<Expression>),
    /// LINQ 查询表达式
    Query(Box<QueryExpression>),
}

/// LINQ 查询表达式
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryExpression {
    pub from_clause: FromClause,
    pub body: QueryBody,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FromClause {
    pub identifier: String,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryBody {
    pub clauses: Vec<QueryClause>,
    pub select_or_group: SelectOrGroupClause,
    pub continuation: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum QueryClause {
    From(FromClause),
    Let(LetClause),
    Where(Expression),
    Join(JoinClause),
    OrderBy(Vec<Ordering>),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum QueryClauseExt {
    GroupBy(Expression),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LetClause {
    pub identifier: String,
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JoinClause {
    pub identifier: String,
    pub in_expression: Expression,
    pub on_expression: Expression,
    pub equals_expression: Expression,
    pub into_identifier: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ordering {
    pub expression: Expression,
    pub ascending: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SelectOrGroupClause {
    Select(Expression),
    Group { expression: Expression, by_expression: Expression },
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
    String(String),
    Boolean(bool),
    Null,
}

/// 成员访问
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemberAccess {
    pub target: Box<Expression>,
    pub name: String,
}

/// 方法调用
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MethodCall {
    pub target: Option<Box<Expression>>,
    pub name: String,
    pub arguments: Vec<Expression>,
}

/// 元素访问 (索引器)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElementAccess {
    pub target: Box<Expression>,
    pub arguments: Vec<Expression>,
}
