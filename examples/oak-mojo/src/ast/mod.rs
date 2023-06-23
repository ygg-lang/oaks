#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MojoStatement {
    Function {
        name: String,
        params: Vec<(String, Option<String>)>, // (name, type)
        return_type: Option<String>,
        body: Vec<MojoStatement>,
    },
    Variable {
        name: String,
        ty: Option<String>,
        value: Option<MojoExpression>,
        is_let: bool,
    },
    Assignment {
        target: MojoExpression,
        value: MojoExpression,
    },
    If {
        condition: MojoExpression,
        then_body: Vec<MojoStatement>,
        else_body: Option<Vec<MojoStatement>>,
    },
    While {
        condition: MojoExpression,
        body: Vec<MojoStatement>,
    },
    For {
        variable: String,
        iterable: MojoExpression,
        body: Vec<MojoStatement>,
    },
    Return(Option<MojoExpression>),
    Expression(MojoExpression),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MojoExpression {
    Literal(MojoLiteral),
    Identifier(String),
    Binary { left: Box<MojoExpression>, op: String, right: Box<MojoExpression> },
    Unary { op: String, right: Box<MojoExpression> },
    Call { callee: Box<MojoExpression>, args: Vec<MojoExpression> },
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MojoLiteral {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
}
