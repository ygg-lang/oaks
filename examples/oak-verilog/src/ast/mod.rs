#![doc = include_str!("readme.md")]
use crate::language::VerilogLanguage;
use oak_core::RedNode;

#[derive(Debug, Clone)]
pub struct VerilogRoot {
    pub modules: Vec<VerilogModule>,
}

#[derive(Debug, Clone)]
pub struct VerilogModule {
    pub name: String,
    pub ports: Vec<VerilogPort>,
    pub items: Vec<VerilogModuleItem>,
}

#[derive(Debug, Clone)]
pub struct VerilogPort {
    pub name: String,
    pub direction: Option<String>, // input, output, inout
    pub ty: Option<String>,        // wire, reg
}

#[derive(Debug, Clone)]
pub enum VerilogModuleItem {
    Declaration(VerilogDeclaration),
    Assign(VerilogAssign),
    Always(VerilogAlways),
    Initial(VerilogInitial),
}

#[derive(Debug, Clone)]
pub struct VerilogDeclaration {
    pub ty: String, // wire, reg, parameter
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VerilogAssign {
    pub left: String,
    pub right: String,
}

#[derive(Debug, Clone)]
pub struct VerilogAlways {
    pub sensitivity: Option<String>,
    pub statement: String,
}

#[derive(Debug, Clone)]
pub struct VerilogInitial {
    pub statement: String,
}
