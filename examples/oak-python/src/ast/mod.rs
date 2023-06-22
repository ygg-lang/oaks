use core::range::Range;
use serde::{Deserialize, Serialize};

/// Python 源文件的根节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PythonRoot {
    pub program: Program,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Python 程序
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// 语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    /// 函数定义
    FunctionDef { name: String, parameters: Vec<Parameter>, return_type: Option<Type>, body: Vec<Statement> },
    /// 类定义
    ClassDef { name: String, bases: Vec<Expression>, body: Vec<Statement> },
    /// 变量赋值
    Assignment { target: Expression, value: Expression },
    /// 复合赋值 (+=, -=, 等)
    AugmentedAssignment { target: Expression, operator: AugmentedOperator, value: Expression },
    /// 表达式语句
    Expression(Expression),
    /// 返回语句
    Return(Option<Expression>),
    /// 条件语句
    If { test: Expression, body: Vec<Statement>, orelse: Vec<Statement> },
    /// for 循环
    For { target: Expression, iter: Expression, body: Vec<Statement>, orelse: Vec<Statement> },
    /// while 循环
    While { test: Expression, body: Vec<Statement>, orelse: Vec<Statement> },
    /// break 语句
    Break,
    /// continue 语句
    Continue,
    /// pass 语句
    Pass,
    /// import 语句
    Import { names: Vec<ImportName> },
    /// from import 语句
    ImportFrom { module: Option<String>, names: Vec<ImportName> },
    /// try 语句
    Try { body: Vec<Statement>, handlers: Vec<ExceptHandler>, orelse: Vec<Statement>, finalbody: Vec<Statement> },
    /// raise 语句
    Raise { exc: Option<Expression>, cause: Option<Expression> },
    /// with 语句
    With { items: Vec<WithItem>, body: Vec<Statement> },
}

/// 表达式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// 字面量
    Literal(Literal),
    /// 标识符
    Name(String),
    /// 二元运算
    BinaryOp { left: Box<Expression>, operator: BinaryOperator, right: Box<Expression> },
    /// 一元运算
    UnaryOp { operator: UnaryOperator, operand: Box<Expression> },
    /// 布尔运算 (and, or)
    BoolOp { operator: BoolOperator, values: Vec<Expression> },
    /// 比较运算
    Compare { left: Box<Expression>, ops: Vec<CompareOperator>, comparators: Vec<Expression> },
    /// 函数调用
    Call { func: Box<Expression>, args: Vec<Expression>, keywords: Vec<Keyword> },
    /// 属性访问
    Attribute { value: Box<Expression>, attr: String },
    /// 下标访问
    Subscript { value: Box<Expression>, slice: Box<Expression> },
    /// 列表
    List { elts: Vec<Expression> },
    /// 元组
    Tuple { elts: Vec<Expression> },
    /// 字典
    Dict { keys: Vec<Option<Expression>>, values: Vec<Expression> },
    /// 集合
    Set { elts: Vec<Expression> },
    /// 列表推导式
    ListComp { elt: Box<Expression>, generators: Vec<Comprehension> },
    /// 字典推导式
    DictComp { key: Box<Expression>, value: Box<Expression>, generators: Vec<Comprehension> },
    /// 集合推导式
    SetComp { elt: Box<Expression>, generators: Vec<Comprehension> },
    /// lambda 表达式
    Lambda { args: Vec<Parameter>, body: Box<Expression> },
    /// 条件表达式 (三元运算符)
    IfExp { test: Box<Expression>, body: Box<Expression>, orelse: Box<Expression> },
}

/// 字面量
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    /// 整数
    Integer(i64),
    /// 浮点数
    Float(f64),
    /// 字符串
    String(String),
    /// 布尔值
    Boolean(bool),
    /// None
    None,
}

/// 二元运算符
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add,      // +
    Sub,      // -
    Mult,     // *
    Div,      // /
    FloorDiv, // //
    Mod,      // %
    Pow,      // **
    LShift,   // <<
    RShift,   // >>
    BitOr,    // |
    BitXor,   // ^
    BitAnd,   // &
}

/// 一元运算符
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Invert, // ~
    Not,    // not
    UAdd,   // +
    USub,   // -
}

/// 布尔运算符
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BoolOperator {
    And, // and
    Or,  // or
}

/// 比较运算符
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompareOperator {
    Eq,    // ==
    NotEq, // !=
    Lt,    // <
    LtE,   // <=
    Gt,    // >
    GtE,   // >=
    Is,    // is
    IsNot, // is not
    In,    // in
    NotIn, // not in
}

/// 复合赋值运算符
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AugmentedOperator {
    Add,      // +=
    Sub,      // -=
    Mult,     // *=
    Div,      // /=
    FloorDiv, // //=
    Mod,      // %=
    Pow,      // **=
    LShift,   // <<=
    RShift,   // >>=
    BitOr,    // |=
    BitXor,   // ^=
    BitAnd,   // &=
}

/// 函数参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub annotation: Option<Type>,
    pub default: Option<Expression>,
}

/// 类型注解
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    /// 基本类型名称
    Name(String),
    /// 泛型类型
    Generic { name: String, args: Vec<Type> },
    /// 联合类型
    Union(Vec<Type>),
    /// 可选类型
    Optional(Box<Type>),
}

/// 关键字参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keyword {
    pub arg: Option<String>,
    pub value: Expression,
}

/// 推导式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comprehension {
    pub target: Expression,
    pub iter: Expression,
    pub ifs: Vec<Expression>,
    pub is_async: bool,
}

/// import 名称
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportName {
    pub name: String,
    pub asname: Option<String>,
}

/// 异常处理器
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExceptHandler {
    pub type_: Option<Expression>,
    pub name: Option<String>,
    pub body: Vec<Statement>,
}

/// with 语句项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithItem {
    pub context_expr: Expression,
    pub optional_vars: Option<Expression>,
}

impl Program {
    /// 创建一个新的程序
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }

    /// 添加语句
    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Expression {
    /// 创建一个标识符表达式
    pub fn name(name: impl Into<String>) -> Self {
        Self::Name(name.into())
    }

    /// 创建一个字符串字面量表达式
    pub fn string(value: impl Into<String>) -> Self {
        Self::Literal(Literal::String(value.into()))
    }

    /// 创建一个整数字面量表达式
    pub fn integer(value: i64) -> Self {
        Self::Literal(Literal::Integer(value))
    }

    /// 创建一个浮点数字面量表达式
    pub fn float(value: f64) -> Self {
        Self::Literal(Literal::Float(value))
    }

    /// 创建一个布尔字面量表达式
    pub fn boolean(value: bool) -> Self {
        Self::Literal(Literal::Boolean(value))
    }

    /// 创建一个 None 字面量表达式
    pub fn none() -> Self {
        Self::Literal(Literal::None)
    }
}

impl Statement {
    /// 创建一个函数定义语句
    pub fn function_def(name: impl Into<String>, parameters: Vec<Parameter>, return_type: Option<Type>, body: Vec<Statement>) -> Self {
        Self::FunctionDef { name: name.into(), parameters, return_type, body }
    }

    /// 创建一个赋值语句
    pub fn assignment(target: Expression, value: Expression) -> Self {
        Self::Assignment { target, value }
    }

    /// 创建一个表达式语句
    pub fn expression(expr: Expression) -> Self {
        Self::Expression(expr)
    }

    /// 创建一个返回语句
    pub fn return_stmt(value: Option<Expression>) -> Self {
        Self::Return(value)
    }
}
