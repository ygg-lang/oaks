use core::range::Range;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HlslRoot {
    pub declarations: Vec<Declaration>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Declaration {
    Function(Function),
    Variable(Variable),
    Struct(Struct),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Function {
    pub name: Identifier,
    pub return_type: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Variable {
    pub name: Identifier,
    pub type_name: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Struct {
    pub name: Identifier,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
