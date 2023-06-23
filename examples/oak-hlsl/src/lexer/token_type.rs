use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type HlslToken = Token<HlslTokenType>;

impl TokenType for HlslTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HlslTokenType {
    // 空白字符和换行
    Whitespace,
    Newline,

    // 注释
    Comment,

    // 字面量
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,

    // 标识符和关键字
    Identifier,

    // 数据类型
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

    // 向量类型
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

    // 矩阵类型
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

    // 纹理类型
    Texture1D,
    Texture2D,
    Texture3D,
    TextureCube,
    Texture1DArray,
    Texture2DArray,
    TextureCubeArray,
    Texture2DMS,
    Texture2DMSArray,

    // 采样器类型
    Sampler,
    SamplerState,
    SamplerComparisonState,

    // 缓冲区类型
    Buffer,
    StructuredBuffer,
    ByteAddressBuffer,
    RWBuffer,
    RWStructuredBuffer,
    RWByteAddressBuffer,
    AppendStructuredBuffer,
    ConsumeStructuredBuffer,

    // 控制流关键字
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

    // 函数和变量修饰符
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

    // 语义修饰符
    Register,
    Packoffset,

    // 特殊关键字
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

    // 预处理器指令
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

    // 运算符
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

    // 分隔符
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

    // 特殊标记
    Eof,
    Root,
    Error,
}
