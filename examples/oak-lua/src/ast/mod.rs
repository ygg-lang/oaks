#![doc = include_str!("readme.md")]
use core::range::Range;
use oak_core::source::{SourceBuffer, ToSource};
#[cfg(feature = "oak-pretty-print")]
use oak_pretty_print::{AsDocument, Document};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Lua 根节点
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LuaRoot {
    pub statements: Vec<LuaStatement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for LuaRoot {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        for stmt in &self.statements {
            stmt.to_source(buffer);
            buffer.push("\n")
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for LuaRoot {
    fn as_document(&self) -> Document<'_> {
        Document::join(self.statements.iter().map(|s| s.as_document()), Document::Line)
    }
}

/// Lua 语句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LuaStatement {
    Local(LuaLocalStatement),
    Assignment(LuaAssignmentStatement),
    Expression(LuaExpression),
    Return(LuaReturnStatement),
    If(LuaIfStatement),
    While(LuaWhileStatement),
    For(LuaForStatement),
    Repeat(LuaRepeatStatement),
    Function(LuaFunctionStatement),
    Break,
    Do(Vec<LuaStatement>),
    Goto(String),
    Label(String),
}

impl ToSource for LuaStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            LuaStatement::Local(s) => s.to_source(buffer),
            LuaStatement::Assignment(s) => s.to_source(buffer),
            LuaStatement::Expression(e) => e.to_source(buffer),
            LuaStatement::Return(s) => s.to_source(buffer),
            LuaStatement::If(s) => s.to_source(buffer),
            LuaStatement::While(s) => s.to_source(buffer),
            LuaStatement::For(s) => s.to_source(buffer),
            LuaStatement::Repeat(s) => s.to_source(buffer),
            LuaStatement::Function(s) => s.to_source(buffer),
            LuaStatement::Break => buffer.push("break"),
            LuaStatement::Do(stmts) => {
                buffer.push("do\n");
                for stmt in stmts {
                    stmt.to_source(buffer);
                    buffer.push("\n")
                }
                buffer.push("end")
            }
            LuaStatement::Goto(label) => {
                buffer.push("goto ");
                buffer.push(label)
            }
            LuaStatement::Label(name) => {
                buffer.push("::");
                buffer.push(name);
                buffer.push("::")
            }
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for LuaStatement {
    fn as_document(&self) -> Document<'_> {
        let mut buffer = SourceBuffer::new();
        self.to_source(&mut buffer);
        Document::Text(buffer.finish().into())
    }
}

/// 本地变量声明
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct LuaLocalStatement {
    pub names: Vec<String>,
    pub values: Vec<LuaExpression>,
}

impl ToSource for LuaLocalStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("local ");
        for (i, name) in self.names.iter().enumerate() {
            if i > 0 {
                buffer.push(", ")
            }
            buffer.push(name)
        }
        if !self.values.is_empty() {
            buffer.push(" = ");
            for (i, val) in self.values.iter().enumerate() {
                if i > 0 {
                    buffer.push(", ")
                }
                val.to_source(buffer)
            }
        }
    }
}

/// 赋值语句
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct LuaAssignmentStatement {
    pub targets: Vec<LuaExpression>,
    pub values: Vec<LuaExpression>,
}

impl ToSource for LuaAssignmentStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        for (i, target) in self.targets.iter().enumerate() {
            if i > 0 {
                buffer.push(", ")
            }
            target.to_source(buffer)
        }
        buffer.push(" = ");
        for (i, val) in self.values.iter().enumerate() {
            if i > 0 {
                buffer.push(", ")
            }
            val.to_source(buffer)
        }
    }
}

/// 返回语句
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct LuaReturnStatement {
    pub values: Vec<LuaExpression>,
}

impl ToSource for LuaReturnStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("return ");
        for (i, val) in self.values.iter().enumerate() {
            if i > 0 {
                buffer.push(", ")
            }
            val.to_source(buffer)
        }
    }
}

/// If 语句
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct LuaIfStatement {
    pub condition: LuaExpression,
    pub then_block: Vec<LuaStatement>,
    pub else_ifs: Vec<(LuaExpression, Vec<LuaStatement>)>,
    pub else_block: Option<Vec<LuaStatement>>,
}

impl ToSource for LuaIfStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("if ");
        self.condition.to_source(buffer);
        buffer.push(" then\n");
        for stmt in &self.then_block {
            stmt.to_source(buffer);
            buffer.push("\n")
        }
        for (cond, block) in &self.else_ifs {
            buffer.push("elseif ");
            cond.to_source(buffer);
            buffer.push(" then\n");
            for stmt in block {
                stmt.to_source(buffer);
                buffer.push("\n")
            }
        }
        if let Some(block) = &self.else_block {
            buffer.push("else\n");
            for stmt in block {
                stmt.to_source(buffer);
                buffer.push("\n")
            }
        }
        buffer.push("end")
    }
}

/// While 语句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LuaWhileStatement {
    pub condition: LuaExpression,
    pub block: Vec<LuaStatement>,
}

impl ToSource for LuaWhileStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("while ");
        self.condition.to_source(buffer);
        buffer.push(" do\n");
        for stmt in &self.block {
            stmt.to_source(buffer);
            buffer.push("\n")
        }
        buffer.push("end")
    }
}

/// For 语句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LuaForStatement {
    Numeric { variable: String, start: LuaExpression, end: LuaExpression, step: Option<LuaExpression>, block: Vec<LuaStatement> },
    Generic { variables: Vec<String>, iterators: Vec<LuaExpression>, block: Vec<LuaStatement> },
}

impl ToSource for LuaForStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            LuaForStatement::Numeric { variable, start, end, step, block } => {
                buffer.push("for ");
                buffer.push(variable);
                buffer.push(" = ");
                start.to_source(buffer);
                buffer.push(", ");
                end.to_source(buffer);
                if let Some(s) = step {
                    buffer.push(", ");
                    s.to_source(buffer)
                }
                buffer.push(" do\n");
                for stmt in block {
                    stmt.to_source(buffer);
                    buffer.push("\n")
                }
                buffer.push("end")
            }
            LuaForStatement::Generic { variables, iterators, block } => {
                buffer.push("for ");
                for (i, var) in variables.iter().enumerate() {
                    if i > 0 {
                        buffer.push(", ")
                    }
                    buffer.push(var)
                }
                buffer.push(" in ");
                for (i, it) in iterators.iter().enumerate() {
                    if i > 0 {
                        buffer.push(", ")
                    }
                    it.to_source(buffer)
                }
                buffer.push(" do\n");
                for stmt in block {
                    stmt.to_source(buffer);
                    buffer.push("\n")
                }
                buffer.push("end")
            }
        }
    }
}

/// Repeat 语句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LuaRepeatStatement {
    pub block: Vec<LuaStatement>,
    pub condition: LuaExpression,
}

impl ToSource for LuaRepeatStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("repeat\n");
        for stmt in &self.block {
            stmt.to_source(buffer);
            buffer.push("\n")
        }
        buffer.push("until ");
        self.condition.to_source(buffer)
    }
}

/// 函数定义语句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LuaFunctionStatement {
    pub name: Vec<String>,
    pub receiver: Option<String>,
    pub parameters: Vec<String>,
    pub is_vararg: bool,
    pub block: Vec<LuaStatement>,
}

impl ToSource for LuaFunctionStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("function ");
        for (i, part) in self.name.iter().enumerate() {
            if i > 0 {
                buffer.push(".")
            }
            buffer.push(part)
        }
        if let Some(recv) = &self.receiver {
            buffer.push(":");
            buffer.push(recv)
        }
        buffer.push("(");
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                buffer.push(", ")
            }
            buffer.push(param)
        }
        if self.is_vararg {
            if !self.parameters.is_empty() {
                buffer.push(", ")
            }
            buffer.push("...")
        }
        buffer.push(")\n");
        for stmt in &self.block {
            stmt.to_source(buffer);
            buffer.push("\n")
        }
        buffer.push("end")
    }
}

/// Lua 表达式
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LuaExpression {
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Binary(Box<LuaBinaryExpression>),
    Unary(Box<LuaUnaryExpression>),
    Call(Box<LuaCallExpression>),
    Table(LuaTableConstructor),
    Function(LuaFunctionExpression),
    Index(Box<LuaIndexExpression>),
    Member(Box<LuaMemberExpression>),
    Vararg,
}

impl ToSource for LuaExpression {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            LuaExpression::Identifier(id) => buffer.push(id),
            LuaExpression::Number(n) => buffer.push(&n.to_string()),
            LuaExpression::String(s) => {
                buffer.push("\"");
                buffer.push(s);
                buffer.push("\"")
            }
            LuaExpression::Boolean(b) => buffer.push(if *b { "true" } else { "false" }),
            LuaExpression::Nil => buffer.push("nil"),
            LuaExpression::Binary(bin) => bin.to_source(buffer),
            LuaExpression::Unary(un) => un.to_source(buffer),
            LuaExpression::Call(call) => call.to_source(buffer),
            LuaExpression::Table(table) => table.to_source(buffer),
            LuaExpression::Function(func) => func.to_source(buffer),
            LuaExpression::Index(idx) => idx.to_source(buffer),
            LuaExpression::Member(mem) => mem.to_source(buffer),
            LuaExpression::Vararg => buffer.push("..."),
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for LuaExpression {
    fn as_document(&self) -> Document<'_> {
        let mut buffer = SourceBuffer::new();
        self.to_source(&mut buffer);
        Document::Text(buffer.finish().into())
    }
}

/// 一元表达式
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LuaUnaryExpression {
    pub op: String,
    pub operand: LuaExpression,
}

impl ToSource for LuaUnaryExpression {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(&self.op);
        self.operand.to_source(buffer)
    }
}

/// 二元表达式
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LuaBinaryExpression {
    pub left: LuaExpression,
    pub op: String,
    pub right: LuaExpression,
}

impl ToSource for LuaBinaryExpression {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        self.left.to_source(buffer);
        buffer.push(" ");
        buffer.push(&self.op);
        buffer.push(" ");
        self.right.to_source(buffer)
    }
}

/// 函数调用表达式
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LuaCallExpression {
    pub function: LuaExpression,
    pub arguments: Vec<LuaExpression>,
}

impl ToSource for LuaCallExpression {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        self.function.to_source(buffer);
        buffer.push("(");
        for (i, arg) in self.arguments.iter().enumerate() {
            if i > 0 {
                buffer.push(", ")
            }
            arg.to_source(buffer)
        }
        buffer.push(")")
    }
}

/// 表构造器
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LuaTableConstructor {
    pub fields: Vec<LuaTableField>,
}

impl ToSource for LuaTableConstructor {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("{");
        for (i, field) in self.fields.iter().enumerate() {
            if i > 0 {
                buffer.push(", ")
            }
            field.to_source(buffer)
        }
        buffer.push("}")
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LuaTableField {
    Keyed { key: LuaExpression, value: LuaExpression },
    Named { name: String, value: LuaExpression },
    List { value: LuaExpression },
}

impl ToSource for LuaTableField {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            LuaTableField::Keyed { key, value } => {
                buffer.push("[");
                key.to_source(buffer);
                buffer.push("] = ");
                value.to_source(buffer)
            }
            LuaTableField::Named { name, value } => {
                buffer.push(name);
                buffer.push(" = ");
                value.to_source(buffer)
            }
            LuaTableField::List { value } => value.to_source(buffer),
        }
    }
}

/// 匿名函数表达式
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LuaFunctionExpression {
    pub parameters: Vec<String>,
    pub is_vararg: bool,
    pub block: Vec<LuaStatement>,
}

impl ToSource for LuaFunctionExpression {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("function(");
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                buffer.push(", ")
            }
            buffer.push(param)
        }
        if self.is_vararg {
            if !self.parameters.is_empty() {
                buffer.push(", ")
            }
            buffer.push("...")
        }
        buffer.push(")\n");
        for stmt in &self.block {
            stmt.to_source(buffer);
            buffer.push("\n")
        }
        buffer.push("end")
    }
}

/// 索引访问表达式
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LuaIndexExpression {
    pub table: LuaExpression,
    pub index: LuaExpression,
}

impl ToSource for LuaIndexExpression {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        self.table.to_source(buffer);
        buffer.push("[");
        self.index.to_source(buffer);
        buffer.push("]")
    }
}

/// 成员访问表达式
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LuaMemberExpression {
    pub table: LuaExpression,
    pub member: String,
    pub is_method: bool,
}

impl ToSource for LuaMemberExpression {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        self.table.to_source(buffer);
        if self.is_method {
            buffer.push(":")
        }
        else {
            buffer.push(".")
        }
        buffer.push(&self.member)
    }
}
