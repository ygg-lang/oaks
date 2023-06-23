use crate::ast::{ClassMember, Decorator, EnumMember, Expression, TypeAnnotation, TypeParameter};
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    ExpressionStatement(ExpressionStatement),
    ImportDeclaration(ImportDeclaration),
    ExportDeclaration(ExportDeclaration),
    Interface(InterfaceDeclaration),
    TypeAlias(TypeAliasDeclaration),
    Enum(EnumDeclaration),
    ReturnStatement(ReturnStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    ForStatement(ForStatement),
    ForInStatement(ForInStatement),
    ForOfStatement(ForOfStatement),
    SwitchStatement(SwitchStatement),
    TryStatement(TryStatement),
    ThrowStatement(ThrowStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    BlockStatement(BlockStatement),
    Namespace(NamespaceDeclaration),
}

impl Statement {
    pub fn span(&self) -> Range<usize> {
        match self {
            Statement::VariableDeclaration(d) => d.span.clone(),
            Statement::FunctionDeclaration(d) => d.span.clone(),
            Statement::ClassDeclaration(d) => d.span.clone(),
            Statement::ExpressionStatement(s) => s.span.clone(),
            Statement::ImportDeclaration(d) => d.span.clone(),
            Statement::ExportDeclaration(d) => d.span.clone(),
            Statement::Interface(d) => d.span.clone(),
            Statement::TypeAlias(d) => d.span.clone(),
            Statement::Enum(d) => d.span.clone(),
            Statement::ReturnStatement(s) => s.span.clone(),
            Statement::IfStatement(s) => s.span.clone(),
            Statement::WhileStatement(s) => s.span.clone(),
            Statement::DoWhileStatement(s) => s.span.clone(),
            Statement::ForStatement(s) => s.span.clone(),
            Statement::ForInStatement(s) => s.span.clone(),
            Statement::ForOfStatement(s) => s.span.clone(),
            Statement::SwitchStatement(s) => s.span.clone(),
            Statement::TryStatement(s) => s.span.clone(),
            Statement::ThrowStatement(s) => s.span.clone(),
            Statement::BreakStatement(s) => s.span.clone(),
            Statement::ContinueStatement(s) => s.span.clone(),
            Statement::BlockStatement(s) => s.span.clone(),
            Statement::Namespace(s) => s.span.clone(),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExpressionStatement {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub expression: Expression,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReturnStatement {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub argument: Option<Expression>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ThrowStatement {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub argument: Expression,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BreakStatement {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub label: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ContinueStatement {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub label: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NamespaceDeclaration {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub name: String,
    pub body: Vec<Statement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InterfaceDeclaration {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub name: String,
    pub type_params: Vec<TypeParameter>,
    pub extends: Vec<TypeAnnotation>,
    pub body: Vec<ClassMember>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeAliasDeclaration {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub name: String,
    pub type_params: Vec<TypeParameter>,
    pub ty: TypeAnnotation,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnumDeclaration {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub name: String,
    pub members: Vec<EnumMember>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IfStatement {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub test: Expression,
    pub consequent: Box<Statement>,
    pub alternate: Option<Box<Statement>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WhileStatement {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub test: Expression,
    pub body: Box<Statement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DoWhileStatement {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub body: Box<Statement>,
    pub test: Expression,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForStatement {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub initializer: Option<Box<Statement>>,
    pub test: Option<Expression>,
    pub incrementor: Option<Expression>,
    pub body: Box<Statement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForInStatement {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub left: Box<Statement>,
    pub right: Expression,
    pub body: Box<Statement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForOfStatement {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub left: Box<Statement>,
    pub right: Expression,
    pub body: Box<Statement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SwitchStatement {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SwitchCase {
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TryStatement {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub block: Vec<Statement>,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<Vec<Statement>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CatchClause {
    pub param: Option<String>,
    pub body: Vec<Statement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BlockStatement {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub statements: Vec<Statement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ImportSpecifier {
    Default(String),
    Namespace(String),
    Named { local: String, imported: String },
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExportSpecifier {
    pub local: String,
    pub exported: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImportDeclaration {
    pub module_specifier: String,
    pub specifiers: Vec<ImportSpecifier>,
    pub is_type_only: bool,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExportDeclaration {
    pub declaration: Option<Box<Statement>>,
    pub specifiers: Vec<ExportSpecifier>,
    pub source: Option<String>,
    pub is_default: bool,
    pub is_type_only: bool,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VariableDeclaration {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub name: String,
    pub ty: Option<TypeAnnotation>,
    pub value: Option<Expression>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionDeclaration {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub name: String,
    pub type_params: Vec<TypeParameter>,
    pub params: Vec<FunctionParam>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Vec<Statement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClassDeclaration {
    pub decorators: Vec<Decorator>,
    pub is_declare: bool,
    pub name: String,
    pub type_params: Vec<TypeParameter>,
    pub extends: Option<TypeAnnotation>,
    pub implements: Vec<TypeAnnotation>,
    pub is_abstract: bool,
    pub body: Vec<ClassMember>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionParam {
    pub decorators: Vec<Decorator>,
    pub name: String,
    pub ty: Option<TypeAnnotation>,
    pub optional: bool,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
