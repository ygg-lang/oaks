#![doc = include_str!("readme.md")]
/// C language abstract syntax.
#[derive(Debug, Clone, PartialEq)]
pub struct CRoot {
    /// The translation unit containing the source code structure.
    pub translation_unit: TranslationUnit,
    /// The source span of the root node.
    pub span: core::range::Range<usize>,
}

/// Translation unit (the top-level structure of a C program).
#[derive(Debug, Clone, PartialEq)]
pub struct TranslationUnit {
    /// List of external declarations.
    pub external_declarations: Vec<ExternalDeclaration>,
    /// The source span of the translation unit.
    pub span: core::range::Range<usize>,
}

/// External declaration.
#[derive(Debug, Clone, PartialEq)]
pub enum ExternalDeclaration {
    /// Function definition.
    FunctionDefinition(FunctionDefinition),
    /// Declaration.
    Declaration(Declaration),
}

impl ExternalDeclaration {
    /// Returns the source span of the external declaration.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::FunctionDefinition(n) => n.span.clone(),
            Self::Declaration(n) => n.span.clone(),
        }
    }
}

/// Function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    /// Declaration specifiers (e.g., return type, storage class).
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    /// The declarator for the function.
    pub declarator: Declarator,
    /// The body of the function.
    pub compound_statement: CompoundStatement,
    /// The source span of the function definition.
    pub span: core::range::Range<usize>,
}

/// Declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    /// Declaration specifiers.
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    /// List of declarators being initialized.
    pub init_declarators: Vec<InitDeclarator>,
    /// The source span of the declaration.
    pub span: core::range::Range<usize>,
}

/// Declaration specifier.
#[derive(Debug, Clone, PartialEq)]
pub enum DeclarationSpecifier {
    /// Storage class specifier (e.g., `static`, `extern`).
    StorageClassSpecifier(StorageClassSpecifier),
    /// Type specifier (e.g., `int`, `char`).
    TypeSpecifier(TypeSpecifier),
    /// Type qualifier (e.g., `const`, `volatile`).
    TypeQualifier(TypeQualifier),
    /// Function specifier (e.g., `inline`).
    FunctionSpecifier(FunctionSpecifier),
}

impl DeclarationSpecifier {
    /// Returns the source span of the declaration specifier.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::StorageClassSpecifier(n) => n.span(),
            Self::TypeSpecifier(n) => n.span(),
            Self::TypeQualifier(n) => n.span(),
            Self::FunctionSpecifier(n) => n.span(),
        }
    }
}

/// Storage class specifier.
#[derive(Debug, Clone, PartialEq)]
pub enum StorageClassSpecifier {
    /// `typedef`
    Typedef {
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// `extern`
    Extern {
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// `static`
    Static {
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// `auto`
    Auto {
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// `register`
    Register {
        /// Source span.
        span: core::range::Range<usize>,
    },
}

impl StorageClassSpecifier {
    /// Returns the source span of the storage class specifier.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Typedef { span } => span.clone(),
            Self::Extern { span } => span.clone(),
            Self::Static { span } => span.clone(),
            Self::Auto { span } => span.clone(),
            Self::Register { span } => span.clone(),
        }
    }
}

/// Type specifier.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeSpecifier {
    /// `void`
    Void {
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// `char`
    Char {
        /// Source span.
        span: core::range::Range<usize>,
    },
    Short {
        span: core::range::Range<usize>,
    },
    Int {
        span: core::range::Range<usize>,
    },
    Long {
        span: core::range::Range<usize>,
    },
    Float {
        span: core::range::Range<usize>,
    },
    Double {
        span: core::range::Range<usize>,
    },
    Signed {
        span: core::range::Range<usize>,
    },
    Unsigned {
        span: core::range::Range<usize>,
    },
    Bool {
        span: core::range::Range<usize>,
    },
    Complex {
        span: core::range::Range<usize>,
    },
    Imaginary {
        span: core::range::Range<usize>,
    },
    StructOrUnion(StructOrUnionSpecifier),
    Enum(EnumSpecifier),
    TypedefName(String, core::range::Range<usize>),
}

impl TypeSpecifier {
    /// Returns the source span of the type specifier.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Void { span } => span.clone(),
            Self::Char { span } => span.clone(),
            Self::Short { span } => span.clone(),
            Self::Int { span } => span.clone(),
            Self::Long { span } => span.clone(),
            Self::Float { span } => span.clone(),
            Self::Double { span } => span.clone(),
            Self::Signed { span } => span.clone(),
            Self::Unsigned { span } => span.clone(),
            Self::Bool { span } => span.clone(),
            Self::Complex { span } => span.clone(),
            Self::Imaginary { span } => span.clone(),
            Self::StructOrUnion(n) => n.span.clone(),
            Self::Enum(n) => n.span.clone(),
            Self::TypedefName(_, span) => span.clone(),
        }
    }
}

/// Type qualifier.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeQualifier {
    /// `const`
    Const {
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// `restrict`
    Restrict {
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// `volatile`
    Volatile {
        /// Source span.
        span: core::range::Range<usize>,
    },
}

impl TypeQualifier {
    /// Returns the source span of the type qualifier.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Const { span } => span.clone(),
            Self::Restrict { span } => span.clone(),
            Self::Volatile { span } => span.clone(),
        }
    }
}

/// Function specifier.
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionSpecifier {
    /// `inline`
    Inline {
        /// Source span.
        span: core::range::Range<usize>,
    },
}

impl FunctionSpecifier {
    /// Returns the source span of the function specifier.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Inline { span } => span.clone(),
        }
    }
}

/// Struct or union specifier.
#[derive(Debug, Clone, PartialEq)]
pub struct StructOrUnionSpecifier {
    /// Whether it's a struct or union.
    pub kind: StructOrUnion,
    /// Optional tag identifier.
    pub identifier: Option<String>,
    /// List of struct declarations.
    pub struct_declarations: Vec<StructDeclaration>,
    /// Source span.
    pub span: core::range::Range<usize>,
}

/// Struct or union keyword.
#[derive(Debug, Clone, PartialEq)]
pub enum StructOrUnion {
    /// `struct`
    Struct {
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// `union`
    Union {
        /// Source span.
        span: core::range::Range<usize>,
    },
}

impl StructOrUnion {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Struct { span } => span.clone(),
            Self::Union { span } => span.clone(),
        }
    }
}

/// Struct declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    /// List of specifiers and qualifiers.
    pub specifier_qualifier_list: Vec<SpecifierQualifier>,
    /// List of struct declarators.
    pub struct_declarator_list: Vec<StructDeclarator>,
    /// Source span.
    pub span: core::range::Range<usize>,
}

/// Specifier or qualifier.
#[derive(Debug, Clone, PartialEq)]
pub enum SpecifierQualifier {
    /// Type specifier.
    TypeSpecifier(TypeSpecifier),
    /// Type qualifier.
    TypeQualifier(TypeQualifier),
}

impl SpecifierQualifier {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::TypeSpecifier(n) => n.span(),
            Self::TypeQualifier(n) => n.span(),
        }
    }
}

/// Struct declarator.
#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclarator {
    /// The declarator.
    pub declarator: Option<Declarator>,
    /// Optional bit-field width expression.
    pub constant_expression: Option<Expression>,
    /// Source span.
    pub span: core::range::Range<usize>,
}

/// Enum specifier.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumSpecifier {
    /// Optional tag identifier.
    pub identifier: Option<String>,
    /// List of enumerators.
    pub enumerators: Vec<Enumerator>,
    /// Source span.
    pub span: core::range::Range<usize>,
}

/// Enumerator.
#[derive(Debug, Clone, PartialEq)]
pub struct Enumerator {
    /// Enumerator identifier.
    pub identifier: String,
    /// Optional constant expression value.
    pub constant_expression: Option<Expression>,
    /// Source span.
    pub span: core::range::Range<usize>,
}

/// Init declarator.
#[derive(Debug, Clone, PartialEq)]
pub struct InitDeclarator {
    /// The declarator.
    pub declarator: Declarator,
    /// Optional initializer.
    pub initializer: Option<Initializer>,
    /// Source span.
    pub span: core::range::Range<usize>,
}

/// Declarator.
#[derive(Debug, Clone, PartialEq)]
pub struct Declarator {
    /// Optional pointer prefix.
    pub pointer: Option<Pointer>,
    /// Direct declarator.
    pub direct_declarator: DirectDeclarator,
    /// Source span.
    pub span: core::range::Range<usize>,
}

/// Pointer.
#[derive(Debug, Clone, PartialEq)]
pub struct Pointer {
    /// List of type qualifiers for this pointer level.
    pub type_qualifiers: Vec<TypeQualifier>,
    /// Optional nested pointer (for `**`, etc.).
    pub pointer: Option<Box<Pointer>>,
    /// Source span.
    pub span: core::range::Range<usize>,
}

/// Direct declarator.
#[derive(Debug, Clone, PartialEq)]
pub enum DirectDeclarator {
    /// Identifier.
    Identifier(String, core::range::Range<usize>),
    /// Parenthesized declarator.
    Declarator(Box<Declarator>, core::range::Range<usize>),
    /// Array declarator.
    Array {
        /// The declarator being declared as an array.
        direct_declarator: Box<DirectDeclarator>,
        /// Type qualifiers inside `[]`.
        type_qualifiers: Vec<TypeQualifier>,
        /// Optional assignment expression for size.
        assignment_expression: Option<Box<Expression>>,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// Function declarator.
    Function {
        /// The declarator being declared as a function.
        direct_declarator: Box<DirectDeclarator>,
        /// Parameter list.
        parameter_list: ParameterList,
        /// Source span.
        span: core::range::Range<usize>,
    },
}

impl DirectDeclarator {
    /// Returns the source span of the direct declarator.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Identifier(_, span) => span.clone(),
            Self::Declarator(n, _) => n.span.clone(),
            Self::Array { span, .. } => span.clone(),
            Self::Function { span, .. } => span.clone(),
        }
    }
}

/// Parameter list.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterList {
    /// List of parameter declarations.
    pub parameter_declarations: Vec<ParameterDeclaration>,
    /// Whether the function is variadic (ends with `...`).
    pub variadic: bool,
    /// Source span.
    pub span: core::range::Range<usize>,
}

/// Parameter declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterDeclaration {
    /// Declaration specifiers.
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    /// Optional declarator.
    pub declarator: Option<Declarator>,
    /// Optional abstract declarator.
    pub abstract_declarator: Option<AbstractDeclarator>,
    /// Source span.
    pub span: core::range::Range<usize>,
}

/// Abstract declarator.
#[derive(Debug, Clone, PartialEq)]
pub struct AbstractDeclarator {
    /// Optional pointer prefix.
    pub pointer: Option<Pointer>,
    /// Direct abstract declarator.
    pub direct_abstract_declarator: Option<Box<DirectAbstractDeclarator>>,
    /// Source span.
    pub span: core::range::Range<usize>,
}

/// Direct abstract declarator.
#[derive(Debug, Clone, PartialEq)]
pub enum DirectAbstractDeclarator {
    /// Parenthesized abstract declarator.
    AbstractDeclarator(Box<AbstractDeclarator>),
    /// Array abstract declarator.
    Array {
        /// Optional direct abstract declarator.
        declarator: Option<Box<DirectAbstractDeclarator>>,
        /// Optional size expression.
        assignment_expression: Option<Box<Expression>>,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// Function abstract declarator.
    Function {
        /// Optional direct abstract declarator.
        declarator: Option<Box<DirectAbstractDeclarator>>,
        /// Parameter list.
        parameter_list: Option<ParameterList>,
        /// Source span.
        span: core::range::Range<usize>,
    },
}

impl DirectAbstractDeclarator {
    /// Returns the source span of the direct abstract declarator.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::AbstractDeclarator(n) => n.span.clone(),
            Self::Array { span, .. } => span.clone(),
            Self::Function { span, .. } => span.clone(),
        }
    }
}

/// Initializer.
#[derive(Debug, Clone, PartialEq)]
pub enum Initializer {
    /// Assignment expression.
    AssignmentExpression(Expression),
    /// Initializer list `{ ... }`.
    InitializerList(Vec<Initializer>, core::range::Range<usize>),
}

impl Initializer {
    /// Returns the source span of the initializer.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::AssignmentExpression(n) => n.span.clone(),
            Self::InitializerList(_, span) => span.clone(),
        }
    }
}

/// Statement.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Labeled statement.
    Labeled(LabeledStatement),
    /// Compound statement.
    Compound(CompoundStatement),
    /// Expression statement.
    Expression(ExpressionStatement),
    /// Selection statement (if, switch).
    Selection(SelectionStatement),
    /// Iteration statement (while, do, for).
    Iteration(IterationStatement),
    /// Jump statement (goto, continue, break, return).
    Jump(JumpStatement),
}

impl Statement {
    /// Returns the source span of the statement.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Labeled(n) => n.span(),
            Self::Compound(n) => n.span.clone(),
            Self::Expression(n) => n.span.clone(),
            Self::Selection(n) => n.span(),
            Self::Iteration(n) => n.span(),
            Self::Jump(n) => n.span(),
        }
    }
}

/// Labeled statement.
#[derive(Debug, Clone, PartialEq)]
pub enum LabeledStatement {
    /// `identifier: statement`
    Label {
        /// Label name.
        identifier: String,
        /// Labeled statement.
        statement: Box<Statement>,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// `case constant-expression: statement`
    Case {
        /// Case expression.
        constant_expression: Expression,
        /// Statement.
        statement: Box<Statement>,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// `default: statement`
    Default {
        /// Statement.
        statement: Box<Statement>,
        /// Source span.
        span: core::range::Range<usize>,
    },
}

impl LabeledStatement {
    /// Returns the source span of the labeled statement.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Label { span, .. } => span.clone(),
            Self::Case { span, .. } => span.clone(),
            Self::Default { span, .. } => span.clone(),
        }
    }
}

/// Compound statement (block).
#[derive(Debug, Clone, PartialEq)]
pub struct CompoundStatement {
    /// List of block items (declarations or statements).
    pub block_items: Vec<BlockItem>,
    /// Source span.
    pub span: core::range::Range<usize>,
}

/// Block item.
#[derive(Debug, Clone, PartialEq)]
pub enum BlockItem {
    /// Declaration.
    Declaration(Declaration),
    /// Statement.
    Statement(Statement),
}

impl BlockItem {
    /// Returns the source span of the block item.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Declaration(n) => n.span.clone(),
            Self::Statement(n) => n.span(),
        }
    }
}

/// Expression statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    /// Optional expression.
    pub expression: Option<Expression>,
    /// Source span.
    pub span: core::range::Range<usize>,
}

/// Selection statement.
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionStatement {
    /// `if (condition) then_statement else else_statement?`
    If {
        /// Condition.
        condition: Expression,
        /// Then branch.
        then_statement: Box<Statement>,
        /// Optional else branch.
        else_statement: Option<Box<Statement>>,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// `switch (expression) statement`
    Switch {
        /// Switch expression.
        expression: Expression,
        /// Switch body.
        statement: Box<Statement>,
        /// Source span.
        span: core::range::Range<usize>,
    },
}

impl SelectionStatement {
    /// Returns the source span of the selection statement.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::If { span, .. } => span.clone(),
            Self::Switch { span, .. } => span.clone(),
        }
    }
}

/// Iteration statement.
#[derive(Debug, Clone, PartialEq)]
pub enum IterationStatement {
    /// `while (condition) statement`
    While {
        /// Condition.
        condition: Expression,
        /// Loop body.
        statement: Box<Statement>,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// `do statement while (condition);`
    DoWhile {
        /// Loop body.
        statement: Box<Statement>,
        /// Condition.
        condition: Expression,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// `for (init; condition; update) statement`
    For {
        /// Optional initializer expression.
        init: Option<Expression>,
        /// Optional condition expression.
        condition: Option<Expression>,
        /// Optional update expression.
        update: Option<Expression>,
        /// Loop body.
        statement: Box<Statement>,
        /// Source span.
        span: core::range::Range<usize>,
    },
}

impl IterationStatement {
    /// Returns the source span of the iteration statement.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::While { span, .. } => span.clone(),
            Self::DoWhile { span, .. } => span.clone(),
            Self::For { span, .. } => span.clone(),
        }
    }
}

/// Jump statement.
#[derive(Debug, Clone, PartialEq)]
pub enum JumpStatement {
    /// `goto identifier;`
    Goto(String, core::range::Range<usize>),
    /// `continue;`
    Continue(core::range::Range<usize>),
    /// `break;`
    Break(core::range::Range<usize>),
    /// `return expression?;`
    Return(Option<Expression>, core::range::Range<usize>),
}

impl JumpStatement {
    /// Returns the source span of the jump statement.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Goto(_, span) => span.clone(),
            Self::Continue(span) => span.clone(),
            Self::Break(span) => span.clone(),
            Self::Return(_, span) => span.clone(),
        }
    }
}

/// Expression.
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    /// Expression kind.
    pub kind: Box<ExpressionKind>,
    /// Source span.
    pub span: core::range::Range<usize>,
}

/// Expression kind.
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    /// Identifier.
    Identifier(String, core::range::Range<usize>),
    /// Constant (literal).
    Constant(Constant, core::range::Range<usize>),
    /// String literal.
    StringLiteral(String, core::range::Range<usize>),
    /// Array subscript `array[index]`.
    ArraySubscript {
        /// The array expression.
        array: Box<Expression>,
        /// The index expression.
        index: Box<Expression>,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// Function call `function(arguments)`.
    FunctionCall {
        /// The function expression.
        function: Box<Expression>,
        /// List of arguments.
        arguments: Vec<Expression>,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// Member access `object.member` or `object->member`.
    MemberAccess {
        /// The object expression.
        object: Box<Expression>,
        /// Member name.
        member: String,
        /// Whether it's a pointer access (`->`).
        is_pointer: bool,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// Postfix increment or decrement (`++`, `--`).
    PostfixIncDec {
        /// The operand.
        operand: Box<Expression>,
        /// Whether it's an increment.
        is_increment: bool,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// Prefix increment or decrement (`++`, `--`).
    PrefixIncDec {
        /// The operand.
        operand: Box<Expression>,
        /// Whether it's an increment.
        is_increment: bool,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// Unary operation.
    Unary {
        /// Unary operator.
        operator: UnaryOperator,
        /// The operand.
        operand: Box<Expression>,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// Type cast `(type_name) expression`.
    Cast {
        /// Target type.
        type_name: Box<TypeName>,
        /// The expression to cast.
        expression: Box<Expression>,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// Binary operation.
    Binary {
        /// Left operand.
        left: Box<Expression>,
        /// Binary operator.
        operator: BinaryOperator,
        /// Right operand.
        right: Box<Expression>,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// Conditional expression `condition ? then_expr : else_expr`.
    Conditional {
        /// Condition.
        condition: Box<Expression>,
        /// Then expression.
        then_expr: Box<Expression>,
        /// Else expression.
        else_expr: Box<Expression>,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// Assignment expression.
    Assignment {
        /// Left side of assignment.
        left: Box<Expression>,
        /// Assignment operator.
        operator: AssignmentOperator,
        /// Right side of assignment.
        right: Box<Expression>,
        /// Source span.
        span: core::range::Range<usize>,
    },
    /// Comma expression `expr1, expr2, ...`.
    Comma {
        /// List of expressions.
        expressions: Vec<Expression>,
        /// Source span.
        span: core::range::Range<usize>,
    },
}

impl ExpressionKind {
    /// Returns the source span of the expression kind.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Identifier(_, span) => span.clone(),
            Self::Constant(_, span) => span.clone(),
            Self::StringLiteral(_, span) => span.clone(),
            Self::ArraySubscript { span, .. } => span.clone(),
            Self::FunctionCall { span, .. } => span.clone(),
            Self::MemberAccess { span, .. } => span.clone(),
            Self::PostfixIncDec { span, .. } => span.clone(),
            Self::PrefixIncDec { span, .. } => span.clone(),
            Self::Unary { span, .. } => span.clone(),
            Self::Cast { span, .. } => span.clone(),
            Self::Binary { span, .. } => span.clone(),
            Self::Conditional { span, .. } => span.clone(),
            Self::Assignment { span, .. } => span.clone(),
            Self::Comma { span, .. } => span.clone(),
        }
    }
}

/// Constant (literal).
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    /// Integer constant.
    Integer(i64, core::range::Range<usize>),
    /// Floating-point constant.
    Float(f64, core::range::Range<usize>),
    /// Character constant.
    Character(char, core::range::Range<usize>),
}

impl Constant {
    /// Returns the source span of the constant.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Integer(_, span) => span.clone(),
            Self::Float(_, span) => span.clone(),
            Self::Character(_, span) => span.clone(),
        }
    }
}

/// Unary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    /// `&`
    AddressOf,
    /// `*`
    Indirection,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `~`
    BitNot,
    /// `!`
    LogicalNot,
    /// `sizeof`
    Sizeof,
}

/// Binary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    /// `*`
    Multiply,
    /// `/`
    Divide,
    /// `%`
    Modulo,
    /// `+`
    Add,
    /// `-`
    Subtract,
    /// `<<`
    ShiftLeft,
    /// `>>`
    ShiftRight,
    /// `<`
    Less,
    /// `>`
    Greater,
    /// `<=`
    LessEqual,
    /// `>=`
    GreaterEqual,
    /// `==`
    Equal,
    /// `!=`
    NotEqual,
    /// `&`
    BitAnd,
    /// `^`
    BitXor,
    /// `|`
    BitOr,
    /// `&&`
    LogicalAnd,
    /// `||`
    LogicalOr,
}

/// Assignment operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentOperator {
    /// `=`
    Assign,
    /// `*=`
    MulAssign,
    /// `/=`
    DivAssign,
    /// `%=`
    ModAssign,
    /// `+=`
    AddAssign,
    /// `-=`
    SubAssign,
    /// `<<=`
    ShlAssign,
    /// `>>=`
    ShrAssign,
    /// `&=`
    AndAssign,
    /// `^=`
    XorAssign,
    /// `|=`
    OrAssign,
}

/// Type name.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeName {
    /// List of specifiers and qualifiers.
    pub specifier_qualifiers: Vec<SpecifierQualifier>,
    /// Optional abstract declarator.
    pub abstract_declarator: Option<Box<AbstractDeclarator>>,
    /// Source span.
    pub span: core::range::Range<usize>,
}

impl TypeName {
    /// Returns the source span of the type name.
    pub fn span(&self) -> core::range::Range<usize> {
        self.span.clone()
    }
}

impl CRoot {
    /// 创建新的 AST
    pub fn new(translation_unit: TranslationUnit, span: core::range::Range<usize>) -> Self {
        Self { translation_unit, span }
    }
}

impl TranslationUnit {
    /// 创建新的翻译单元
    pub fn new(external_declarations: Vec<ExternalDeclaration>, span: core::range::Range<usize>) -> Self {
        Self { external_declarations, span }
    }
}
