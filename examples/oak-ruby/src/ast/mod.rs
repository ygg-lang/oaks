use alloc::{boxed::Box, string::String, vec::Vec};

/// Ruby AST 根节点
pub type RubyAst = ProgramNode;

/// 程序节点
#[derive(Debug, Clone, PartialEq)]
pub struct ProgramNode {
    pub statements: Vec<StatementNode>,
}

/// Ruby 语句节点
#[derive(Debug, Clone, PartialEq)]
pub enum StatementNode {
    /// 表达式语句
    Expression(ExpressionNode),
    /// 方法定义
    MethodDef { name: String, params: Vec<String>, body: Vec<StatementNode> },
    /// 类定义
    ClassDef { name: String, superclass: Option<String>, body: Vec<StatementNode> },
    /// 赋值语句
    Assignment { target: String, value: ExpressionNode },
    /// 条件语句
    If { condition: ExpressionNode, then_body: Vec<StatementNode>, else_body: Option<Vec<StatementNode>> },
    /// 循环语句
    While { condition: ExpressionNode, body: Vec<StatementNode> },
    /// 返回语句
    Return(Option<ExpressionNode>),
}

/// Ruby 表达式节点
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    /// 标识符
    Identifier(String),
    /// 字面量
    Literal(LiteralNode),
    /// 方法调用
    MethodCall { receiver: Option<Box<ExpressionNode>>, method: String, args: Vec<ExpressionNode> },
    /// 二元操作
    BinaryOp { left: Box<ExpressionNode>, operator: String, right: Box<ExpressionNode> },
    /// 一元操作
    UnaryOp { operator: String, operand: Box<ExpressionNode> },
    /// 数组
    Array(Vec<ExpressionNode>),
    /// 哈希
    Hash(Vec<(ExpressionNode, ExpressionNode)>),
}

/// 字面量节点
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralNode {
    /// 整数
    Integer(i64),
    /// 浮点数
    Float(f64),
    /// 字符串
    String(String),
    /// 符号
    Symbol(String),
    /// 布尔值
    Boolean(bool),
    /// nil
    Nil,
}

/// Ruby AST 访问者 trait
pub trait RubyAstVisitor {
    fn visit_program(&mut self, node: &ProgramNode);
    fn visit_statement(&mut self, stmt: &StatementNode);
    fn visit_expression(&mut self, expr: &ExpressionNode);
    fn visit_literal(&mut self, literal: &LiteralNode);
}
