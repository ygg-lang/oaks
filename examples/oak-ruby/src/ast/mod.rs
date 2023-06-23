#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Ruby AST 根节点
pub type RubyAst = RubyRoot;

/// 程序节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RubyRoot {
    pub statements: Vec<StatementNode>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Ruby 语句节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StatementNode {
    /// 表达式语句
    Expression(ExpressionNode),
    /// 方法定义
    MethodDef {
        name: String,
        params: Vec<String>,
        body: Vec<StatementNode>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 类定义
    ClassDef {
        name: String,
        superclass: Option<String>,
        body: Vec<StatementNode>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 赋值语句
    Assignment {
        target: String,
        value: ExpressionNode,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 条件语句
    If {
        condition: ExpressionNode,
        then_body: Vec<StatementNode>,
        else_body: Option<Vec<StatementNode>>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 循环语句
    While {
        condition: ExpressionNode,
        body: Vec<StatementNode>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 返回语句
    Return {
        value: Option<ExpressionNode>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

/// Ruby 表达式节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExpressionNode {
    /// 标识符
    Identifier {
        name: String,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 字面量
    Literal(LiteralNode),
    /// 方法调用
    MethodCall {
        receiver: Option<Box<ExpressionNode>>,
        method: String,
        args: Vec<ExpressionNode>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 二元操作
    BinaryOp {
        left: Box<ExpressionNode>,
        operator: String,
        right: Box<ExpressionNode>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 一元操作
    UnaryOp {
        operator: String,
        operand: Box<ExpressionNode>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 数组
    Array {
        elements: Vec<ExpressionNode>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 哈希
    Hash {
        pairs: Vec<(ExpressionNode, ExpressionNode)>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

/// 字面量节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LiteralNode {
    /// 整数
    Integer {
        value: i64,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 浮点数
    Float {
        value: f64,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 字符串
    String {
        value: String,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 符号
    Symbol {
        value: String,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// 布尔值
    Boolean {
        value: bool,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// nil
    Nil {
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

/// Ruby AST 访问者 trait
pub trait RubyAstVisitor {
    fn visit_program(&mut self, node: &RubyRoot);
    fn visit_statement(&mut self, stmt: &StatementNode);
    fn visit_expression(&mut self, expr: &ExpressionNode);
    fn visit_literal(&mut self, literal: &LiteralNode);
}
