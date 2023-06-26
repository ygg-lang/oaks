use core::range::Range;
use std::{boxed::Box, string::String, vec::Vec};

use super::{Dimension, ExprNode, ProgramUnitKind};

/// Specification statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpecificationStmt {
    /// Type declaration.
    TypeDeclaration(TypeDeclarationNode),
    /// Parameter declaration.
    Parameter(ParameterNode),
    /// Implicit declaration.
    Implicit(ImplicitNode),
    /// Use statement.
    Use(UseNode),
    /// Import statement.
    Import(ImportNode),
    /// Interface declaration.
    Interface(InterfaceNode),
    /// Procedure declaration.
    Procedure(ProcedureNode),
    /// Generic declaration.
    Generic(GenericNode),
}

/// Type specification.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypeSpec {
    /// Integer type.
    Integer(Option<KindSelector>),
    /// Real type.
    Real(Option<KindSelector>),
    /// Double precision type.
    DoublePrecision,
    /// Complex type.
    Complex(Option<KindSelector>),
    /// Character type.
    Character(Option<CharacterSelector>),
    /// Logical type.
    Logical(Option<KindSelector>),
    /// Derived type.
    Derived(String),
    /// Class type.
    Class(String),
    /// Type star.
    TypeStar,
}

/// Kind selector.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KindSelector {
    /// Expression.
    Expression(Box<ExprNode>),
}

/// Character selector.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CharacterSelector {
    /// Length.
    Length(Box<ExprNode>),
    /// Length and kind.
    LengthAndKind(Box<ExprNode>, Box<ExprNode>),
}

/// Type declaration node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeDeclarationNode {
    /// The type specification for the declared entities.
    pub type_spec: TypeSpec,
    /// The attributes applied to the declared entities.
    pub attributes: Vec<Attribute>,
    /// The list of entity declarations.
    pub entities: Vec<EntityDecl>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Attribute.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Attribute {
    /// Allocatable.
    Allocatable,
    /// Asynchronous.
    Asynchronous,
    /// Bind.
    Bind(String),
    /// Dimension.
    Dimension(Vec<Dimension>),
    /// External.
    External,
    /// Intent.
    Intent(Intent),
    /// Intrinsic.
    Intrinsic,
    /// Optional.
    Optional,
    /// Parameter.
    Parameter,
    /// Pointer.
    Pointer,
    /// Protected.
    Protected,
    /// Private.
    Private,
    /// Public.
    Public,
    /// Save.
    Save,
    /// Target.
    Target,
    /// Value.
    Value,
    /// Volatile.
    Volatile,
}

/// Intent.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Intent {
    /// In.
    In,
    /// Out.
    Out,
    /// InOut.
    InOut,
}

/// Entity declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntityDecl {
    /// The name of the entity.
    pub name: String,
    /// The optional array specification.
    pub array_spec: Option<Vec<Dimension>>,
    /// The optional character length specification.
    pub char_length: Option<Box<ExprNode>>,
    /// The optional initialization expression.
    pub initialization: Option<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Parameter node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParameterNode {
    /// The list of parameter entity declarations.
    pub entities: Vec<EntityDecl>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Implicit node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImplicitNode {
    /// None.
    None,
    /// Spec.
    Spec(Vec<ImplicitSpec>),
}

/// Implicit specification.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImplicitSpec {
    /// The type specification for the implicit declaration.
    pub type_spec: TypeSpec,
    /// The letter ranges for the implicit declaration.
    pub letter_ranges: Vec<LetterRange>,
}

/// Letter range.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LetterRange {
    /// The starting letter of the range.
    pub start: char,
    /// The optional ending letter of the range.
    pub end: Option<char>,
}

/// Use node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UseNode {
    /// The name of the module to use.
    pub module_name: String,
    /// The optional module nature (intrinsic or non-intrinsic).
    pub nature: Option<ModuleNature>,
    /// The list of rename specifications.
    pub rename_list: Vec<Rename>,
    /// The list of only specifications.
    pub only_list: Vec<Only>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Module nature.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ModuleNature {
    /// Intrinsic.
    Intrinsic,
    /// Non-intrinsic.
    NonIntrinsic,
}

/// Rename specification.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rename {
    /// The local name to use.
    pub local_name: String,
    /// The original name in the module.
    pub use_name: String,
}

/// Only specification.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Only {
    /// Generic.
    Generic(String),
    /// Rename.
    Rename(Rename),
}

/// Import node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportNode {
    /// The list of names to import.
    pub import_names: Vec<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Interface node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterfaceNode {
    /// The optional generic specification for the interface.
    pub generic_spec: Option<GenericSpec>,
    /// The interface bodies (procedure interfaces).
    pub interface_bodies: Vec<ProgramUnitKind>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Generic specification.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GenericSpec {
    /// Generic name.
    GenericName(String),
    /// Operator.
    Operator(String),
    /// Assignment.
    Assignment,
    /// Read defined.
    ReadDefined,
    /// Write defined.
    WriteDefined,
}

/// Procedure node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProcedureNode {
    /// The optional interface name for the procedure.
    pub interface_name: Option<String>,
    /// The attributes applied to the procedure.
    pub attributes: Vec<Attribute>,
    /// The procedure entity declarations.
    pub entities: Vec<ProcedureEntity>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Procedure entity.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProcedureEntity {
    /// The name of the procedure.
    pub name: String,
    /// The optional binding name.
    pub binding_name: Option<String>,
}

/// Generic node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericNode {
    /// The generic specification.
    pub generic_spec: GenericSpec,
    /// The optional access specification.
    pub access_spec: Option<Attribute>,
    /// The list of procedure names bound to this generic.
    pub procedure_names: Vec<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
