use super::*;

/// Represents a canonical C type.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// A primitive type (int, char, etc.) or a struct/union/enum.
    Base(TypeSpecifier),
    /// A pointer type.
    Pointer(Box<Type>),
    /// An array type.
    Array {
        /// The type of elements in the array.
        element_type: Box<Type>,
        /// The size of the array, if specified.
        size: Option<Box<Expression>>,
    },
    /// A function type.
    Function {
        /// The return type of the function.
        return_type: Box<Type>,
        /// The types of the parameters.
        parameters: Vec<Type>,
        /// Whether the function is variadic.
        variadic: bool,
    },
}

/// External declaration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    /// Declaration specifiers (e.g., return type, storage class).
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    /// The declarator for the function.
    pub declarator: Declarator,
    /// The body of the function.
    pub compound_statement: CompoundStatement,
    /// The source span of the function definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Declaration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    /// Declaration specifiers.
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    /// List of declarators being initialized.
    pub init_declarators: Vec<InitDeclarator>,
    /// The source span of the declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Declaration specifier.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum StorageClassSpecifier {
    /// `typedef`
    Typedef {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `extern`
    Extern {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `static`
    Static {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `auto`
    Auto {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `register`
    Register {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum TypeSpecifier {
    /// `void`
    Void {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `char`
    Char {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `short`
    Short {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `int`
    Int {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `long`
    Long {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `float`
    Float {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `double`
    Double {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `signed`
    Signed {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `unsigned`
    Unsigned {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `_Bool`
    Bool {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `_Complex`
    Complex {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `_Imaginary`
    Imaginary {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// A struct or union specifier.
    StructOrUnion(StructOrUnionSpecifier),
    /// An enum specifier.
    Enum(EnumSpecifier),
    /// A typedef name.
    TypedefName(String, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] core::range::Range<usize>),
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum TypeQualifier {
    /// `const`
    Const {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `restrict`
    Restrict {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `volatile`
    Volatile {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionSpecifier {
    /// `inline`
    Inline {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct StructOrUnionSpecifier {
    /// Whether it's a struct or union.
    pub kind: StructOrUnion,
    /// Optional tag identifier.
    pub identifier: Option<String>,
    /// List of struct declarations.
    pub struct_declarations: Vec<StructDeclaration>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Struct or union keyword.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum StructOrUnion {
    /// `struct`
    Struct {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `union`
    Union {
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
}

impl StructOrUnion {
    /// Returns the source span of the struct or union keyword.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Struct { span } => span.clone(),
            Self::Union { span } => span.clone(),
        }
    }
}

/// Struct declaration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    /// List of specifiers and qualifiers.
    pub specifier_qualifier_list: Vec<SpecifierQualifier>,
    /// List of struct declarators.
    pub struct_declarator_list: Vec<StructDeclarator>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Specifier or qualifier.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum SpecifierQualifier {
    /// Type specifier.
    TypeSpecifier(TypeSpecifier),
    /// Type qualifier.
    TypeQualifier(TypeQualifier),
}

impl SpecifierQualifier {
    /// Returns the source span of the specifier or qualifier.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::TypeSpecifier(n) => n.span(),
            Self::TypeQualifier(n) => n.span(),
        }
    }
}

/// Struct declarator.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclarator {
    /// The declarator.
    pub declarator: Option<Declarator>,
    /// Optional bit-field width expression.
    pub constant_expression: Option<Expression>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Enum specifier.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct EnumSpecifier {
    /// Optional tag identifier.
    pub identifier: Option<String>,
    /// List of enumerators.
    pub enumerators: Vec<Enumerator>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Enumerator.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Enumerator {
    /// Enumerator identifier.
    pub identifier: String,
    /// Optional constant expression value.
    pub constant_expression: Option<Expression>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Init declarator.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct InitDeclarator {
    /// The declarator.
    pub declarator: Declarator,
    /// Optional initializer.
    pub initializer: Option<Initializer>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Declarator.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Declarator {
    /// Optional pointer prefix.
    pub pointer: Option<Pointer>,
    /// Direct declarator.
    pub direct_declarator: DirectDeclarator,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Pointer.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Pointer {
    /// List of type qualifiers for this pointer level.
    pub type_qualifiers: Vec<TypeQualifier>,
    /// Optional nested pointer (for `**`, etc.).
    pub pointer: Option<Box<Pointer>>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Direct declarator.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum DirectDeclarator {
    /// Identifier.
    Identifier(String, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] core::range::Range<usize>),
    /// Parenthesized declarator.
    Declarator(Box<Declarator>, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] core::range::Range<usize>),
    /// Array declarator.
    Array {
        /// The declarator being declared as an array.
        direct_declarator: Box<DirectDeclarator>,
        /// Type qualifiers inside `[]`.
        type_qualifiers: Vec<TypeQualifier>,
        /// Optional assignment expression for size.
        assignment_expression: Option<Box<Expression>>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// Function declarator.
    Function {
        /// The declarator being declared as a function.
        direct_declarator: Box<DirectDeclarator>,
        /// Parameter list.
        parameter_list: ParameterList,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterList {
    /// List of parameter declarations.
    pub parameter_declarations: Vec<ParameterDeclaration>,
    /// Whether the function is variadic (ends with `...`).
    pub variadic: bool,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Parameter declaration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterDeclaration {
    /// Declaration specifiers.
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    /// Optional declarator.
    pub declarator: Option<Declarator>,
    /// Optional abstract declarator.
    pub abstract_declarator: Option<AbstractDeclarator>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Abstract declarator.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct AbstractDeclarator {
    /// Optional pointer prefix.
    pub pointer: Option<Pointer>,
    /// Direct abstract declarator.
    pub direct_abstract_declarator: Option<Box<DirectAbstractDeclarator>>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Direct abstract declarator.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// Function abstract declarator.
    Function {
        /// Optional direct abstract declarator.
        declarator: Option<Box<DirectAbstractDeclarator>>,
        /// Parameter list.
        parameter_list: Option<ParameterList>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Initializer {
    /// Assignment expression.
    AssignmentExpression(Expression),
    /// Initializer list `{ ... }`.
    InitializerList(Vec<Initializer>, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] core::range::Range<usize>),
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
