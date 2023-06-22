use core::range::Range;
use serde::{Deserialize, Serialize};

/// Type alias for source span
type SourceSpan = Range<usize>;

/// Abstract syntax tree for C++ language
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CppRoot {
    pub translation_unit: TranslationUnit,
}

/// Translation unit (top-level structure of a C++ program)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranslationUnit {
    pub external_declarations: Vec<ExternalDeclaration>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// External declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExternalDeclaration {
    /// Function definition
    FunctionDefinition(FunctionDefinition),
    /// Declaration
    Declaration(Declaration),
}

/// Function definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    pub declarator: Declarator,
    pub compound_statement: CompoundStatement,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Declaration {
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    pub init_declarators: Vec<InitDeclarator>,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Declaration specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeclarationSpecifier {
    /// Storage class specifier
    StorageClassSpecifier(StorageClassSpecifier),
    /// Type specifier
    TypeSpecifier(TypeSpecifier),
    /// Type qualifier
    TypeQualifier(TypeQualifier),
    /// Function specifier
    FunctionSpecifier(FunctionSpecifier),
}

/// Storage class specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StorageClassSpecifier {
    Typedef,
    Extern,
    Static,
    Auto,
    Register,
}

/// Type specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeSpecifier {
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    Unsigned,
    Bool,
    Complex,
    Imaginary,
    StructOrUnion(StructOrUnionSpecifier),
    Enum(EnumSpecifier),
    TypedefName(String),
}

/// Type qualifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeQualifier {
    Const,
    Restrict,
    Volatile,
}

/// Function specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FunctionSpecifier {
    Inline,
    Noreturn,
}

/// Struct or union specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructOrUnionSpecifier {
    pub struct_or_union: StructOrUnion,
    pub identifier: Option<String>,
    pub struct_declarations: Option<Vec<StructDeclaration>>,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Struct or union
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StructOrUnion {
    Struct,
    Union,
}

/// Struct declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructDeclaration {
    pub specifier_qualifiers: Vec<SpecifierQualifier>,
    pub struct_declarators: Vec<StructDeclarator>,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Specifier qualifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpecifierQualifier {
    TypeSpecifier(TypeSpecifier),
    TypeQualifier(TypeQualifier),
}

/// Struct declarator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructDeclarator {
    pub declarator: Option<Declarator>,
    pub constant_expression: Option<Expression>,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Enum specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumSpecifier {
    pub identifier: Option<String>,
    pub enumerators: Option<Vec<Enumerator>>,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Enumerator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enumerator {
    pub identifier: String,
    pub constant_expression: Option<Expression>,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Init declarator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitDeclarator {
    pub declarator: Declarator,
    pub initializer: Option<Initializer>,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Declarator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Declarator {
    pub pointer: Option<Pointer>,
    pub direct_declarator: DirectDeclarator,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Pointer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pointer {
    pub type_qualifiers: Vec<TypeQualifier>,
    pub pointer: Option<Box<Pointer>>,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Direct declarator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DirectDeclarator {
    Identifier(String),
    Declarator(Box<Declarator>),
    Array { declarator: Box<DirectDeclarator>, assignment_expression: Option<Expression> },
    Function { declarator: Box<DirectDeclarator>, parameter_type_list: Option<ParameterTypeList>, identifier_list: Option<Vec<String>> },
}

/// Parameter type list
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterTypeList {
    pub parameter_list: Vec<ParameterDeclaration>,
    pub variadic: bool,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Parameter declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterDeclaration {
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    pub declarator: Option<Declarator>,
    pub abstract_declarator: Option<AbstractDeclarator>,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Abstract declarator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbstractDeclarator {
    pub pointer: Option<Pointer>,
    pub direct_abstract_declarator: Option<Box<DirectAbstractDeclarator>>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Direct abstract declarator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DirectAbstractDeclarator {
    AbstractDeclarator(Box<AbstractDeclarator>),
    Array { declarator: Option<Box<DirectAbstractDeclarator>>, assignment_expression: Option<Box<Expression>> },
    Function { declarator: Option<Box<DirectAbstractDeclarator>>, parameter_type_list: Option<ParameterTypeList> },
}

/// Initializer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Initializer {
    AssignmentExpression(Expression),
    InitializerList(Vec<Initializer>),
}

/// Statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    /// Labeled statement
    Labeled(LabeledStatement),
    /// Compound statement
    Compound(CompoundStatement),
    /// Expression statement
    Expression(ExpressionStatement),
    /// Selection statement
    Selection(SelectionStatement),
    /// Iteration statement
    Iteration(IterationStatement),
    /// Jump statement
    Jump(JumpStatement),
}

/// Labeled statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LabeledStatement {
    Label { identifier: String, statement: Box<Statement> },
    Case { constant_expression: Expression, statement: Box<Statement> },
    Default { statement: Box<Statement> },
}

/// Compound statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompoundStatement {
    pub block_items: Vec<BlockItem>,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Block item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockItem {
    Declaration(Declaration),
    Statement(Statement),
}

/// Expression statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpressionStatement {
    pub expression: Option<Expression>,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Selection statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SelectionStatement {
    If { condition: Expression, then_branch: Box<Statement>, else_branch: Option<Box<Statement>> },
    Switch { condition: Expression, statement: Box<Statement> },
}

/// Iteration statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IterationStatement {
    While { condition: Expression, statement: Box<Statement> },
    DoWhile { statement: Box<Statement>, condition: Expression },
    For { initializer: Option<Box<ForInitializer>>, condition: Option<Expression>, increment: Option<Expression>, statement: Box<Statement> },
}

/// For initializer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ForInitializer {
    Expression(Expression),
    Declaration(Declaration),
}

/// Jump statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JumpStatement {
    Goto(String),
    Continue,
    Break,
    Return(Option<Expression>),
}

/// Expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Expression {
    pub kind: ExpressionKind,
    #[serde(with = "oak_core::serde_range")]
    pub span: SourceSpan,
}

/// Expression kind
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpressionKind {
    /// Identifier
    Identifier(String),
    /// Constant
    Constant(String),
    /// String literal
    StringLiteral(String),
    /// Parenthesized expression
    Parenthesized(Box<Expression>),
    /// Array access
    ArrayAccess { array: Box<Expression>, index: Box<Expression> },
    /// Function call
    FunctionCall { function: Box<Expression>, arguments: Vec<Expression> },
    /// Member access
    MemberAccess { object: Box<Expression>, member: String, is_pointer: bool },
    /// Unary operation
    Unary { operator: UnaryOperator, operand: Box<Expression> },
    /// Binary operation
    Binary { left: Box<Expression>, operator: BinaryOperator, right: Box<Expression> },
    /// Conditional expression
    Conditional { condition: Box<Expression>, then_branch: Box<Expression>, else_branch: Box<Expression> },
    /// Assignment
    Assignment { left: Box<Expression>, operator: AssignmentOperator, right: Box<Expression> },
    /// Comma expression
    Comma(Vec<Expression>),
}

/// Unary operator
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    PostIncrement,
    PostDecrement,
    PreIncrement,
    PreDecrement,
    AddressOf,
    Deref,
    Plus,
    Minus,
    BitNot,
    LogicalNot,
    Sizeof,
}

/// Binary operator
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    Multiply,
    Divide,
    Remainder,
    Add,
    Subtract,
    ShiftLeft,
    ShiftRight,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    BitAnd,
    BitXor,
    BitOr,
    LogicalAnd,
    LogicalOr,
}

/// Assignment operator
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AssignmentOperator {
    Assign,
    MulAssign,
    DivAssign,
    RemAssign,
    AddAssign,
    SubAssign,
    ShlAssign,
    ShrAssign,
    AndAssign,
    XorAssign,
    OrAssign,
}

/// Type name
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeName {
    pub specifier_qualifiers: Vec<SpecifierQualifier>,
    pub abstract_declarator: Option<Box<AbstractDeclarator>>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl TypeName {
    /// Create a new type name
    pub fn new(specifier_qualifiers: Vec<SpecifierQualifier>, abstract_declarator: Option<Box<AbstractDeclarator>>, span: Range<usize>) -> Self {
        Self { specifier_qualifiers, abstract_declarator, span }
    }
}

impl CppRoot {
    /// Create a new AST
    pub fn new(translation_unit: TranslationUnit) -> Self {
        Self { translation_unit }
    }
}

impl TranslationUnit {
    /// Create a new translation unit
    pub fn new(external_declarations: Vec<ExternalDeclaration>, span: Range<usize>) -> Self {
        Self { external_declarations, span }
    }
}
