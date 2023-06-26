use core::range::Range;
use std::{string::String, vec::Vec};

use super::{ExecutableStmt, SpecificationStmt, TypeSpec};

/// A placeholder statement node in the Fortran AST.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Statement {}

/// The root node of the Fortran AST.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FortranRoot {
    /// The optional name of the program.
    pub name: Option<String>,
    /// The list of program units contained in this root.
    pub units: Vec<ProgramUnitKind>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Fortran program unit kinds.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProgramUnitKind {
    /// Main program.
    MainProgram(MainProgramNode),
    /// Subroutine.
    Subroutine(SubroutineNode),
    /// Function.
    Function(FunctionNode),
    /// Module.
    Module(ModuleNode),
    /// Submodule.
    Submodule(SubmoduleNode),
    /// Block data.
    BlockData(BlockDataNode),
}

/// Main program node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MainProgramNode {
    /// The optional name of the main program.
    pub name: Option<String>,
    /// The specification statements in the main program.
    pub specification_part: Vec<SpecificationStmt>,
    /// The executable statements in the main program.
    pub execution_part: Vec<ExecutableStmt>,
    /// Internal subprograms defined within the main program.
    pub internal_subprograms: Vec<ProgramUnitKind>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Subroutine node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubroutineNode {
    /// The name of the subroutine.
    pub name: String,
    /// The list of parameter names.
    pub parameters: Vec<String>,
    /// The specification statements in the subroutine.
    pub specification_part: Vec<SpecificationStmt>,
    /// The executable statements in the subroutine.
    pub execution_part: Vec<ExecutableStmt>,
    /// Internal subprograms defined within the subroutine.
    pub internal_subprograms: Vec<ProgramUnitKind>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Function node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionNode {
    /// The name of the function.
    pub name: String,
    /// The list of parameter names.
    pub parameters: Vec<String>,
    /// The optional name of the result variable.
    pub result_name: Option<String>,
    /// The optional return type specification.
    pub return_type: Option<TypeSpec>,
    /// The specification statements in the function.
    pub specification_part: Vec<SpecificationStmt>,
    /// The executable statements in the function.
    pub execution_part: Vec<ExecutableStmt>,
    /// Internal subprograms defined within the function.
    pub internal_subprograms: Vec<ProgramUnitKind>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Module node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModuleNode {
    /// The name of the module.
    pub name: String,
    /// The specification statements in the module.
    pub specification_part: Vec<SpecificationStmt>,
    /// Subprograms defined within the module.
    pub module_subprograms: Vec<ProgramUnitKind>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Submodule node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubmoduleNode {
    /// The name of the parent module.
    pub parent_name: String,
    /// The name of the submodule.
    pub name: String,
    /// The specification statements in the submodule.
    pub specification_part: Vec<SpecificationStmt>,
    /// Subprograms defined within the submodule.
    pub module_subprograms: Vec<ProgramUnitKind>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Block data node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockDataNode {
    /// The optional name of the block data.
    pub name: Option<String>,
    /// The specification statements in the block data.
    pub specification_part: Vec<SpecificationStmt>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
