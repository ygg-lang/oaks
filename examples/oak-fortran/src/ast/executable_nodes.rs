use core::range::Range;
use std::{boxed::Box, string::String, vec::Vec};

use super::{Dimension, ExprNode, SpecificationStmt};

/// Executable statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExecutableStmt {
    /// Assignment statement.
    Assignment(AssignmentNode),
    /// Call statement.
    Call(CallNode),
    /// If construct.
    IfConstruct(IfConstructNode),
    /// Do construct.
    DoConstruct(DoConstructNode),
    /// Select Case.
    SelectCase(SelectCaseNode),
    /// Where construct.
    WhereConstruct(WhereConstructNode),
    /// Forall construct.
    ForallConstruct(ForallConstructNode),
    /// Associate construct.
    AssociateConstruct(AssociateConstructNode),
    /// Block construct.
    BlockConstruct(BlockConstructNode),
    /// Critical construct.
    CriticalConstruct(CriticalConstructNode),
    /// Allocate statement.
    Allocate(AllocateNode),
    /// Deallocate statement.
    Deallocate(DeallocateNode),
    /// Nullify statement.
    Nullify(NullifyNode),
    /// Stop statement.
    Stop(StopNode),
    /// Return statement.
    Return(ReturnNode),
    /// Continue statement.
    Continue,
    /// Cycle statement.
    Cycle(Option<String>),
    /// Exit statement.
    Exit(Option<String>),
    /// Read statement.
    Read(ReadNode),
    /// Write statement.
    Write(WriteNode),
    /// Print statement.
    Print(PrintNode),
}

/// Assignment node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssignmentNode {
    /// The left-hand side variable expression.
    pub variable: Box<ExprNode>,
    /// The right-hand side expression to assign.
    pub expression: Box<ExprNode>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Call node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CallNode {
    /// The name of the procedure to call.
    pub procedure_name: String,
    /// The arguments passed to the procedure.
    pub arguments: Vec<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// If construct node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfConstructNode {
    /// The condition expression for the if block.
    pub condition: Box<ExprNode>,
    /// The statements in the then block.
    pub then_part: Vec<ExecutableStmt>,
    /// The else-if blocks with their conditions and statements.
    pub else_if_parts: Vec<(Box<ExprNode>, Vec<ExecutableStmt>)>,
    /// The optional else block statements.
    pub else_part: Option<Vec<ExecutableStmt>>,
    /// The optional construct name.
    pub name: Option<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Do construct node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DoConstructNode {
    /// The optional construct name.
    pub name: Option<String>,
    /// The optional loop control (iterative, while, or concurrent).
    pub control: Option<DoControl>,
    /// The statements in the loop body.
    pub body: Vec<ExecutableStmt>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Do control.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DoControl {
    /// Iterative do loop with variable, start, end, and optional step.
    Iterative {
        /// The loop variable name.
        variable: String,
        /// The start value expression.
        start: Box<ExprNode>,
        /// The end value expression.
        end: Box<ExprNode>,
        /// The optional step value expression.
        step: Option<Box<ExprNode>>,
    },
    /// While loop with condition.
    While(Box<ExprNode>),
    /// Concurrent loop with header and locality specs.
    Concurrent {
        /// The concurrent header with control list and optional mask.
        header: ConcurrentHeader,
        /// The locality specifications.
        locality: Vec<LocalitySpec>,
    },
}

/// Concurrent header.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConcurrentHeader {
    /// The list of concurrent controls.
    pub control_list: Vec<ConcurrentControl>,
    /// The optional mask expression.
    pub mask: Option<Box<ExprNode>>,
}

/// Concurrent control.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConcurrentControl {
    /// The index variable name.
    pub name: String,
    /// The start value expression.
    pub start: Box<ExprNode>,
    /// The end value expression.
    pub end: Box<ExprNode>,
    /// The optional step value expression.
    pub step: Option<Box<ExprNode>>,
}

/// Locality specification.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LocalitySpec {
    /// Local.
    Local(Vec<String>),
    /// Local init.
    LocalInit(Vec<String>),
    /// Shared.
    Shared(Vec<String>),
    /// Default none.
    DefaultNone,
}

/// Select Case node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectCaseNode {
    /// The expression to match against cases.
    pub expression: Box<ExprNode>,
    /// The list of case blocks.
    pub cases: Vec<CaseConstruct>,
    /// The optional construct name.
    pub name: Option<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Case construct.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaseConstruct {
    /// The case selector (specific values or default).
    pub selector: CaseSelector,
    /// The statements in this case block.
    pub body: Vec<ExecutableStmt>,
}

/// Case selector.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CaseSelector {
    /// Case.
    Case(Vec<CaseValue>),
    /// Default.
    Default,
}

/// Case value.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CaseValue {
    /// Single value.
    Single(Box<ExprNode>),
    /// Range.
    Range(Option<Box<ExprNode>>, Option<Box<ExprNode>>),
}

/// Where construct node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhereConstructNode {
    /// The mask expression for the where block.
    pub mask: Box<ExprNode>,
    /// The statements in the where block.
    pub where_body: Vec<ExecutableStmt>,
    /// The else-where blocks with optional masks and statements.
    pub else_where_parts: Vec<(Option<Box<ExprNode>>, Vec<ExecutableStmt>)>,
    /// The optional construct name.
    pub name: Option<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Forall construct node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForallConstructNode {
    /// The concurrent header with control list and optional mask.
    pub header: ConcurrentHeader,
    /// The statements in the forall body.
    pub body: Vec<ExecutableStmt>,
    /// The optional construct name.
    pub name: Option<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Associate construct node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssociateConstructNode {
    /// The list of associations.
    pub associates: Vec<Associate>,
    /// The statements in the associate body.
    pub body: Vec<ExecutableStmt>,
    /// The optional construct name.
    pub name: Option<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Associate.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Associate {
    /// The name of the associate.
    pub name: String,
    /// The expression to associate with.
    pub expression: Box<ExprNode>,
}

/// Block construct node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockConstructNode {
    /// The specification statements in the block.
    pub specification_part: Vec<SpecificationStmt>,
    /// The executable statements in the block.
    pub execution_part: Vec<ExecutableStmt>,
    /// The optional construct name.
    pub name: Option<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Critical construct node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CriticalConstructNode {
    /// The statements in the critical section.
    pub body: Vec<ExecutableStmt>,
    /// The optional construct name.
    pub name: Option<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Allocate node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AllocateNode {
    /// The objects to allocate.
    pub objects: Vec<Allocation>,
    /// The allocation options.
    pub options: Vec<AllocOpt>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Allocation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Allocation {
    /// The variable to allocate.
    pub variable: Box<ExprNode>,
    /// The optional array specification.
    pub array_spec: Option<Vec<Dimension>>,
}

/// Allocation option.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AllocOpt {
    /// Stat.
    Stat(Box<ExprNode>),
    /// Error message.
    Errmsg(Box<ExprNode>),
    /// Source.
    Source(Box<ExprNode>),
    /// Mold.
    Mold(Box<ExprNode>),
}

/// Deallocate node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeallocateNode {
    /// The objects to deallocate.
    pub objects: Vec<Box<ExprNode>>,
    /// The deallocation options.
    pub options: Vec<DeallocOpt>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Deallocation option.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeallocOpt {
    /// Stat.
    Stat(Box<ExprNode>),
    /// Error message.
    Errmsg(Box<ExprNode>),
}

/// Nullify node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NullifyNode {
    /// The pointer objects to nullify.
    pub pointers: Vec<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Stop node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StopNode {
    /// The optional stop code expression.
    pub stop_code: Option<Box<ExprNode>>,
    /// The optional quiet expression.
    pub quiet: Option<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Return node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReturnNode {
    /// The optional return expression.
    pub expression: Option<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Read node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReadNode {
    /// The I/O control specifications.
    pub io_control_spec: Vec<IoControlSpec>,
    /// The input items to read into.
    pub input_items: Vec<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Write node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WriteNode {
    /// The I/O control specifications.
    pub io_control_spec: Vec<IoControlSpec>,
    /// The output items to write.
    pub output_items: Vec<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Print node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrintNode {
    /// The optional format expression.
    pub format: Option<Box<ExprNode>>,
    /// The output items to print.
    pub output_items: Vec<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// IO control specification.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IoControlSpec {
    /// Unit.
    Unit(Box<ExprNode>),
    /// Format.
    Format(Box<ExprNode>),
    /// Nml.
    Nml(Box<ExprNode>),
    /// Iomsg.
    Iomsg(Box<ExprNode>),
    /// Iostat.
    Iostat(Box<ExprNode>),
    /// Advance.
    Advance(Box<ExprNode>),
    /// Other.
    Other(String, Box<ExprNode>),
}
