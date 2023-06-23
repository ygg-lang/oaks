#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Type alias for source span.
type SourceSpan = Range<usize>;

/// C++ language abstract syntax tree root.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CppRoot {
    /// Translation unit.
    pub translation_unit: TranslationUnit,
}

/// Translation unit (top-level structure of a C++ program).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TranslationUnit {
    /// List of external declarations.
    pub external_declarations: Vec<ExternalDeclaration>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// External declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExternalDeclaration {
    /// Function definition.
    FunctionDefinition(FunctionDefinition),
    /// Declaration.
    Declaration(Declaration),
}

/// Function definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionDefinition {
    /// Declaration specifiers.
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    /// Declarator.
    pub declarator: Declarator,
    /// Compound statement.
    pub compound_statement: CompoundStatement,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Declaration {
    /// Declaration specifiers.
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    /// List of initialization declarators.
    pub init_declarators: Vec<InitDeclarator>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Declaration specifier.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DeclarationSpecifier {
    /// Storage class specifier.
    StorageClassSpecifier(StorageClassSpecifier),
    /// Type specifier.
    TypeSpecifier(TypeSpecifier),
    /// Type qualifier.
    TypeQualifier(TypeQualifier),
    /// Function specifier.
    FunctionSpecifier(FunctionSpecifier),
}

/// Storage class specifier.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StorageClassSpecifier {
    /// typedef
    Typedef,
    /// extern
    Extern,
    /// static
    Static,
    /// auto
    Auto,
    /// register
    Register,
}

/// Type specifier.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypeSpecifier {
    /// void
    Void,
    /// char
    Char,
    /// short
    Short,
    /// int
    Int,
    /// long
    Long,
    /// float
    Float,
    /// double
    Double,
    /// signed
    Signed,
    /// unsigned
    Unsigned,
    /// bool
    Bool,
    /// _Complex
    Complex,
    /// _Imaginary
    Imaginary,
    /// Struct or union specifier.
    StructOrUnion(StructOrUnionSpecifier),
    /// Enum specifier.
    Enum(EnumSpecifier),
    /// typedef name.
    TypedefName(String),
}

/// Type qualifier
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypeQualifier {
    Const,
    Restrict,
    Volatile,
}

/// Function specifier
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FunctionSpecifier {
    Inline,
    Noreturn,
}

/// Struct or union specifier
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StructOrUnionSpecifier {
    pub struct_or_union: StructOrUnion,
    pub identifier: Option<String>,
    pub struct_declarations: Option<Vec<StructDeclaration>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Struct or union
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StructOrUnion {
    Struct,
    Union,
}

/// Struct declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StructDeclaration {
    pub specifier_qualifiers: Vec<SpecifierQualifier>,
    pub struct_declarators: Vec<StructDeclarator>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Specifier qualifier
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SpecifierQualifier {
    TypeSpecifier(TypeSpecifier),
    TypeQualifier(TypeQualifier),
}

/// Struct declarator
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StructDeclarator {
    pub declarator: Option<Declarator>,
    pub constant_expression: Option<Expression>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Enum specifier
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnumSpecifier {
    pub identifier: Option<String>,
    pub enumerators: Option<Vec<Enumerator>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Enumerator
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Enumerator {
    /// Enumerator identifier.
    pub identifier: String,
    /// Constant expression.
    pub constant_expression: Option<Expression>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Init declarator
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InitDeclarator {
    /// Declarator.
    pub declarator: Declarator,
    /// Initializer.
    pub initializer: Option<Initializer>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Declarator
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Declarator {
    /// Pointer.
    pub pointer: Option<Pointer>,
    /// Direct declarator.
    pub direct_declarator: DirectDeclarator,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Pointer
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pointer {
    /// Type qualifiers.
    pub type_qualifiers: Vec<TypeQualifier>,
    /// Pointer.
    pub pointer: Option<Box<Pointer>>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Direct declarator
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DirectDeclarator {
    /// Identifier.
    Identifier(String),
    /// Declarator.
    Declarator(Box<Declarator>),
    /// Array.
    Array {
        /// Declarator.
        declarator: Box<DirectDeclarator>,
        /// Assignment expression.
        assignment_expression: Option<Expression>,
    },
    /// Function.
    Function {
        /// Declarator.
        declarator: Box<DirectDeclarator>,
        /// Parameter type list.
        parameter_type_list: Option<ParameterTypeList>,
        /// Identifier list.
        identifier_list: Option<Vec<String>>,
    },
}

/// Parameter type list
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ParameterTypeList {
    /// Parameter list.
    pub parameter_list: Vec<ParameterDeclaration>,
    /// Whether it is variadic.
    pub variadic: bool,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Parameter declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ParameterDeclaration {
    /// Declaration specifiers.
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    /// Declarator.
    pub declarator: Option<Declarator>,
    /// Abstract declarator.
    pub abstract_declarator: Option<AbstractDeclarator>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Abstract declarator
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AbstractDeclarator {
    /// Pointer.
    pub pointer: Option<Pointer>,
    /// Direct abstract declarator.
    pub direct_abstract_declarator: Option<Box<DirectAbstractDeclarator>>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Direct abstract declarator
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DirectAbstractDeclarator {
    /// Abstract declarator.
    AbstractDeclarator(Box<AbstractDeclarator>),
    /// Array.
    Array {
        /// Declarator.
        declarator: Option<Box<DirectAbstractDeclarator>>,
        /// Assignment expression.
        assignment_expression: Option<Box<Expression>>,
    },
    /// Function.
    Function {
        /// Declarator.
        declarator: Option<Box<DirectAbstractDeclarator>>,
        /// Parameter type list.
        parameter_type_list: Option<ParameterTypeList>,
    },
}

/// Initializer
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Initializer {
    /// Assignment expression.
    AssignmentExpression(Expression),
    /// Initializer list.
    InitializerList(Vec<Initializer>),
}

/// Statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LabeledStatement {
    /// Label.
    Label {
        /// Identifier.
        identifier: String,
        /// Statement.
        statement: Box<Statement>,
    },
    /// Case.
    Case {
        /// Constant expression.
        constant_expression: Expression,
        /// Statement.
        statement: Box<Statement>,
    },
    /// Default.
    Default {
        /// Statement.
        statement: Box<Statement>,
    },
}

/// Compound statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CompoundStatement {
    /// Block items.
    pub block_items: Vec<BlockItem>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Block item
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BlockItem {
    /// Declaration.
    Declaration(Declaration),
    /// Statement.
    Statement(Statement),
}

/// Expression statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExpressionStatement {
    /// Expression.
    pub expression: Option<Expression>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Selection statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SelectionStatement {
    /// If statement.
    If {
        /// Condition.
        condition: Expression,
        /// Then branch.
        then_branch: Box<Statement>,
        /// Else branch.
        else_branch: Option<Box<Statement>>,
    },
    /// Switch statement.
    Switch {
        /// Condition.
        condition: Expression,
        /// Statement.
        statement: Box<Statement>,
    },
}

/// Iteration statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IterationStatement {
    /// While loop.
    While {
        /// Condition.
        condition: Expression,
        /// Statement.
        statement: Box<Statement>,
    },
    /// Do-while loop.
    DoWhile {
        /// Statement.
        statement: Box<Statement>,
        /// Condition.
        condition: Expression,
    },
    /// For loop.
    For {
        /// Initializer.
        initializer: Option<Box<ForInitializer>>,
        /// Condition.
        condition: Option<Expression>,
        /// Increment.
        increment: Option<Expression>,
        /// Statement.
        statement: Box<Statement>,
    },
}

/// For initializer
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ForInitializer {
    /// Expression.
    Expression(Expression),
    /// Declaration.
    Declaration(Declaration),
}

/// Jump statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JumpStatement {
    /// Goto.
    Goto(String),
    /// Continue.
    Continue,
    /// Break.
    Break,
    /// Return.
    Return(Option<Expression>),
}

/// Expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Expression {
    /// Expression kind.
    pub kind: ExpressionKind,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

/// Expression kind
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    ArrayAccess {
        /// Array.
        array: Box<Expression>,
        /// Index.
        index: Box<Expression>,
    },
    /// Function call
    FunctionCall {
        /// Function.
        function: Box<Expression>,
        /// Arguments.
        arguments: Vec<Expression>,
    },
    /// Member access
    MemberAccess {
        /// Object.
        object: Box<Expression>,
        /// Member.
        member: String,
        /// Whether it's a pointer.
        is_pointer: bool,
    },
    /// Unary operation
    Unary {
        /// Operator.
        operator: UnaryOperator,
        /// Operand.
        operand: Box<Expression>,
    },
    /// Binary operation
    Binary {
        /// Left.
        left: Box<Expression>,
        /// Operator.
        operator: BinaryOperator,
        /// Right.
        right: Box<Expression>,
    },
    /// Conditional expression
    Conditional {
        /// Condition.
        condition: Box<Expression>,
        /// Then branch.
        then_branch: Box<Expression>,
        /// Else branch.
        else_branch: Box<Expression>,
    },
    /// Assignment
    Assignment {
        /// Left.
        left: Box<Expression>,
        /// Operator.
        operator: AssignmentOperator,
        /// Right.
        right: Box<Expression>,
    },
    /// Comma expression
    Comma(Vec<Expression>),
}

/// Unary operator
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnaryOperator {
    /// Post-increment (x++)
    PostIncrement,
    /// Post-decrement (x--)
    PostDecrement,
    /// Pre-increment (++x)
    PreIncrement,
    /// Pre-decrement (--x)
    PreDecrement,
    /// Address of (&x)
    AddressOf,
    /// Indirection (*x)
    Deref,
    /// Unary plus (+x)
    Plus,
    /// Unary minus (-x)
    Minus,
    /// Bitwise NOT (~x)
    BitNot,
    /// Logical NOT (!x)
    LogicalNot,
    /// Size of (sizeof)
    Sizeof,
    /// Align of (alignof)
    AlignOf,
}

/// Binary operator
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BinaryOperator {
    /// Addition (+)
    Add,
    /// Subtraction (-)
    Subtract,
    /// Multiplication (*)
    Multiply,
    /// Division (/)
    Divide,
    /// Modulo (%)
    Remainder,
    /// Bitwise shift left (<<)
    ShiftLeft,
    /// Bitwise shift right (>>)
    ShiftRight,
    /// Less than (<)
    Less,
    /// Greater than (>)
    Greater,
    /// Less than or equal (<=)
    LessEqual,
    /// Greater than or equal (>=)
    GreaterEqual,
    /// Equal (==)
    Equal,
    /// Not equal (!=)
    NotEqual,
    /// Bitwise AND (&)
    BitAnd,
    /// Bitwise XOR (^)
    BitXor,
    /// Bitwise OR (|)
    BitOr,
    /// Logical AND (&&)
    LogicalAnd,
    /// Logical OR (||)
    LogicalOr,
}

/// Assignment operator
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AssignmentOperator {
    /// Assignment (=)
    Assign,
    /// Addition assignment (+=)
    AddAssign,
    /// Subtraction assignment (-=)
    SubAssign,
    /// Multiplication assignment (*=)
    MulAssign,
    /// Division assignment (/=)
    DivAssign,
    /// Modulo assignment (%=)
    RemAssign,
    /// Bitwise shift left assignment (<<=)
    ShlAssign,
    /// Bitwise shift right assignment (>>=)
    ShrAssign,
    /// Bitwise AND assignment (&=)
    AndAssign,
    /// Bitwise XOR assignment (^=)
    XorAssign,
    /// Bitwise OR assignment (|=)
    OrAssign,
}

/// Type name
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeName {
    pub specifier_qualifiers: Vec<SpecifierQualifier>,
    pub abstract_declarator: Option<Box<AbstractDeclarator>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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
