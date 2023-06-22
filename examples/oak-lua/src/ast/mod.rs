use core::range::Range;
use oak_core::source::{SourceBuffer, ToSource};
use serde::{Deserialize, Serialize};

/// Lua 根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaRoot {
    pub statements: Vec<LuaStatement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl ToSource for LuaRoot {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        for stmt in &self.statements {
            stmt.to_source(buffer);
            buffer.push("\n");
        }
    }
}

/// Lua 语句
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LuaStatement {
    Local(LuaLocalStatement),
    Assignment(LuaAssignmentStatement),
    Expression(LuaExpression),
    Return(LuaReturnStatement),
    // ... 其他语句
}

impl ToSource for LuaStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            LuaStatement::Local(s) => s.to_source(buffer),
            LuaStatement::Assignment(s) => s.to_source(buffer),
            LuaStatement::Expression(e) => e.to_source(buffer),
            LuaStatement::Return(s) => s.to_source(buffer),
        }
    }
}

/// 本地变量声明
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaLocalStatement {
    pub names: Vec<String>,
    pub values: Vec<LuaExpression>,
}

impl ToSource for LuaLocalStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("local ");
        for (i, name) in self.names.iter().enumerate() {
            if i > 0 {
                buffer.push(", ");
            }
            buffer.push(name);
        }
        if !self.values.is_empty() {
            buffer.push(" = ");
            for (i, val) in self.values.iter().enumerate() {
                if i > 0 {
                    buffer.push(", ");
                }
                val.to_source(buffer);
            }
        }
    }
}

/// 赋值语句
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaAssignmentStatement {
    pub targets: Vec<LuaExpression>,
    pub values: Vec<LuaExpression>,
}

impl ToSource for LuaAssignmentStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        for (i, target) in self.targets.iter().enumerate() {
            if i > 0 {
                buffer.push(", ");
            }
            target.to_source(buffer);
        }
        buffer.push(" = ");
        for (i, val) in self.values.iter().enumerate() {
            if i > 0 {
                buffer.push(", ");
            }
            val.to_source(buffer);
        }
    }
}

/// 返回语句
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaReturnStatement {
    pub values: Vec<LuaExpression>,
}

impl ToSource for LuaReturnStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("return ");
        for (i, val) in self.values.iter().enumerate() {
            if i > 0 {
                buffer.push(", ");
            }
            val.to_source(buffer);
        }
    }
}

/// Lua 表达式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LuaExpression {
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Binary(Box<LuaBinaryExpression>),
    // ... 其他表达式
}

impl ToSource for LuaExpression {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            LuaExpression::Identifier(id) => buffer.push(id),
            LuaExpression::Number(n) => buffer.push(&n.to_string()),
            LuaExpression::String(s) => {
                buffer.push("\"");
                buffer.push(s);
                buffer.push("\"");
            }
            LuaExpression::Boolean(b) => buffer.push(if *b { "true" } else { "false" }),
            LuaExpression::Nil => buffer.push("nil"),
            LuaExpression::Binary(bin) => bin.to_source(buffer),
        }
    }
}

/// 二元表达式
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        self.right.to_source(buffer);
    }
}
