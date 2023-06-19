use serde::{Deserialize, Serialize};

/// PHP AST 根节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpRoot {
    pub items: Vec<PhpItem>,
}

/// PHP 顶级项目
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhpItem {
    OpenTag,
    CloseTag,
    Statement(PhpStatement),
    Function(PhpFunction),
    Class(PhpClass),
    Interface(PhpInterface),
    Trait(PhpTrait),
    Namespace(PhpNamespace),
    Use(PhpUse),
}

/// PHP 语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhpStatement {
    Expression(PhpExpression),
    If(PhpIf),
    While(PhpWhile),
    For(PhpFor),
    Foreach(PhpForeach),
    Switch(PhpSwitch),
    Try(PhpTry),
    Return(Option<PhpExpression>),
    Break(Option<PhpExpression>),
    Continue(Option<PhpExpression>),
    Echo(Vec<PhpExpression>),
    Print(PhpExpression),
    Global(Vec<String>),
    Static(Vec<PhpVariable>),
    Unset(Vec<PhpExpression>),
    Declare(Vec<PhpDeclareItem>),
    Block(Vec<PhpStatement>),
}

/// PHP 表达式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhpExpression {
    Literal(PhpLiteral),
    Variable(PhpVariable),
    Array(Vec<PhpArrayElement>),
    FunctionCall(PhpFunctionCall),
    MethodCall(PhpMethodCall),
    PropertyAccess(PhpPropertyAccess),
    ArrayAccess(PhpArrayAccess),
    Assignment(PhpAssignment),
    BinaryOp(PhpBinaryOp),
    UnaryOp(PhpUnaryOp),
    TernaryOp(PhpTernaryOp),
    Cast(PhpCast),
    New(PhpNew),
    Clone(Box<PhpExpression>),
    Instanceof(PhpInstanceof),
    Include(PhpInclude),
    Require(PhpRequire),
    Eval(Box<PhpExpression>),
    Exit(Option<Box<PhpExpression>>),
    Empty(Box<PhpExpression>),
    Isset(Vec<PhpExpression>),
    List(Vec<Option<PhpExpression>>),
    Yield(PhpYield),
}

/// PHP 字面量
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhpLiteral {
    String(String),
    Number(String),
    Boolean(bool),
    Null,
}

/// PHP 变量
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpVariable {
    pub name: String,
}

/// PHP 数组元素
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpArrayElement {
    pub key: Option<PhpExpression>,
    pub value: PhpExpression,
}

/// PHP 函数调用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpFunctionCall {
    pub name: Box<PhpExpression>,
    pub arguments: Vec<PhpExpression>,
}

/// PHP 方法调用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpMethodCall {
    pub object: Box<PhpExpression>,
    pub method: String,
    pub arguments: Vec<PhpExpression>,
}

/// PHP 属性访问
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpPropertyAccess {
    pub object: Box<PhpExpression>,
    pub property: String,
}

/// PHP 数组访问
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpArrayAccess {
    pub array: Box<PhpExpression>,
    pub index: Box<PhpExpression>,
}

/// PHP 赋值
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpAssignment {
    pub left: Box<PhpExpression>,
    pub operator: PhpAssignmentOp,
    pub right: Box<PhpExpression>,
}

/// PHP 赋值操作符
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhpAssignmentOp {
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    PowerAssign,
    ConcatAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    LeftShiftAssign,
    RightShiftAssign,
    NullCoalesceAssign,
}

/// PHP 二元操作
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpBinaryOp {
    pub left: Box<PhpExpression>,
    pub operator: PhpBinaryOperator,
    pub right: Box<PhpExpression>,
}

/// PHP 二元操作符
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhpBinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    Concat,
    Equal,
    NotEqual,
    Identical,
    NotIdentical,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Spaceship,
    LogicalAnd,
    LogicalOr,
    LogicalXor,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    NullCoalesce,
}

/// PHP 一元操作
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpUnaryOp {
    pub operator: PhpUnaryOperator,
    pub operand: Box<PhpExpression>,
}

/// PHP 一元操作符
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhpUnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    PreIncrement,
    PostIncrement,
    PreDecrement,
    PostDecrement,
    ErrorSuppression,
}

/// PHP 三元操作
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpTernaryOp {
    pub condition: Box<PhpExpression>,
    pub true_expr: Option<Box<PhpExpression>>,
    pub false_expr: Box<PhpExpression>,
}

/// PHP 类型转换
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpCast {
    pub cast_type: PhpCastType,
    pub expression: Box<PhpExpression>,
}

/// PHP 类型转换类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhpCastType {
    Int,
    Float,
    String,
    Bool,
    Array,
    Object,
    Unset,
}

/// PHP new 表达式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpNew {
    pub class: Box<PhpExpression>,
    pub arguments: Vec<PhpExpression>,
}

/// PHP instanceof 表达式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpInstanceof {
    pub expression: Box<PhpExpression>,
    pub class: Box<PhpExpression>,
}

/// PHP include 表达式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpInclude {
    pub once: bool,
    pub path: Box<PhpExpression>,
}

/// PHP require 表达式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpRequire {
    pub once: bool,
    pub path: Box<PhpExpression>,
}

/// PHP yield 表达式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpYield {
    pub key: Option<Box<PhpExpression>>,
    pub value: Option<Box<PhpExpression>>,
    pub from: bool,
}

/// PHP if 语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpIf {
    pub condition: Box<PhpExpression>,
    pub then_stmt: Box<PhpStatement>,
    pub elseif_stmts: Vec<PhpElseif>,
    pub else_stmt: Option<Box<PhpStatement>>,
}

/// PHP elseif 语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpElseif {
    pub condition: Box<PhpExpression>,
    pub statement: Box<PhpStatement>,
}

/// PHP while 语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpWhile {
    pub condition: Box<PhpExpression>,
    pub statement: Box<PhpStatement>,
}

/// PHP for 语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpFor {
    pub init: Vec<PhpExpression>,
    pub condition: Vec<PhpExpression>,
    pub update: Vec<PhpExpression>,
    pub statement: Box<PhpStatement>,
}

/// PHP foreach 语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpForeach {
    pub iterable: Box<PhpExpression>,
    pub key: Option<Box<PhpExpression>>,
    pub value: Box<PhpExpression>,
    pub statement: Box<PhpStatement>,
}

/// PHP switch 语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpSwitch {
    pub expression: Box<PhpExpression>,
    pub cases: Vec<PhpCase>,
}

/// PHP case 语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpCase {
    pub value: Option<Box<PhpExpression>>, // None for default case
    pub statements: Vec<PhpStatement>,
}

/// PHP try 语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpTry {
    pub statement: Box<PhpStatement>,
    pub catches: Vec<PhpCatch>,
    pub finally_stmt: Option<Box<PhpStatement>>,
}

/// PHP catch 语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpCatch {
    pub types: Vec<String>,
    pub variable: PhpVariable,
    pub statement: Box<PhpStatement>,
}

/// PHP declare 项目
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpDeclareItem {
    pub name: String,
    pub value: Box<PhpExpression>,
}

/// PHP 函数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpFunction {
    pub name: String,
    pub parameters: Vec<PhpParameter>,
    pub return_type: Option<PhpType>,
    pub body: Box<PhpStatement>,
    pub by_ref: bool,
}

/// PHP 参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpParameter {
    pub name: String,
    pub param_type: Option<PhpType>,
    pub default: Option<Box<PhpExpression>>,
    pub by_ref: bool,
    pub variadic: bool,
}

/// PHP 类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhpType {
    Named(String),
    Nullable(Box<PhpType>),
    Union(Vec<PhpType>),
}

/// PHP 类
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpClass {
    pub name: String,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub members: Vec<PhpClassMember>,
    pub modifiers: Vec<PhpModifier>,
}

/// PHP 类成员
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhpClassMember {
    Property(PhpProperty),
    Method(PhpMethod),
    Constant(PhpConstant),
    Use(PhpTraitUse),
}

/// PHP 属性
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpProperty {
    pub name: String,
    pub property_type: Option<PhpType>,
    pub default: Option<Box<PhpExpression>>,
    pub modifiers: Vec<PhpModifier>,
}

/// PHP 方法
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpMethod {
    pub name: String,
    pub parameters: Vec<PhpParameter>,
    pub return_type: Option<PhpType>,
    pub body: Option<Box<PhpStatement>>,
    pub modifiers: Vec<PhpModifier>,
    pub by_ref: bool,
}

/// PHP 常量
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpConstant {
    pub name: String,
    pub value: Box<PhpExpression>,
    pub modifiers: Vec<PhpModifier>,
}

/// PHP trait use
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpTraitUse {
    pub traits: Vec<String>,
    pub adaptations: Vec<PhpTraitAdaptation>,
}

/// PHP trait 适配
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhpTraitAdaptation {
    Precedence { method: String, trait_name: String, insteadof: Vec<String> },
    Alias { method: String, trait_name: Option<String>, alias: String, modifier: Option<PhpModifier> },
}

/// PHP 修饰符
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhpModifier {
    Public,
    Protected,
    Private,
    Static,
    Abstract,
    Final,
}

/// PHP 接口
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpInterface {
    pub name: String,
    pub extends: Vec<String>,
    pub members: Vec<PhpInterfaceMember>,
}

/// PHP 接口成员
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhpInterfaceMember {
    Method(PhpMethod),
    Constant(PhpConstant),
}

/// PHP trait
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpTrait {
    pub name: String,
    pub members: Vec<PhpClassMember>,
}

/// PHP 命名空间
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpNamespace {
    pub name: Option<String>,
    pub items: Vec<PhpItem>,
}

/// PHP use 语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpUse {
    pub uses: Vec<PhpUseItem>,
    pub use_type: PhpUseType,
}

/// PHP use 项目
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhpUseItem {
    pub name: String,
    pub alias: Option<String>,
}

/// PHP use 类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhpUseType {
    Normal,
    Function,
    Const,
}
