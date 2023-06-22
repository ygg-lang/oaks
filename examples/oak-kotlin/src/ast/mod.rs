use oak_core::Range;
use serde::{Deserialize, Serialize};

/// Kotlin root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KotlinRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Declaration {
    Class {
        name: String,
        members: Vec<Declaration>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Function {
        name: String,
        params: Vec<Parameter>,
        body: Vec<Statement>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Variable {
        name: String,
        is_val: bool,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_name: Option<String>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Return(Option<String>),
    Expression(String),
    Variable { name: String, is_val: bool },
    Assignment { target: String, value: String },
}
