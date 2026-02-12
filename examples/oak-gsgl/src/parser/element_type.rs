use oak_core::{ElementType, UniversalElementRole};
use std::fmt;

/// Element types for the GSGL (Game Shader Graphics Language) language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GsglElementType {
    /// Root node of the AST.
    Root,
    /// A source file.
    SourceFile,
    /// A function definition.
    FunctionDefinition,
    /// A variable declaration.
    VariableDeclaration,
    /// A struct definition.
    StructDefinition,
    /// A block of code.
    Block,
    /// An `if` statement.
    IfStatement,
    /// A `for` statement.
    ForStatement,
    /// A `while` statement.
    WhileStatement,
    /// A `return` statement.
    ReturnStatement,

    /// `shader` keyword.
    Shader,
    /// `vertex` keyword.
    Vertex,
    /// `fragment` keyword.
    Fragment,
    /// `geometry` keyword.
    Geometry,
    /// `compute` keyword.
    Compute,
    /// `uniform` keyword.
    Uniform,
    /// `attribute` keyword.
    Attribute,
    /// `varying` keyword.
    Varying,
    /// `in` keyword.
    In,
    /// `out` keyword.
    Out,
    /// `inout` keyword.
    Inout,
    /// `const` keyword.
    Const,
    /// `struct` keyword.
    Struct,
    /// `if` keyword.
    If,
    /// `else` keyword.
    Else,
    /// `for` keyword.
    For,
    /// `while` keyword.
    While,
    /// `do` keyword.
    Do,
    /// `break` keyword.
    Break,
    /// `continue` keyword.
    Continue,
    /// `return` keyword.
    Return,
    /// `discard` keyword.
    Discard,
    /// `true` keyword.
    True,
    /// `false` keyword.
    False,

    /// `float` type.
    Float,
    /// `int` type.
    Int,
    /// `bool` type.
    Bool,
    /// `vec2` type.
    Vec2,
    /// `vec3` type.
    Vec3,
    /// `vec4` type.
    Vec4,
    /// `mat2` type.
    Mat2,
    /// `mat3` type.
    Mat3,
    /// `mat4` type.
    Mat4,
    /// `sampler2D` type.
    Sampler2D,
    /// `samplerCube` type.
    SamplerCube,
    /// `void` type.
    Void,

    /// An identifier.
    Identifier,
    /// A number literal.
    Number,
    /// A string literal.
    String,

    /// `+`.
    Plus,
    /// `-`.
    Minus,
    /// `*`.
    Star,
    /// `/`.
    Slash,
    /// `%`.
    Percent,
    /// `=`.
    Assign,
    /// `+=`.
    PlusAssign,
    /// `-=`.
    MinusAssign,
    /// `*=`.
    StarAssign,
    /// `/=`.
    SlashAssign,

    /// `==`.
    Eq,
    /// `!=`.
    Ne,
    /// `<`.
    Lt,
    /// `<=`.
    Le,
    /// `>`.
    Gt,
    /// `>=`.
    Ge,

    /// `&&`.
    And,
    /// `||`.
    Or,
    /// `!`.
    Not,

    /// `&`.
    BitAnd,
    /// `|`.
    BitOr,
    /// `^`.
    BitXor,
    /// `~`.
    BitNot,
    /// `<<`.
    LeftShift,
    /// `>>`.
    RightShift,

    /// `(`.
    LeftParen,
    /// `)`.
    RightParen,
    /// `{`.
    LeftBrace,
    /// `}`.
    RightBrace,
    /// `[`.
    LeftBracket,
    /// `]`.
    RightBracket,
    /// `;`.
    Semicolon,
    /// `,`.
    Comma,
    /// `.`.
    Dot,
    /// `:`.
    Colon,
    /// `?`.
    Question,
    /// `#`.
    Hash,
    /// `@`.
    At,

    /// Preprocessor directive.
    Preprocessor,
    /// `#include`.
    Include,
    /// `#define`.
    Define,
    /// `#ifdef`.
    Ifdef,
    /// `#ifndef`.
    Ifndef,
    /// `#endif`.
    Endif,
    /// `#version`.
    Version,

    /// `sin` function.
    Sin,
    /// `cos` function.
    Cos,
    /// `tan` function.
    Tan,
    /// `sqrt` function.
    Sqrt,
    /// `pow` function.
    Pow,
    /// `exp` function.
    Exp,
    /// `log` function.
    Log,
    /// `abs` function.
    Abs,
    /// `sign` function.
    Sign,
    /// `floor` function.
    Floor,
    /// `ceil` function.
    Ceil,
    /// `fract` function.
    Fract,
    /// `mod` function.
    Mod,
    /// `min` function.
    Min,
    /// `max` function.
    Max,
    /// `clamp` function.
    Clamp,
    /// `mix` function.
    Mix,
    /// `step` function.
    Step,
    /// `smoothstep` function.
    Smoothstep,
    /// `length` function.
    Length,
    /// `distance` function.
    Distance,
    /// `dot` function.
    DotProduct,
    /// `cross` function.
    Cross,
    /// `normalize` function.
    Normalize,
    /// `faceforward` function.
    Faceforward,
    /// `reflect` function.
    Reflect,
    /// `refract` function.
    Refract,

    /// Whitespace.
    Whitespace,
    /// A comment.
    Comment,
    /// A newline.
    Newline,
    /// End of stream.
    Eof,
    /// An error token.
    Error,

    /// A function declaration.
    FunctionDecl,
    /// A variable declaration.
    VariableDecl,
    /// A struct declaration.
    StructDecl,
    /// A statement.
    Statement,
    /// An expression.
    Expression,
    /// A parameter.
    Parameter,
    /// An argument.
    Argument,
    /// A field access.
    FieldAccess,
    /// An array access.
    ArrayAccess,
    /// A function call.
    FunctionCall,
    /// A binary expression.
    BinaryExpr,
    /// A unary expression.
    UnaryExpr,
    /// An assignment expression.
    AssignmentExpr,
    /// A conditional expression.
    ConditionalExpr,
    /// A literal.
    Literal,
}

impl GsglElementType {
    /// Checks if the kind is a keyword.
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            Self::Shader
                | Self::Vertex
                | Self::Fragment
                | Self::Geometry
                | Self::Compute
                | Self::Uniform
                | Self::Attribute
                | Self::Varying
                | Self::In
                | Self::Out
                | Self::Inout
                | Self::Const
                | Self::Struct
                | Self::If
                | Self::Else
                | Self::For
                | Self::While
                | Self::Do
                | Self::Break
                | Self::Continue
                | Self::Return
                | Self::Discard
                | Self::True
                | Self::False
        )
    }

    /// Checks if the kind is a data type.
    pub fn is_type(self) -> bool {
        matches!(self, Self::Float | Self::Int | Self::Bool | Self::Vec2 | Self::Vec3 | Self::Vec4 | Self::Mat2 | Self::Mat3 | Self::Mat4 | Self::Sampler2D | Self::SamplerCube | Self::Void)
    }

    /// Checks if the kind is an operator.
    pub fn is_operator(self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::Assign
                | Self::PlusAssign
                | Self::MinusAssign
                | Self::StarAssign
                | Self::SlashAssign
                | Self::Eq
                | Self::Ne
                | Self::Lt
                | Self::Le
                | Self::Gt
                | Self::Ge
                | Self::And
                | Self::Or
                | Self::Not
                | Self::BitAnd
                | Self::BitOr
                | Self::BitXor
                | Self::BitNot
                | Self::LeftShift
                | Self::RightShift
        )
    }
}

impl fmt::Display for GsglElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Root => "ROOT",
            Self::SourceFile => "SOURCE_FILE",
            Self::Shader => "shader",
            Self::Vertex => "vertex",
            Self::Fragment => "fragment",
            Self::Geometry => "geometry",
            Self::Compute => "compute",
            Self::Uniform => "uniform",
            Self::Attribute => "attribute",
            Self::Varying => "varying",
            Self::In => "in",
            Self::Out => "out",
            Self::Inout => "inout",
            Self::Const => "const",
            Self::Struct => "struct",
            Self::If => "if",
            Self::Else => "else",
            Self::For => "for",
            Self::While => "while",
            Self::Do => "do",
            Self::Break => "break",
            Self::Continue => "continue",
            Self::Return => "return",
            Self::Discard => "discard",
            Self::True => "true",
            Self::False => "false",
            Self::Float => "float",
            Self::Int => "int",
            Self::Bool => "bool",
            Self::Vec2 => "vec2",
            Self::Vec3 => "vec3",
            Self::Vec4 => "vec4",
            Self::Mat2 => "mat2",
            Self::Mat3 => "mat3",
            Self::Mat4 => "mat4",
            Self::Sampler2D => "sampler2D",
            Self::SamplerCube => "samplerCube",
            Self::Void => "void",
            Self::Identifier => "IDENTIFIER",
            Self::Number => "NUMBER",
            Self::String => "STRING",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Star => "*",
            Self::Slash => "/",
            Self::Percent => "%",
            Self::Assign => "=",
            Self::PlusAssign => "+=",
            Self::MinusAssign => "-=",
            Self::StarAssign => "*=",
            Self::SlashAssign => "/=",
            Self::Eq => "==",
            Self::Ne => "!=",
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::And => "&&",
            Self::Or => "||",
            Self::Not => "!",
            Self::BitAnd => "&",
            Self::BitOr => "|",
            Self::BitXor => "^",
            Self::BitNot => "~",
            Self::LeftShift => "<<",
            Self::RightShift => ">>",
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
            Self::LeftBracket => "[",
            Self::RightBracket => "]",
            Self::Semicolon => ";",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::Colon => ":",
            Self::Question => "?",
            Self::Hash => "#",
            Self::At => "@",
            Self::Preprocessor => "PREPROCESSOR",
            Self::Include => "#include",
            Self::Define => "#define",
            Self::Ifdef => "#ifdef",
            Self::Ifndef => "#ifndef",
            Self::Endif => "#endif",
            Self::Version => "#version",
            Self::Sin => "sin",
            Self::Cos => "cos",
            Self::Tan => "tan",
            Self::Sqrt => "sqrt",
            Self::Pow => "pow",
            Self::Abs => "abs",
            Self::Min => "min",
            Self::Max => "max",
            Self::Clamp => "clamp",
            Self::Mix => "mix",
            Self::Step => "step",
            Self::Smoothstep => "smoothstep",
            Self::Length => "length",
            Self::Distance => "distance",
            Self::DotProduct => "dot",
            Self::Whitespace => "WHITESPACE",
            Self::Newline => "NEWLINE",
            Self::Comment => "COMMENT",
            Self::Error => "ERROR",
            Self::Eof => "EOF",
            Self::FunctionDecl => "FUNCTION_DECL",
            Self::VariableDecl => "VARIABLE_DECL",
            Self::StructDecl => "STRUCT_DECL",
            Self::Statement => "STATEMENT",
            Self::Expression => "EXPRESSION",
            Self::Parameter => "PARAMETER",
            Self::Argument => "ARGUMENT",
            Self::FieldAccess => "FIELD_ACCESS",
            Self::ArrayAccess => "ARRAY_ACCESS",
            Self::FunctionCall => "FUNCTION_CALL",
            Self::BinaryExpr => "BINARY_EXPR",
            Self::UnaryExpr => "UNARY_EXPR",
            Self::AssignmentExpr => "ASSIGNMENT_EXPR",
            Self::ConditionalExpr => "CONDITIONAL_EXPR",
            Self::Literal => "LITERAL",
            _ => "UNKNOWN",
        };
        write!(f, "{}", name)
    }
}

impl ElementType for GsglElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::SourceFile => UniversalElementRole::Root,
            Self::FunctionDefinition | Self::FunctionDecl => UniversalElementRole::Definition,
            Self::VariableDeclaration | Self::VariableDecl => UniversalElementRole::Definition,
            Self::StructDefinition | Self::StructDecl => UniversalElementRole::Typing,
            Self::Block => UniversalElementRole::Container,
            Self::IfStatement => UniversalElementRole::Statement,
            Self::ForStatement | Self::WhileStatement => UniversalElementRole::Statement,
            Self::ReturnStatement => UniversalElementRole::Statement,
            Self::Expression | Self::BinaryExpr | Self::UnaryExpr | Self::AssignmentExpr | Self::ConditionalExpr => UniversalElementRole::Expression,
            Self::Statement => UniversalElementRole::Statement,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::GsglTokenType> for GsglElementType {
    fn from(token: crate::lexer::token_type::GsglTokenType) -> Self {
        use crate::lexer::token_type::GsglTokenType as T;
        match token {
            T::Root => Self::Root,
            T::SourceFile => Self::SourceFile,
            T::FunctionDefinition => Self::FunctionDefinition,
            T::VariableDeclaration => Self::VariableDeclaration,
            T::StructDefinition => Self::StructDefinition,
            T::Block => Self::Block,
            T::IfStatement => Self::IfStatement,
            T::ForStatement => Self::ForStatement,
            T::WhileStatement => Self::WhileStatement,
            T::ReturnStatement => Self::ReturnStatement,
            T::Shader => Self::Shader,
            T::Vertex => Self::Vertex,
            T::Fragment => Self::Fragment,
            T::Geometry => Self::Geometry,
            T::Compute => Self::Compute,
            T::Uniform => Self::Uniform,
            T::Attribute => Self::Attribute,
            T::Varying => Self::Varying,
            T::In => Self::In,
            T::Out => Self::Out,
            T::Inout => Self::Inout,
            T::Const => Self::Const,
            T::Struct => Self::Struct,
            T::If => Self::If,
            T::Else => Self::Else,
            T::For => Self::For,
            T::While => Self::While,
            T::Do => Self::Do,
            T::Break => Self::Break,
            T::Continue => Self::Continue,
            T::Return => Self::Return,
            T::Discard => Self::Discard,
            T::True => Self::True,
            T::False => Self::False,
            T::Float => Self::Float,
            T::Int => Self::Int,
            T::Bool => Self::Bool,
            T::Vec2 => Self::Vec2,
            T::Vec3 => Self::Vec3,
            T::Vec4 => Self::Vec4,
            T::Mat2 => Self::Mat2,
            T::Mat3 => Self::Mat3,
            T::Mat4 => Self::Mat4,
            T::Sampler2D => Self::Sampler2D,
            T::SamplerCube => Self::SamplerCube,
            T::Void => Self::Void,
            T::Identifier => Self::Identifier,
            T::Number => Self::Number,
            T::String => Self::String,
            T::Plus => Self::Plus,
            T::Minus => Self::Minus,
            T::Star => Self::Star,
            T::Slash => Self::Slash,
            T::Percent => Self::Percent,
            T::Assign => Self::Assign,
            T::PlusAssign => Self::PlusAssign,
            T::MinusAssign => Self::MinusAssign,
            T::StarAssign => Self::StarAssign,
            T::SlashAssign => Self::SlashAssign,
            T::Eq => Self::Eq,
            T::Ne => Self::Ne,
            T::Lt => Self::Lt,
            T::Le => Self::Le,
            T::Gt => Self::Gt,
            T::Ge => Self::Ge,
            T::And => Self::And,
            T::Or => Self::Or,
            T::Not => Self::Not,
            T::BitAnd => Self::BitAnd,
            T::BitOr => Self::BitOr,
            T::BitXor => Self::BitXor,
            T::BitNot => Self::BitNot,
            T::LeftShift => Self::LeftShift,
            T::RightShift => Self::RightShift,
            T::LeftParen => Self::LeftParen,
            T::RightParen => Self::RightParen,
            T::LeftBrace => Self::LeftBrace,
            T::RightBrace => Self::RightBrace,
            T::LeftBracket => Self::LeftBracket,
            T::RightBracket => Self::RightBracket,
            T::Semicolon => Self::Semicolon,
            T::Comma => Self::Comma,
            T::Dot => Self::Dot,
            T::Colon => Self::Colon,
            T::Question => Self::Question,
            T::Hash => Self::Hash,
            T::At => Self::At,
            T::Preprocessor => Self::Preprocessor,
            T::Include => Self::Include,
            T::Define => Self::Define,
            T::Ifdef => Self::Ifdef,
            T::Ifndef => Self::Ifndef,
            T::Endif => Self::Endif,
            T::Version => Self::Version,
            T::Sin => Self::Sin,
            T::Cos => Self::Cos,
            T::Tan => Self::Tan,
            T::Sqrt => Self::Sqrt,
            T::Pow => Self::Pow,
            T::Exp => Self::Exp,
            T::Log => Self::Log,
            T::Abs => Self::Abs,
            T::Sign => Self::Sign,
            T::Floor => Self::Floor,
            T::Ceil => Self::Ceil,
            T::Fract => Self::Fract,
            T::Mod => Self::Mod,
            T::Min => Self::Min,
            T::Max => Self::Max,
            T::Clamp => Self::Clamp,
            T::Mix => Self::Mix,
            T::Step => Self::Step,
            T::Smoothstep => Self::Smoothstep,
            T::Length => Self::Length,
            T::Distance => Self::Distance,
            T::DotProduct => Self::DotProduct,
            T::Cross => Self::Cross,
            T::Normalize => Self::Normalize,
            T::Faceforward => Self::Faceforward,
            T::Reflect => Self::Reflect,
            T::Refract => Self::Refract,
            T::Whitespace => Self::Whitespace,
            T::Comment => Self::Comment,
            T::Newline => Self::Newline,
            T::Eof => Self::Eof,
            T::Error => Self::Error,
        }
    }
}
