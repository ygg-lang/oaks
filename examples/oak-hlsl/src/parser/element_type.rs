use oak_core::{ElementType, Parser, UniversalElementRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum HlslElementType {
    // Whitespace and newline
    Whitespace,
    Newline,

    // Comment
    Comment,

    // Literals
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,

    // Identifiers and keywords
    Identifier,

    // Data types
    Bool,
    Int,
    Uint,
    Half,
    Float,
    Double,
    Min16float,
    Min10float,
    Min16int,
    Min12int,
    Min16uint,

    // Vector types
    Bool2,
    Bool3,
    Bool4,
    Int2,
    Int3,
    Int4,
    Uint2,
    Uint3,
    Uint4,
    Half2,
    Half3,
    Half4,
    Float2,
    Float3,
    Float4,
    Double2,
    Double3,
    Double4,

    // Matrix types
    Float2x2,
    Float2x3,
    Float2x4,
    Float3x2,
    Float3x3,
    Float3x4,
    Float4x2,
    Float4x3,
    Float4x4,
    Double2x2,
    Double2x3,
    Double2x4,
    Double3x2,
    Double3x3,
    Double3x4,
    Double4x2,
    Double4x3,
    Double4x4,

    // Texture types
    Texture1D,
    Texture2D,
    Texture3D,
    TextureCube,
    Texture1DArray,
    Texture2DArray,
    TextureCubeArray,
    Texture2DMS,
    Texture2DMSArray,

    // Sampler types
    Sampler,
    SamplerState,
    SamplerComparisonState,

    // Buffer types
    Buffer,
    StructuredBuffer,
    ByteAddressBuffer,
    RWBuffer,
    RWStructuredBuffer,
    RWByteAddressBuffer,
    AppendStructuredBuffer,
    ConsumeStructuredBuffer,

    // Control flow keywords
    If,
    Else,
    For,
    While,
    Do,
    Switch,
    Case,
    Default,
    Break,
    Continue,
    Return,
    Discard,

    // Function and variable modifiers
    Static,
    Const,
    Uniform,
    Varying,
    In,
    Out,
    Inout,
    Inline,
    Extern,
    Shared,
    Groupshared,
    Volatile,
    Precise,
    Nointerpolation,
    Linear,
    Centroid,
    Sample,
    Noperspective,
    Target,

    // Semantic modifiers
    Register,
    Packoffset,

    // Special keywords
    Struct,
    Cbuffer,
    Tbuffer,
    Technique,
    Pass,
    Interface,
    Class,
    Namespace,
    Typedef,
    Template,
    Typename,
    Using,
    Sizeof,
    Undef,

    // Preprocessor directives
    Include,
    Define,
    If_,
    Ifdef,
    Ifndef,
    Else_,
    Elif,
    Endif,
    Line,
    Pragma,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    LeftShiftAssign,
    RightShiftAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    Increment,
    Decrement,
    Dot,
    Arrow,
    Conditional,

    // Separators
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Colon,
    DoubleColon,
    Question,
    Hash,
    At,
    Backslash,

    // Special tokens
    Eof,
    Root,
    FunctionDeclaration,
    StructDeclaration,
    VariableDeclaration,
    ParameterList,
    Parameter,
    Block,
    Statement,
    Expression,
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
        unsafe { std::mem::transmute(token) }
    }
}
