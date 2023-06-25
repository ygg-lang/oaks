#![doc = include_str!("readme.md")]
use crate::{FortranLanguage, parser::element_type::FortranElementType};
use core::range::Range;
use oak_core::{GreenNode, RedNode};
use std::{boxed::Box, string::String, vec::Vec};
type SyntaxKind = FortranElementType;
type SyntaxNode<'a> = RedNode<'a, FortranLanguage>;
type FortranKind = FortranElementType;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Statement {
    // Placeholder fields
}

impl Statement {
    pub fn cast<'a>(_node: SyntaxNode<'a>) -> Option<Self> {
        Some(Statement {})
    }
}

/// The root node of the Fortran AST.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FortranRoot {
    pub name: Option<String>,
    pub units: Vec<ProgramUnitKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl FortranRoot {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FortranKind::Program
    }

    fn cast(syntax: SyntaxNode<'_>) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            // TODO: Implement casting for units
            Some(Self { name: None, units: Vec::new(), span: syntax.span() })
        }
        else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode<'_> {
        // Implementation requires holding the SyntaxNode. Omitted for simplicity.
        unimplemented!("FortranRoot::syntax")
    }
}

/// Fortran program unit kinds
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProgramUnitKind {
    /// Main program
    MainProgram(MainProgramNode),
    /// Subroutine
    Subroutine(SubroutineNode),
    /// Function
    Function(FunctionNode),
    /// Module
    Module(ModuleNode),
    /// Submodule
    Submodule(SubmoduleNode),
    /// Block data
    BlockData(BlockDataNode),
}

/// Main program node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MainProgramNode {
    pub name: Option<String>,
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub internal_subprograms: Vec<ProgramUnitKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Subroutine node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubroutineNode {
    pub name: String,
    pub parameters: Vec<String>,
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub internal_subprograms: Vec<ProgramUnitKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Function node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionNode {
    pub name: String,
    pub parameters: Vec<String>,
    pub result_name: Option<String>,
    pub return_type: Option<TypeSpec>,
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub internal_subprograms: Vec<ProgramUnitKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Module node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModuleNode {
    pub name: String,
    pub specification_part: Vec<SpecificationStmt>,
    pub module_subprograms: Vec<ProgramUnitKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Submodule node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubmoduleNode {
    pub parent_name: String,
    pub name: String,
    pub specification_part: Vec<SpecificationStmt>,
    pub module_subprograms: Vec<ProgramUnitKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Block data node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockDataNode {
    pub name: Option<String>,
    pub specification_part: Vec<SpecificationStmt>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Specification statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpecificationStmt {
    /// Type declaration
    TypeDeclaration(TypeDeclarationNode),
    /// Parameter declaration
    Parameter(ParameterNode),
    /// Implicit declaration
    Implicit(ImplicitNode),
    /// Use statement
    Use(UseNode),
    /// Import statement
    Import(ImportNode),
    /// Interface declaration
    Interface(InterfaceNode),
    /// Procedure declaration
    Procedure(ProcedureNode),
    /// Generic declaration
    Generic(GenericNode),
}

/// Executable statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExecutableStmt {
    /// Assignment statement
    Assignment(AssignmentNode),
    /// Call statement
    Call(CallNode),
    /// If construct
    IfConstruct(IfConstructNode),
    /// Do construct
    DoConstruct(DoConstructNode),
    /// Select Case
    SelectCase(SelectCaseNode),
    /// Where construct
    WhereConstruct(WhereConstructNode),
    /// Forall construct
    ForallConstruct(ForallConstructNode),
    /// Associate construct
    AssociateConstruct(AssociateConstructNode),
    /// Block construct
    BlockConstruct(BlockConstructNode),
    /// Critical construct
    CriticalConstruct(CriticalConstructNode),
    /// Allocate statement
    Allocate(AllocateNode),
    /// Deallocate statement
    Deallocate(DeallocateNode),
    /// Nullify statement
    Nullify(NullifyNode),
    /// Stop statement
    Stop(StopNode),
    /// Return statement
    Return(ReturnNode),
    /// Continue statement
    Continue,
    /// Cycle statement
    Cycle(Option<String>),
    /// Exit statement
    Exit(Option<String>),
    /// Read statement
    Read(ReadNode),
    /// Write statement
    Write(WriteNode),
    /// Print statement
    Print(PrintNode),
}

/// Type specification
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypeSpec {
    /// Integer type
    Integer(Option<KindSelector>),
    /// Real type
    Real(Option<KindSelector>),
    /// Double precision type
    DoublePrecision,
    /// Complex type
    Complex(Option<KindSelector>),
    /// Character type
    Character(Option<CharacterSelector>),
    /// Logical type
    Logical(Option<KindSelector>),
    /// Derived type
    Derived(String),
    /// Class type
    Class(String),
    /// Type star
    TypeStar,
}

/// Kind selector
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KindSelector {
    /// Expression
    Expression(Box<ExprNode>),
}

/// Character selector
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CharacterSelector {
    /// Length
    Length(Box<ExprNode>),
    /// Length and kind
    LengthAndKind(Box<ExprNode>, Box<ExprNode>),
}

/// Type declaration node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeDeclarationNode {
    pub type_spec: TypeSpec,
    pub attributes: Vec<Attribute>,
    pub entities: Vec<EntityDecl>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Attribute
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Attribute {
    /// Allocatable
    Allocatable,
    /// Asynchronous
    Asynchronous,
    /// Bind
    Bind(String),
    /// Dimension
    Dimension(Vec<Dimension>),
    /// External
    External,
    /// Intent
    Intent(Intent),
    /// Intrinsic
    Intrinsic,
    /// Optional
    Optional,
    /// Parameter
    Parameter,
    /// Pointer
    Pointer,
    /// Protected
    Protected,
    /// Private
    Private,
    /// Public
    Public,
    /// Save
    Save,
    /// Target
    Target,
    /// Value
    Value,
    /// Volatile
    Volatile,
}

/// Intent
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Intent {
    /// In
    In,
    /// Out
    Out,
    /// InOut
    InOut,
}

/// Dimension
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Dimension {
    /// Explicit shape
    Explicit(Box<ExprNode>, Box<ExprNode>),
    /// Assumed shape
    Assumed(Option<Box<ExprNode>>),
    /// Deferred shape
    Deferred,
    /// Assumed size
    AssumedSize(Option<Box<ExprNode>>),
    /// Assumed rank
    AssumedRank,
}

/// Entity declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntityDecl {
    pub name: String,
    pub array_spec: Option<Vec<Dimension>>,
    pub char_length: Option<Box<ExprNode>>,
    pub initialization: Option<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Parameter node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParameterNode {
    pub entities: Vec<EntityDecl>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Implicit node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImplicitNode {
    /// None
    None,
    /// Spec
    Spec(Vec<ImplicitSpec>),
}

/// Implicit specification
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImplicitSpec {
    pub type_spec: TypeSpec,
    pub letter_ranges: Vec<LetterRange>,
}

/// Letter range
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LetterRange {
    pub start: char,
    pub end: Option<char>,
}

/// Use node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UseNode {
    pub module_name: String,
    pub nature: Option<ModuleNature>,
    pub rename_list: Vec<Rename>,
    pub only_list: Vec<Only>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Module nature
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ModuleNature {
    /// Intrinsic
    Intrinsic,
    /// Non-intrinsic
    NonIntrinsic,
}

/// Rename
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rename {
    pub local_name: String,
    pub use_name: String,
}

/// Only
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Only {
    /// Generic
    Generic(String),
    /// Rename
    Rename(Rename),
}

/// Import node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportNode {
    pub import_names: Vec<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Interface node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterfaceNode {
    pub generic_spec: Option<GenericSpec>,
    pub interface_bodies: Vec<ProgramUnitKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Generic specification
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GenericSpec {
    /// Generic name
    GenericName(String),
    /// Operator
    Operator(String),
    /// Assignment
    Assignment,
    /// Read defined
    ReadDefined,
    /// Write defined
    WriteDefined,
}

/// Procedure node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProcedureNode {
    pub interface_name: Option<String>,
    pub attributes: Vec<Attribute>,
    pub entities: Vec<ProcedureEntity>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Procedure entity
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProcedureEntity {
    pub name: String,
    pub binding_name: Option<String>,
}

/// Generic node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericNode {
    pub generic_spec: GenericSpec,
    pub access_spec: Option<Attribute>,
    pub procedure_names: Vec<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Assignment node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssignmentNode {
    pub variable: Box<ExprNode>,
    pub expression: Box<ExprNode>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Call node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CallNode {
    pub procedure_name: String,
    pub arguments: Vec<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// If construct node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfConstructNode {
    pub condition: Box<ExprNode>,
    pub then_part: Vec<ExecutableStmt>,
    pub else_if_parts: Vec<(Box<ExprNode>, Vec<ExecutableStmt>)>,
    pub else_part: Option<Vec<ExecutableStmt>>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Do construct node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DoConstructNode {
    pub name: Option<String>,
    pub control: Option<DoControl>,
    pub body: Vec<ExecutableStmt>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Do control
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DoControl {
    /// Iterative
    Iterative { variable: String, start: Box<ExprNode>, end: Box<ExprNode>, step: Option<Box<ExprNode>> },
    /// While
    While(Box<ExprNode>),
    /// Concurrent
    Concurrent { header: ConcurrentHeader, locality: Vec<LocalitySpec> },
}

/// Concurrent header
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConcurrentHeader {
    pub control_list: Vec<ConcurrentControl>,
    pub mask: Option<Box<ExprNode>>,
}

/// Concurrent control
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConcurrentControl {
    pub name: String,
    pub start: Box<ExprNode>,
    pub end: Box<ExprNode>,
    pub step: Option<Box<ExprNode>>,
}

/// Locality specification
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LocalitySpec {
    /// Local
    Local(Vec<String>),
    /// Local init
    LocalInit(Vec<String>),
    /// Shared
    Shared(Vec<String>),
    /// Default none
    DefaultNone,
}

/// Select Case node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectCaseNode {
    pub expression: Box<ExprNode>,
    pub cases: Vec<CaseConstruct>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Case construct
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaseConstruct {
    pub selector: CaseSelector,
    pub body: Vec<ExecutableStmt>,
}

/// Case selector
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CaseSelector {
    /// Case
    Case(Vec<CaseValue>),
    /// Default
    Default,
}

/// Case value
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CaseValue {
    /// Single value
    Single(Box<ExprNode>),
    /// Range
    Range(Option<Box<ExprNode>>, Option<Box<ExprNode>>),
}

/// Where construct node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhereConstructNode {
    pub mask: Box<ExprNode>,
    pub where_body: Vec<ExecutableStmt>,
    pub else_where_parts: Vec<(Option<Box<ExprNode>>, Vec<ExecutableStmt>)>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Forall construct node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForallConstructNode {
    pub header: ConcurrentHeader,
    pub body: Vec<ExecutableStmt>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Associate construct node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssociateConstructNode {
    pub associates: Vec<Associate>,
    pub body: Vec<ExecutableStmt>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Associate
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Associate {
    pub name: String,
    pub expression: Box<ExprNode>,
}

/// Block construct node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockConstructNode {
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Critical construct node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CriticalConstructNode {
    pub body: Vec<ExecutableStmt>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Allocate node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AllocateNode {
    pub objects: Vec<Allocation>,
    pub options: Vec<AllocOpt>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Allocation
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Allocation {
    pub variable: Box<ExprNode>,
    pub array_spec: Option<Vec<Dimension>>,
}

/// Allocation option
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AllocOpt {
    /// Stat
    Stat(Box<ExprNode>),
    /// Error message
    Errmsg(Box<ExprNode>),
    /// Source
    Source(Box<ExprNode>),
    /// Mold
    Mold(Box<ExprNode>),
}

/// Deallocate node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeallocateNode {
    pub objects: Vec<Box<ExprNode>>,
    pub options: Vec<DeallocOpt>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Deallocation option
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeallocOpt {
    /// Stat
    Stat(Box<ExprNode>),
    /// Error message
    Errmsg(Box<ExprNode>),
}

/// Nullify node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NullifyNode {
    pub pointers: Vec<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Stop node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StopNode {
    pub stop_code: Option<Box<ExprNode>>,
    pub quiet: Option<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Return node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReturnNode {
    pub expression: Option<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Read node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReadNode {
    pub io_control_spec: Vec<IoControlSpec>,
    pub input_items: Vec<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Write node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WriteNode {
    pub io_control_spec: Vec<IoControlSpec>,
    pub output_items: Vec<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Print node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrintNode {
    pub format: Option<Box<ExprNode>>,
    pub output_items: Vec<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// IO control specification
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IoControlSpec {
    /// Unit
    Unit(Box<ExprNode>),
    /// Format
    Format(Box<ExprNode>),
    /// Nml
    Nml(Box<ExprNode>),
    /// Iomsg
    Iomsg(Box<ExprNode>),
    /// Iostat
    Iostat(Box<ExprNode>),
    /// Advance
    Advance(Box<ExprNode>),
    /// Other
    Other(String, Box<ExprNode>),
}

/// Expression node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExprNode {
    /// Literal
    Literal(LiteralNode),
    /// Name
    Name(String),
    /// Array element
    ArrayElement(ArrayElementNode),
    /// Function reference
    FunctionReference(FunctionReferenceNode),
    /// Unary operation
    UnaryOp(UnaryOpNode),
    /// Binary operation
    BinaryOp(BinaryOpNode),
    /// Parenthesized expression
    ParenExpr(Box<ExprNode>),
    /// Structure constructor
    StructureConstructor(StructureConstructorNode),
}

/// Literal node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiteralNode {
    pub value: String,
    pub kind: LiteralKind,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Literal kind
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LiteralKind {
    /// Integer
    Integer,
    /// Real
    Real,
    /// Complex
    Complex,
    /// Character
    Character,
    /// Logical
    Logical,
}

/// Array element node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayElementNode {
    pub name: String,
    pub subscripts: Vec<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Function reference node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionReferenceNode {
    pub name: String,
    pub arguments: Vec<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Unary operation node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnaryOpNode {
    pub operator: UnaryOperator,
    pub operand: Box<ExprNode>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Unary operator
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnaryOperator {
    /// Not
    Not,
    /// Plus
    Plus,
    /// Minus
    Minus,
}

/// Binary operation node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinaryOpNode {
    pub operator: BinaryOperator,
    pub left: Box<ExprNode>,
    pub right: Box<ExprNode>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Binary operator
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BinaryOperator {
    /// Add
    Add,
    /// Subtract
    Subtract,
    /// Multiply
    Multiply,
    /// Divide
    Divide,
    /// Power
    Power,
    /// Concat
    Concat,
    /// Equal
    Equal,
    /// Not equal
    NotEqual,
    /// Less than
    LessThan,
    /// Less than or equal
    LessThanOrEqual,
    /// Greater than
    GreaterThan,
    /// Greater than or equal
    GreaterThanOrEqual,
    /// And
    And,
    /// Or
    Or,
    /// Eqv
    Eqv,
    /// Neqv
    Neqv,
}

/// Structure constructor node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructureConstructorNode {
    pub type_name: String,
    pub args: Vec<(Option<String>, Box<ExprNode>)>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
