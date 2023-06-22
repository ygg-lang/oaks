use serde::{Deserialize, Serialize};

/// Perl 程序的根节点
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerlRoot {
    pub items: Vec<PerlItem>,
}

/// Perl 程序中的顶级项目
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerlItem {
    /// 包声明
    Package(PerlPackage),
    /// 使用声明
    Use(PerlUse),
    /// 子程序定义
    Subroutine(PerlSubroutine),
    /// 变量声明
    Variable(PerlVariable),
    /// 表达式语句
    Expression(PerlExpression),
    /// 注释
    Comment(String),
}

/// 包声明
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerlPackage {
    pub name: String,
}

/// 使用声明
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerlUse {
    pub module: String,
    pub imports: Option<Vec<String>>,
}

/// 子程序定义
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerlSubroutine {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<PerlStatement>,
}

/// 变量声明
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerlVariable {
    pub scope: PerlScope,
    pub name: String,
    pub value: Option<PerlExpression>,
}

/// 变量作用域
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerlScope {
    My,
    Our,
    Local,
}

/// Perl 语句
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerlStatement {
    /// 表达式语句
    Expression(PerlExpression),
    /// 条件语句
    If(PerlIf),
    /// 循环语句
    Loop(PerlLoop),
    /// 返回语句
    Return(Option<PerlExpression>),
    /// 控制流语句
    Control(PerlControl),
}

/// 条件语句
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerlIf {
    pub condition: PerlExpression,
    pub then_block: Vec<PerlStatement>,
    pub elsif_blocks: Vec<(PerlExpression, Vec<PerlStatement>)>,
    pub else_block: Option<Vec<PerlStatement>>,
}

/// 循环语句
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerlLoop {
    While { condition: PerlExpression, body: Vec<PerlStatement> },
    Until { condition: PerlExpression, body: Vec<PerlStatement> },
    For { init: Option<PerlExpression>, condition: Option<PerlExpression>, update: Option<PerlExpression>, body: Vec<PerlStatement> },
    Foreach { variable: String, iterable: PerlExpression, body: Vec<PerlStatement> },
}

/// 控制流语句
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerlControl {
    Last,
    Next,
    Redo,
}

/// Perl 表达式
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerlExpression {
    /// 字面量
    Literal(PerlLiteral),
    /// 变量引用
    Variable(PerlVariableRef),
    /// 二元操作
    Binary { left: Box<PerlExpression>, operator: PerlBinaryOp, right: Box<PerlExpression> },
    /// 一元操作
    Unary { operator: PerlUnaryOp, operand: Box<PerlExpression> },
    /// 函数调用
    Call { function: String, arguments: Vec<PerlExpression> },
    /// 数组访问
    ArrayAccess { array: Box<PerlExpression>, index: Box<PerlExpression> },
    /// 哈希访问
    HashAccess { hash: Box<PerlExpression>, key: Box<PerlExpression> },
}

/// 字面量
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerlLiteral {
    String(String),
    Number(String),
    Array(Vec<PerlExpression>),
    Hash(Vec<(PerlExpression, PerlExpression)>),
}

/// 变量引用
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerlVariableRef {
    pub sigil: PerlSigil,
    pub name: String,
}

/// 变量符号
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerlSigil {
    Scalar, // $
    Array,  // @
    Hash,   // %
    Code,   // &
    Glob,   // *
}

/// 二元操作符
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerlBinaryOp {
    // 算术操作符
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,

    // 字符串操作符
    Concat,
    Repeat,

    // 比较操作符
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    Spaceship,

    // 逻辑操作符
    LogicalAnd,
    LogicalOr,

    // 位操作符
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,

    // 赋值操作符
    Assign,

    // 模式匹配
    Match,
    NotMatch,
}

/// 一元操作符
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerlUnaryOp {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    Increment,
    Decrement,
    Reference,
    Dereference,
}
