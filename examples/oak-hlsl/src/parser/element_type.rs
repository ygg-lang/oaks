use oak_core::{ElementType, Parser, UniversalElementRole};

/// Element types for the HLSL (High-Level Shading Language) parser.
///
/// This enum represents all possible element types in HLSL,
/// including data types, texture types, control flow keywords, operators, and AST nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum HlslElementType {
    /// Whitespace token.
    Whitespace,
    /// Newline token.
    Newline,

    /// Comment token.
    Comment,

    /// String literal token.
    StringLiteral,
    /// Number literal token.
    NumberLiteral,
    /// Boolean literal token.
    BooleanLiteral,

    /// Identifier token.
    Identifier,

    /// Boolean data type `bool`.
    Bool,
    /// Integer data type `int`.
    Int,
    /// Unsigned integer data type `uint`.
    Uint,
    /// Half-precision floating-point data type `half`.
    Half,
    /// Single-precision floating-point data type `float`.
    Float,
    /// Double-precision floating-point data type `double`.
    Double,
    /// Minimum 16-bit float data type `min16float`.
    Min16float,
    /// Minimum 10-bit float data type `min10float`.
    Min10float,
    /// Minimum 16-bit integer data type `min16int`.
    Min16int,
    /// Minimum 12-bit integer data type `min12int`.
    Min12int,
    /// Minimum 16-bit unsigned integer data type `min16uint`.
    Min16uint,

    /// 2-component boolean vector `bool2`.
    Bool2,
    /// 3-component boolean vector `bool3`.
    Bool3,
    /// 4-component boolean vector `bool4`.
    Bool4,
    /// 2-component integer vector `int2`.
    Int2,
    /// 3-component integer vector `int3`.
    Int3,
    /// 4-component integer vector `int4`.
    Int4,
    /// 2-component unsigned integer vector `uint2`.
    Uint2,
    /// 3-component unsigned integer vector `uint3`.
    Uint3,
    /// 4-component unsigned integer vector `uint4`.
    Uint4,
    /// 2-component half-precision vector `half2`.
    Half2,
    /// 3-component half-precision vector `half3`.
    Half3,
    /// 4-component half-precision vector `half4`.
    Half4,
    /// 2-component float vector `float2`.
    Float2,
    /// 3-component float vector `float3`.
    Float3,
    /// 4-component float vector `float4`.
    Float4,
    /// 2-component double vector `double2`.
    Double2,
    /// 3-component double vector `double3`.
    Double3,
    /// 4-component double vector `double4`.
    Double4,

    /// 2x2 float matrix `float2x2`.
    Float2x2,
    /// 2x3 float matrix `float2x3`.
    Float2x3,
    /// 2x4 float matrix `float2x4`.
    Float2x4,
    /// 3x2 float matrix `float3x2`.
    Float3x2,
    /// 3x3 float matrix `float3x3`.
    Float3x3,
    /// 3x4 float matrix `float3x4`.
    Float3x4,
    /// 4x2 float matrix `float4x2`.
    Float4x2,
    /// 4x3 float matrix `float4x3`.
    Float4x3,
    /// 4x4 float matrix `float4x4`.
    Float4x4,
    /// 2x2 double matrix `double2x2`.
    Double2x2,
    /// 2x3 double matrix `double2x3`.
    Double2x3,
    /// 2x4 double matrix `double2x4`.
    Double2x4,
    /// 3x2 double matrix `double3x2`.
    Double3x2,
    /// 3x3 double matrix `double3x3`.
    Double3x3,
    /// 3x4 double matrix `double3x4`.
    Double3x4,
    /// 4x2 double matrix `double4x2`.
    Double4x2,
    /// 4x3 double matrix `double4x3`.
    Double4x3,
    /// 4x4 double matrix `double4x4`.
    Double4x4,

    /// 1D texture type `Texture1D`.
    Texture1D,
    /// 2D texture type `Texture2D`.
    Texture2D,
    /// 3D texture type `Texture3D`.
    Texture3D,
    /// Cubemap texture type `TextureCube`.
    TextureCube,
    /// 1D texture array type `Texture1DArray`.
    Texture1DArray,
    /// 2D texture array type `Texture2DArray`.
    Texture2DArray,
    /// Cubemap array type `TextureCubeArray`.
    TextureCubeArray,
    /// Multisampled 2D texture type `Texture2DMS`.
    Texture2DMS,
    /// Multisampled 2D texture array type `Texture2DMSArray`.
    Texture2DMSArray,

    /// Sampler type.
    Sampler,
    /// Sampler state type `SamplerState`.
    SamplerState,
    /// Comparison sampler state type `SamplerComparisonState`.
    SamplerComparisonState,

    /// Buffer type.
    Buffer,
    /// Structured buffer type `StructuredBuffer`.
    StructuredBuffer,
    /// Byte address buffer type `ByteAddressBuffer`.
    ByteAddressBuffer,
    /// Read-write buffer type `RWBuffer`.
    RWBuffer,
    /// Read-write structured buffer type `RWStructuredBuffer`.
    RWStructuredBuffer,
    /// Read-write byte address buffer type `RWByteAddressBuffer`.
    RWByteAddressBuffer,
    /// Append structured buffer type `AppendStructuredBuffer`.
    AppendStructuredBuffer,
    /// Consume structured buffer type `ConsumeStructuredBuffer`.
    ConsumeStructuredBuffer,

    /// If keyword `if`.
    If,
    /// Else keyword `else`.
    Else,
    /// For keyword `for`.
    For,
    /// While keyword `while`.
    While,
    /// Do keyword `do`.
    Do,
    /// Switch keyword `switch`.
    Switch,
    /// Case keyword `case`.
    Case,
    /// Default keyword `default`.
    Default,
    /// Break keyword `break`.
    Break,
    /// Continue keyword `continue`.
    Continue,
    /// Return keyword `return`.
    Return,
    /// Discard keyword `discard`.
    Discard,

    /// Static modifier `static`.
    Static,
    /// Const modifier `const`.
    Const,
    /// Uniform modifier `uniform`.
    Uniform,
    /// Varying modifier `varying`.
    Varying,
    /// Input parameter modifier `in`.
    In,
    /// Output parameter modifier `out`.
    Out,
    /// Input-output parameter modifier `inout`.
    Inout,
    /// Inline modifier `inline`.
    Inline,
    /// Extern modifier `extern`.
    Extern,
    /// Shared modifier `shared`.
    Shared,
    /// Group-shared modifier `groupshared`.
    Groupshared,
    /// Volatile modifier `volatile`.
    Volatile,
    /// Precise modifier `precise`.
    Precise,
    /// No interpolation modifier `nointerpolation`.
    Nointerpolation,
    /// Linear interpolation modifier `linear`.
    Linear,
    /// Centroid interpolation modifier `centroid`.
    Centroid,
    /// Sample interpolation modifier `sample`.
    Sample,
    /// No perspective modifier `noperspective`.
    Noperspective,
    /// Target modifier `target`.
    Target,

    /// Register semantic modifier `register`.
    Register,
    /// Pack offset modifier `packoffset`.
    Packoffset,

    /// Struct keyword `struct`.
    Struct,
    /// Constant buffer keyword `cbuffer`.
    Cbuffer,
    /// Texture buffer keyword `tbuffer`.
    Tbuffer,
    /// Technique keyword `technique`.
    Technique,
    /// Pass keyword `pass`.
    Pass,
    /// Interface keyword `interface`.
    Interface,
    /// Class keyword `class`.
    Class,
    /// Namespace keyword `namespace`.
    Namespace,
    /// Typedef keyword `typedef`.
    Typedef,
    /// Template keyword `template`.
    Template,
    /// Typename keyword `typename`.
    Typename,
    /// Using keyword `using`.
    Using,
    /// Sizeof keyword `sizeof`.
    Sizeof,
    /// Undef keyword `undef`.
    Undef,

    /// Include preprocessor directive `#include`.
    Include,
    /// Define preprocessor directive `#define`.
    Define,
    /// If preprocessor directive `#if`.
    If_,
    /// Ifdef preprocessor directive `#ifdef`.
    Ifdef,
    /// Ifndef preprocessor directive `#ifndef`.
    Ifndef,
    /// Else preprocessor directive `#else`.
    Else_,
    /// Elif preprocessor directive `#elif`.
    Elif,
    /// Endif preprocessor directive `#endif`.
    Endif,
    /// Line preprocessor directive `#line`.
    Line,
    /// Pragma preprocessor directive `#pragma`.
    Pragma,

    /// Plus operator `+`.
    Plus,
    /// Minus operator `-`.
    Minus,
    /// Multiply operator `*`.
    Multiply,
    /// Divide operator `/`.
    Divide,
    /// Modulo operator `%`.
    Modulo,
    /// Assignment operator `=`.
    Assign,
    /// Plus assignment operator `+=`.
    PlusAssign,
    /// Minus assignment operator `-=`.
    MinusAssign,
    /// Multiply assignment operator `*=`.
    MultiplyAssign,
    /// Divide assignment operator `/=`.
    DivideAssign,
    /// Modulo assignment operator `%=`.
    ModuloAssign,
    /// Equality operator `==`.
    Equal,
    /// Inequality operator `!=`.
    NotEqual,
    /// Less than operator `<`.
    Less,
    /// Greater than operator `>`.
    Greater,
    /// Less than or equal operator `<=`.
    LessEqual,
    /// Greater than or equal operator `>=`.
    GreaterEqual,
    /// Logical and operator `&&`.
    LogicalAnd,
    /// Logical or operator `||`.
    LogicalOr,
    /// Logical not operator `!`.
    LogicalNot,
    /// Bitwise and operator `&`.
    BitwiseAnd,
    /// Bitwise or operator `|`.
    BitwiseOr,
    /// Bitwise xor operator `^`.
    BitwiseXor,
    /// Bitwise not operator `~`.
    BitwiseNot,
    /// Left shift operator `<<`.
    LeftShift,
    /// Right shift operator `>>`.
    RightShift,
    /// Left shift assignment operator `<<=`.
    LeftShiftAssign,
    /// Right shift assignment operator `>>=`.
    RightShiftAssign,
    /// Bitwise and assignment operator `&=`.
    BitwiseAndAssign,
    /// Bitwise or assignment operator `|=`.
    BitwiseOrAssign,
    /// Bitwise xor assignment operator `^=`.
    BitwiseXorAssign,
    /// Increment operator `++`.
    Increment,
    /// Decrement operator `--`.
    Decrement,
    /// Dot operator `.`.
    Dot,
    /// Arrow operator `->`.
    Arrow,
    /// Conditional ternary operator `?:`.
    Conditional,

    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Semicolon `;`.
    Semicolon,
    /// Comma `,`.
    Comma,
    /// Colon `:`.
    Colon,
    /// Double colon `::`.
    DoubleColon,
    /// Question mark `?`.
    Question,
    /// Hash symbol `#`.
    Hash,
    /// At symbol `@`.
    At,
    /// Backslash symbol `\`.
    Backslash,

    /// End of file marker.
    Eof,
    /// Root node of the AST.
    Root,
    /// Function declaration node.
    FunctionDeclaration,
    /// Struct declaration node.
    StructDeclaration,
    /// Variable declaration node.
    VariableDeclaration,
    /// Parameter list node.
    ParameterList,
    /// Parameter node.
    Parameter,
    /// Code block node.
    Block,
    /// Statement node.
    Statement,
    /// Expression node.
    Expression,
    /// Error node.
    Error,
}

impl ElementType for HlslElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::HlslTokenType> for HlslElementType {
    fn from(token: crate::lexer::token_type::HlslTokenType) -> Self {
                match token {
            crate::lexer::token_type::HlslTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::HlslTokenType::Newline => Self::Newline,
            crate::lexer::token_type::HlslTokenType::Comment => Self::Comment,
            crate::lexer::token_type::HlslTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::HlslTokenType::NumberLiteral => Self::NumberLiteral,
            crate::lexer::token_type::HlslTokenType::BooleanLiteral => Self::BooleanLiteral,
            crate::lexer::token_type::HlslTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::HlslTokenType::Bool => Self::Bool,
            crate::lexer::token_type::HlslTokenType::Int => Self::Int,
            crate::lexer::token_type::HlslTokenType::Uint => Self::Uint,
            crate::lexer::token_type::HlslTokenType::Half => Self::Half,
            crate::lexer::token_type::HlslTokenType::Float => Self::Float,
            crate::lexer::token_type::HlslTokenType::Double => Self::Double,
            crate::lexer::token_type::HlslTokenType::Min16float => Self::Min16float,
            crate::lexer::token_type::HlslTokenType::Min10float => Self::Min10float,
            crate::lexer::token_type::HlslTokenType::Min16int => Self::Min16int,
            crate::lexer::token_type::HlslTokenType::Min12int => Self::Min12int,
            crate::lexer::token_type::HlslTokenType::Min16uint => Self::Min16uint,
            crate::lexer::token_type::HlslTokenType::Bool2 => Self::Bool2,
            crate::lexer::token_type::HlslTokenType::Bool3 => Self::Bool3,
            crate::lexer::token_type::HlslTokenType::Bool4 => Self::Bool4,
            crate::lexer::token_type::HlslTokenType::Int2 => Self::Int2,
            crate::lexer::token_type::HlslTokenType::Int3 => Self::Int3,
            crate::lexer::token_type::HlslTokenType::Int4 => Self::Int4,
            crate::lexer::token_type::HlslTokenType::Uint2 => Self::Uint2,
            crate::lexer::token_type::HlslTokenType::Uint3 => Self::Uint3,
            crate::lexer::token_type::HlslTokenType::Uint4 => Self::Uint4,
            crate::lexer::token_type::HlslTokenType::Half2 => Self::Half2,
            crate::lexer::token_type::HlslTokenType::Half3 => Self::Half3,
            crate::lexer::token_type::HlslTokenType::Half4 => Self::Half4,
            crate::lexer::token_type::HlslTokenType::Float2 => Self::Float2,
            crate::lexer::token_type::HlslTokenType::Float3 => Self::Float3,
            crate::lexer::token_type::HlslTokenType::Float4 => Self::Float4,
            crate::lexer::token_type::HlslTokenType::Double2 => Self::Double2,
            crate::lexer::token_type::HlslTokenType::Double3 => Self::Double3,
            crate::lexer::token_type::HlslTokenType::Double4 => Self::Double4,
            crate::lexer::token_type::HlslTokenType::Float2x2 => Self::Float2x2,
            crate::lexer::token_type::HlslTokenType::Float2x3 => Self::Float2x3,
            crate::lexer::token_type::HlslTokenType::Float2x4 => Self::Float2x4,
            crate::lexer::token_type::HlslTokenType::Float3x2 => Self::Float3x2,
            crate::lexer::token_type::HlslTokenType::Float3x3 => Self::Float3x3,
            crate::lexer::token_type::HlslTokenType::Float3x4 => Self::Float3x4,
            crate::lexer::token_type::HlslTokenType::Float4x2 => Self::Float4x2,
            crate::lexer::token_type::HlslTokenType::Float4x3 => Self::Float4x3,
            crate::lexer::token_type::HlslTokenType::Float4x4 => Self::Float4x4,
            crate::lexer::token_type::HlslTokenType::Double2x2 => Self::Double2x2,
            crate::lexer::token_type::HlslTokenType::Double2x3 => Self::Double2x3,
            crate::lexer::token_type::HlslTokenType::Double2x4 => Self::Double2x4,
            crate::lexer::token_type::HlslTokenType::Double3x2 => Self::Double3x2,
            crate::lexer::token_type::HlslTokenType::Double3x3 => Self::Double3x3,
            crate::lexer::token_type::HlslTokenType::Double3x4 => Self::Double3x4,
            crate::lexer::token_type::HlslTokenType::Double4x2 => Self::Double4x2,
            crate::lexer::token_type::HlslTokenType::Double4x3 => Self::Double4x3,
            crate::lexer::token_type::HlslTokenType::Double4x4 => Self::Double4x4,
            crate::lexer::token_type::HlslTokenType::Texture1D => Self::Texture1D,
            crate::lexer::token_type::HlslTokenType::Texture2D => Self::Texture2D,
            crate::lexer::token_type::HlslTokenType::Texture3D => Self::Texture3D,
            crate::lexer::token_type::HlslTokenType::TextureCube => Self::TextureCube,
            crate::lexer::token_type::HlslTokenType::Texture1DArray => Self::Texture1DArray,
            crate::lexer::token_type::HlslTokenType::Texture2DArray => Self::Texture2DArray,
            crate::lexer::token_type::HlslTokenType::TextureCubeArray => Self::TextureCubeArray,
            crate::lexer::token_type::HlslTokenType::Texture2DMS => Self::Texture2DMS,
            crate::lexer::token_type::HlslTokenType::Texture2DMSArray => Self::Texture2DMSArray,
            crate::lexer::token_type::HlslTokenType::Sampler => Self::Sampler,
            crate::lexer::token_type::HlslTokenType::SamplerState => Self::SamplerState,
            crate::lexer::token_type::HlslTokenType::SamplerComparisonState => Self::SamplerComparisonState,
            crate::lexer::token_type::HlslTokenType::Buffer => Self::Buffer,
            crate::lexer::token_type::HlslTokenType::StructuredBuffer => Self::StructuredBuffer,
            crate::lexer::token_type::HlslTokenType::ByteAddressBuffer => Self::ByteAddressBuffer,
            crate::lexer::token_type::HlslTokenType::RWBuffer => Self::RWBuffer,
            crate::lexer::token_type::HlslTokenType::RWStructuredBuffer => Self::RWStructuredBuffer,
            crate::lexer::token_type::HlslTokenType::RWByteAddressBuffer => Self::RWByteAddressBuffer,
            crate::lexer::token_type::HlslTokenType::AppendStructuredBuffer => Self::AppendStructuredBuffer,
            crate::lexer::token_type::HlslTokenType::ConsumeStructuredBuffer => Self::ConsumeStructuredBuffer,
            crate::lexer::token_type::HlslTokenType::If => Self::If,
            crate::lexer::token_type::HlslTokenType::Else => Self::Else,
            crate::lexer::token_type::HlslTokenType::For => Self::For,
            crate::lexer::token_type::HlslTokenType::While => Self::While,
            crate::lexer::token_type::HlslTokenType::Do => Self::Do,
            crate::lexer::token_type::HlslTokenType::Switch => Self::Switch,
            crate::lexer::token_type::HlslTokenType::Case => Self::Case,
            crate::lexer::token_type::HlslTokenType::Default => Self::Default,
            crate::lexer::token_type::HlslTokenType::Break => Self::Break,
            crate::lexer::token_type::HlslTokenType::Continue => Self::Continue,
            crate::lexer::token_type::HlslTokenType::Return => Self::Return,
            crate::lexer::token_type::HlslTokenType::Discard => Self::Discard,
            crate::lexer::token_type::HlslTokenType::Static => Self::Static,
            crate::lexer::token_type::HlslTokenType::Const => Self::Const,
            crate::lexer::token_type::HlslTokenType::Uniform => Self::Uniform,
            crate::lexer::token_type::HlslTokenType::Varying => Self::Varying,
            crate::lexer::token_type::HlslTokenType::In => Self::In,
            crate::lexer::token_type::HlslTokenType::Out => Self::Out,
            crate::lexer::token_type::HlslTokenType::Inout => Self::Inout,
            crate::lexer::token_type::HlslTokenType::Inline => Self::Inline,
            crate::lexer::token_type::HlslTokenType::Extern => Self::Extern,
            crate::lexer::token_type::HlslTokenType::Shared => Self::Shared,
            crate::lexer::token_type::HlslTokenType::Groupshared => Self::Groupshared,
            crate::lexer::token_type::HlslTokenType::Volatile => Self::Volatile,
            crate::lexer::token_type::HlslTokenType::Precise => Self::Precise,
            crate::lexer::token_type::HlslTokenType::Nointerpolation => Self::Nointerpolation,
            crate::lexer::token_type::HlslTokenType::Linear => Self::Linear,
            crate::lexer::token_type::HlslTokenType::Centroid => Self::Centroid,
            crate::lexer::token_type::HlslTokenType::Sample => Self::Sample,
            crate::lexer::token_type::HlslTokenType::Noperspective => Self::Noperspective,
            crate::lexer::token_type::HlslTokenType::Target => Self::Target,
            crate::lexer::token_type::HlslTokenType::Register => Self::Register,
            crate::lexer::token_type::HlslTokenType::Packoffset => Self::Packoffset,
            crate::lexer::token_type::HlslTokenType::Struct => Self::Struct,
            crate::lexer::token_type::HlslTokenType::Cbuffer => Self::Cbuffer,
            crate::lexer::token_type::HlslTokenType::Tbuffer => Self::Tbuffer,
            crate::lexer::token_type::HlslTokenType::Technique => Self::Technique,
            crate::lexer::token_type::HlslTokenType::Pass => Self::Pass,
            crate::lexer::token_type::HlslTokenType::Interface => Self::Interface,
            crate::lexer::token_type::HlslTokenType::Class => Self::Class,
            crate::lexer::token_type::HlslTokenType::Namespace => Self::Namespace,
            crate::lexer::token_type::HlslTokenType::Typedef => Self::Typedef,
            crate::lexer::token_type::HlslTokenType::Template => Self::Template,
            crate::lexer::token_type::HlslTokenType::Typename => Self::Typename,
            crate::lexer::token_type::HlslTokenType::Using => Self::Using,
            crate::lexer::token_type::HlslTokenType::Sizeof => Self::Sizeof,
            crate::lexer::token_type::HlslTokenType::Undef => Self::Undef,
            crate::lexer::token_type::HlslTokenType::Include => Self::Include,
            crate::lexer::token_type::HlslTokenType::Define => Self::Define,
            crate::lexer::token_type::HlslTokenType::If_ => Self::If_,
            crate::lexer::token_type::HlslTokenType::Ifdef => Self::Ifdef,
            crate::lexer::token_type::HlslTokenType::Ifndef => Self::Ifndef,
            crate::lexer::token_type::HlslTokenType::Else_ => Self::Else_,
            crate::lexer::token_type::HlslTokenType::Elif => Self::Elif,
            crate::lexer::token_type::HlslTokenType::Endif => Self::Endif,
            crate::lexer::token_type::HlslTokenType::Line => Self::Line,
            crate::lexer::token_type::HlslTokenType::Pragma => Self::Pragma,
            crate::lexer::token_type::HlslTokenType::Plus => Self::Plus,
            crate::lexer::token_type::HlslTokenType::Minus => Self::Minus,
            crate::lexer::token_type::HlslTokenType::Multiply => Self::Multiply,
            crate::lexer::token_type::HlslTokenType::Divide => Self::Divide,
            crate::lexer::token_type::HlslTokenType::Modulo => Self::Modulo,
            crate::lexer::token_type::HlslTokenType::Assign => Self::Assign,
            crate::lexer::token_type::HlslTokenType::PlusAssign => Self::PlusAssign,
            crate::lexer::token_type::HlslTokenType::MinusAssign => Self::MinusAssign,
            crate::lexer::token_type::HlslTokenType::MultiplyAssign => Self::MultiplyAssign,
            crate::lexer::token_type::HlslTokenType::DivideAssign => Self::DivideAssign,
            crate::lexer::token_type::HlslTokenType::ModuloAssign => Self::ModuloAssign,
            crate::lexer::token_type::HlslTokenType::Equal => Self::Equal,
            crate::lexer::token_type::HlslTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::HlslTokenType::Less => Self::Less,
            crate::lexer::token_type::HlslTokenType::Greater => Self::Greater,
            crate::lexer::token_type::HlslTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::HlslTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::HlslTokenType::LogicalAnd => Self::LogicalAnd,
            crate::lexer::token_type::HlslTokenType::LogicalOr => Self::LogicalOr,
            crate::lexer::token_type::HlslTokenType::LogicalNot => Self::LogicalNot,
            crate::lexer::token_type::HlslTokenType::BitwiseAnd => Self::BitwiseAnd,
            crate::lexer::token_type::HlslTokenType::BitwiseOr => Self::BitwiseOr,
            crate::lexer::token_type::HlslTokenType::BitwiseXor => Self::BitwiseXor,
            crate::lexer::token_type::HlslTokenType::BitwiseNot => Self::BitwiseNot,
            crate::lexer::token_type::HlslTokenType::LeftShift => Self::LeftShift,
            crate::lexer::token_type::HlslTokenType::RightShift => Self::RightShift,
            crate::lexer::token_type::HlslTokenType::LeftShiftAssign => Self::LeftShiftAssign,
            crate::lexer::token_type::HlslTokenType::RightShiftAssign => Self::RightShiftAssign,
            crate::lexer::token_type::HlslTokenType::BitwiseAndAssign => Self::BitwiseAndAssign,
            crate::lexer::token_type::HlslTokenType::BitwiseOrAssign => Self::BitwiseOrAssign,
            crate::lexer::token_type::HlslTokenType::BitwiseXorAssign => Self::BitwiseXorAssign,
            crate::lexer::token_type::HlslTokenType::Increment => Self::Increment,
            crate::lexer::token_type::HlslTokenType::Decrement => Self::Decrement,
            crate::lexer::token_type::HlslTokenType::Dot => Self::Dot,
            crate::lexer::token_type::HlslTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::HlslTokenType::Conditional => Self::Conditional,
            crate::lexer::token_type::HlslTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::HlslTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::HlslTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::HlslTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::HlslTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::HlslTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::HlslTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::HlslTokenType::Comma => Self::Comma,
            crate::lexer::token_type::HlslTokenType::Colon => Self::Colon,
            crate::lexer::token_type::HlslTokenType::DoubleColon => Self::DoubleColon,
            crate::lexer::token_type::HlslTokenType::Question => Self::Question,
            crate::lexer::token_type::HlslTokenType::Hash => Self::Hash,
            crate::lexer::token_type::HlslTokenType::At => Self::At,
            crate::lexer::token_type::HlslTokenType::Backslash => Self::Backslash,
            crate::lexer::token_type::HlslTokenType::Eof => Self::Eof,
            crate::lexer::token_type::HlslTokenType::Root => Self::Root,
            crate::lexer::token_type::HlslTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
