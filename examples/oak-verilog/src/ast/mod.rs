#![doc = include_str!("readme.md")]
use crate::language::VerilogLanguage;
use oak_core::RedNode;

/// Root node of a Verilog AST containing all modules.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerilogRoot {
    /// List of modules in the Verilog source.
    pub modules: Vec<VerilogModule>,
}

/// Represents a Verilog module definition.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerilogModule {
    /// Name of the module.
    pub name: String,
    /// List of port declarations.
    pub ports: Vec<VerilogPort>,
    /// Items within the module body.
    pub items: Vec<VerilogModuleItem>,
}

/// Represents a port declaration in a module.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerilogPort {
    /// Name of the port.
    pub name: String,
    /// Direction of the port (input, output, inout).
    pub direction: Option<String>,
    /// Type of the port (wire, reg).
    pub ty: Option<String>,
}

/// Represents an item within a module body.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VerilogModuleItem {
    /// A variable or net declaration.
    Declaration(VerilogDeclaration),
    /// A continuous assignment statement.
    Assign(VerilogAssign),
    /// An always procedural block.
    Always(VerilogAlways),
    /// An initial procedural block.
    Initial(VerilogInitial),
}

/// Represents a variable or net declaration.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerilogDeclaration {
    /// Type of the declaration (wire, reg, parameter).
    pub ty: String,
    /// Name of the declared variable.
    pub name: String,
    /// Optional initial value.
    pub value: Option<String>,
}

/// Represents a continuous assignment statement.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerilogAssign {
    /// Left-hand side (target) of the assignment.
    pub left: String,
    /// Right-hand side (source) of the assignment.
    pub right: String,
}

/// Represents an always procedural block.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerilogAlways {
    /// Sensitivity list for the always block.
    pub sensitivity: Option<String>,
    /// The procedural statement.
    pub statement: String,
}

/// Represents an initial procedural block.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerilogInitial {
    /// The procedural statement.
    pub statement: String,
}
