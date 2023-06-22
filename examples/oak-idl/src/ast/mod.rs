use core::range::Range;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdlRoot {
    pub items: Vec<IdlItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IdlItem {
    Module(Module),
    Interface(Interface),
    Struct(Struct),
    Enum(Enum),
    Typedef(Typedef),
    Const(Const),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub items: Vec<IdlItem>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Interface {
    pub name: String,
    pub members: Vec<IdlMember>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IdlMember {
    Attribute(Attribute),
    Operation(Operation),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub type_name: String,
    pub readonly: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Operation {
    pub name: String,
    pub return_type: String,
    pub params: Vec<Param>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub type_name: String,
    pub direction: ParamDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParamDirection {
    In,
    Out,
    Inout,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Typedef {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Const {
    pub name: String,
    pub type_name: String,
    pub value: String,
}
