use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};

pub type HlslToken = Token<HlslTokenType>;

impl TokenType for HlslTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Eof => UniversalTokenRole::Eof,
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral | Self::NumberLiteral | Self::BooleanLiteral => UniversalTokenRole::Literal,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HlslTokenType {
    // Whitespace and newline
    Whitespace,
    Newline,

    // Comments
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

    // Punctuations
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
