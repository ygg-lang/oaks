use core::range::Range;
use serde::{Deserialize, Serialize};

/// TypeScript AST 根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptRoot {
    pub statements: Vec<Statement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    ExpressionStatement(Expression),
    ImportDeclaration(ImportDeclaration),
    ExportDeclaration(ExportDeclaration),
    ReturnStatement(Option<Expression>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDeclaration {
    pub name: String,
    pub extends: Option<String>,
    pub body: Vec<ClassMember>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassMember {
    Property {
        name: String,
        ty: Option<String>,
        initializer: Option<Expression>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Method {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDeclaration {
    pub module_specifier: String,
    pub imports: Vec<String>,
    #[serde(with = "oak_core::serde_range")]
    pub span: core::range::Range<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDeclaration {
    pub declaration: Box<Statement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: core::range::Range<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub name: String,
    pub value: Option<Expression>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Statement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Identifier(String),
    NumericLiteral(f64),
    StringLiteral(String),
    BigIntLiteral(String),
    BooleanLiteral(bool),
    NullLiteral,
    RegexLiteral(String),
    TemplateString(String),
    UnaryExpression { operator: String, argument: Box<Expression> },
    BinaryExpression { left: Box<Expression>, operator: String, right: Box<Expression> },
    ConditionalExpression { test: Box<Expression>, consequent: Box<Expression>, alternate: Box<Expression> },
    MemberExpression { object: Box<Expression>, property: Box<Expression>, computed: bool, optional: bool },
    CallExpression { func: Box<Expression>, args: Vec<Expression> },
    NewExpression { func: Box<Expression>, args: Vec<Expression> },
    AssignmentExpression { left: Box<Expression>, operator: String, right: Box<Expression> },
    AsExpression { expression: Box<Expression>, type_annotation: String },
}

impl TypeScriptRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { statements: vec![], span }
    }
}
