use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HlslSyntaxKind {
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
    Error,
}

use oak_core::SyntaxKind;

impl SyntaxKind for HlslSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error)
    }

    fn is_element_type(&self) -> bool {
        false
    }
}
