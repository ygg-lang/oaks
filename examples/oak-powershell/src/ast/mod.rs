#![doc = include_str!("readme.md")]
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// PowerShell AST 根节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellRoot {
    pub items: Vec<PowerShellItem>,
}

/// PowerShell 顶级项目
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PowerShellItem {
    Statement(PowerShellStatement),
    Function(PowerShellFunction),
    Class(PowerShellClass),
    Workflow(PowerShellWorkflow),
}

/// PowerShell 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PowerShellStatement {
    Expression(Box<PowerShellExpression>),
    Assignment(PowerShellAssignment),
    If(PowerShellIf),
    While(PowerShellWhile),
    For(PowerShellFor),
    ForEach(PowerShellForEach),
    Switch(PowerShellSwitch),
    Try(PowerShellTry),
    Return(PowerShellReturn),
    Break(PowerShellBreak),
    Continue(PowerShellContinue),
    Exit(PowerShellExit),
    Throw(PowerShellThrow),
    Block(PowerShellBlock),
}

/// PowerShell 表达式
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PowerShellExpression {
    Literal(PowerShellLiteral),
    Variable(PowerShellVariable),
    Command(PowerShellCommand),
    Pipeline(PowerShellPipeline),
    Binary(PowerShellBinaryOp),
    Unary(PowerShellUnaryOp),
    Member(PowerShellMemberAccess),
    Index(PowerShellIndexAccess),
    Subexpression(Box<PowerShellExpression>),
    Array(PowerShellArray),
    Hashtable(PowerShellHashtable),
    ScriptBlock(PowerShellScriptBlock),
}

/// PowerShell 字面量
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PowerShellLiteral {
    String(String),
    Number(String),
    Boolean(bool),
    Null,
}

/// PowerShell 变量
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellVariable {
    pub name: String,
    pub scope: Option<String>,
}

/// PowerShell 命令
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellCommand {
    pub name: String,
    pub arguments: Vec<PowerShellArgument>,
}

/// PowerShell 参数
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PowerShellArgument {
    Positional(Box<PowerShellExpression>),
    Named(String, Box<PowerShellExpression>),
    Switch(String),
}

/// PowerShell 管道
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellPipeline {
    pub commands: Vec<PowerShellCommand>,
}

/// PowerShell 二元操作
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellBinaryOp {
    pub left: Box<PowerShellExpression>,
    pub operator: PowerShellBinaryOperator,
    pub right: Box<PowerShellExpression>,
}

/// PowerShell 二元操作符
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PowerShellBinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Like,
    NotLike,
    Match,
    NotMatch,
    Contains,
    NotContains,
    In,
    NotIn,
    And,
    Or,
    Xor,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
}

/// PowerShell 一元操作
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellUnaryOp {
    pub operator: PowerShellUnaryOperator,
    pub operand: Box<PowerShellExpression>,
}

/// PowerShell 一元操作符
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PowerShellUnaryOperator {
    Plus,
    Minus,
    Not,
    BitwiseNot,
    Increment,
    Decrement,
}

/// PowerShell 成员访问
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellMemberAccess {
    pub object: Box<PowerShellExpression>,
    pub member: String,
}

/// PowerShell 索引访问
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellIndexAccess {
    pub object: Box<PowerShellExpression>,
    pub index: Box<PowerShellExpression>,
}

/// PowerShell 数组
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellArray {
    pub elements: Vec<PowerShellExpression>,
}

/// PowerShell 哈希表
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellHashtable {
    pub entries: Vec<PowerShellHashtableEntry>,
}

/// PowerShell 哈希表项
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellHashtableEntry {
    pub key: Box<PowerShellExpression>,
    pub value: Box<PowerShellExpression>,
}

/// PowerShell 脚本块
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellScriptBlock {
    pub statements: Vec<PowerShellStatement>,
}

/// PowerShell 赋值
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellAssignment {
    pub target: Box<PowerShellExpression>,
    pub operator: PowerShellAssignmentOperator,
    pub value: Box<PowerShellExpression>,
}

/// PowerShell 赋值操作符
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PowerShellAssignmentOperator {
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
}

/// PowerShell If 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellIf {
    pub condition: Box<PowerShellExpression>,
    pub then_block: PowerShellScriptBlock,
    pub elseif_blocks: Vec<PowerShellElseIf>,
    pub else_block: Option<PowerShellScriptBlock>,
}

/// PowerShell ElseIf 子句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellElseIf {
    pub condition: Box<PowerShellExpression>,
    pub block: PowerShellScriptBlock,
}

/// PowerShell While 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellWhile {
    pub condition: Box<PowerShellExpression>,
    pub block: PowerShellScriptBlock,
}

/// PowerShell For 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellFor {
    pub init: Option<Box<PowerShellExpression>>,
    pub condition: Option<Box<PowerShellExpression>>,
    pub update: Option<Box<PowerShellExpression>>,
    pub block: PowerShellScriptBlock,
}

/// PowerShell ForEach 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellForEach {
    pub variable: PowerShellVariable,
    pub collection: Box<PowerShellExpression>,
    pub block: PowerShellScriptBlock,
}

/// PowerShell Switch 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellSwitch {
    pub expression: Box<PowerShellExpression>,
    pub cases: Vec<PowerShellSwitchCase>,
    pub default: Option<PowerShellScriptBlock>,
}

/// PowerShell Switch Case 子句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellSwitchCase {
    pub pattern: Box<PowerShellExpression>,
    pub block: PowerShellScriptBlock,
}

/// PowerShell Try 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellTry {
    pub block: PowerShellScriptBlock,
    pub catch_blocks: Vec<PowerShellCatch>,
    pub finally_block: Option<PowerShellScriptBlock>,
}

/// PowerShell Catch 子句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellCatch {
    pub exception_type: Option<String>,
    pub block: PowerShellScriptBlock,
}

/// PowerShell Return 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellReturn {
    pub value: Option<Box<PowerShellExpression>>,
}

/// PowerShell Break 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellBreak {
    pub label: Option<String>,
}

/// PowerShell Continue 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellContinue {
    pub label: Option<String>,
}

/// PowerShell Exit 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellExit {
    pub code: Option<Box<PowerShellExpression>>,
}

/// PowerShell Throw 语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellThrow {
    pub exception: Option<Box<PowerShellExpression>>,
}

/// PowerShell 块
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellBlock {
    pub statements: Vec<PowerShellStatement>,
}

/// PowerShell 函数
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellFunction {
    pub name: String,
    pub body: PowerShellScriptBlock,
    pub attributes: Vec<PowerShellAttribute>,
}

/// PowerShell 参数块
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellParamBlock {
    pub parameters: Vec<PowerShellParameter>,
}

/// PowerShell 参数
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellParameter {
    pub name: String,
    pub param_type: Option<String>,
    pub default_value: Option<Box<PowerShellExpression>>,
    pub attributes: Vec<PowerShellAttribute>,
}

/// PowerShell 属性
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellAttribute {
    pub name: String,
    pub arguments: Vec<PowerShellExpression>,
}

/// PowerShell 类
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellClass {
    pub name: String,
    pub base_class: Option<String>,
    pub members: Vec<PowerShellClassMember>,
}

/// PowerShell 类成员
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PowerShellClassMember {
    Property(PowerShellProperty),
    Method(PowerShellMethod),
    Constructor(PowerShellConstructor),
}

/// PowerShell 属性
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellProperty {
    pub name: String,
    pub property_type: Option<String>,
    pub default_value: Option<Box<PowerShellExpression>>,
    pub attributes: Vec<PowerShellAttribute>,
}

/// PowerShell 方法
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellMethod {
    pub name: String,
    pub return_type: Option<String>,
    pub parameters: Vec<PowerShellParameter>,
    pub body: PowerShellScriptBlock,
    pub attributes: Vec<PowerShellAttribute>,
}

/// PowerShell 构造函数
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellConstructor {
    pub parameters: Vec<PowerShellParameter>,
    pub body: PowerShellScriptBlock,
}

/// PowerShell 工作流
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerShellWorkflow {
    pub name: String,
    pub parameters: Vec<PowerShellParameter>,
    pub body: PowerShellScriptBlock,
}
