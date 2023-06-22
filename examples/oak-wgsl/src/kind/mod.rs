use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WgslSyntaxKind {
    // 基础 kind
    Whitespace,
    Newline,
    Comment,
    Error,
    Eof,
    Text,

    // 字面量
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    BoolLiteral,
    Identifier,

    // WGSL 关键字 - 类型
    BoolKw,
    I32Kw,
    U32Kw,
    F32Kw,
    F16Kw,
    Vec2Kw,
    Vec3Kw,
    Vec4Kw,
    Mat2x2Kw,
    Mat2x3Kw,
    Mat2x4Kw,
    Mat3x2Kw,
    Mat3x3Kw,
    Mat3x4Kw,
    Mat4x2Kw,
    Mat4x3Kw,
    Mat4x4Kw,
    ArrayKw,
    PtrKw,
    AtomicKw,
    SamplerKw,
    SamplerComparisonKw,
    Texture1dKw,
    Texture2dKw,
    Texture2dArrayKw,
    Texture3dKw,
    TextureCubeKw,
    TextureCubeArrayKw,
    TextureMultisampled2dKw,
    TextureDepth2dKw,
    TextureDepthCubeKw,
    TextureDepthMultisampled2dKw,
    TextureStorage1dKw,
    TextureStorage2dKw,
    TextureStorage2dArrayKw,
    TextureStorage3dKw,

    // WGSL 关键字 - 函数和控制流
    FnKw,
    VarKw,
    LetKw,
    ConstKw,
    IfKw,
    ElseKw,
    SwitchKw,
    CaseKw,
    DefaultKw,
    LoopKw,
    ForKw,
    WhileKw,
    BreakKw,
    ContinueKw,
    ReturnKw,
    DiscardKw,

    // WGSL 关键字 - 存储类
    FunctionKw,
    PrivateKw,
    WorkgroupKw,
    UniformKw,
    StorageKw,

    // 存储访问模式
    ReadKw,
    WriteKw,
    ReadWriteKw,

    // WGSL 关键字 - 着色器阶段
    VertexKw,
    FragmentKw,
    ComputeKw,
    BindingKw,
    GroupKw,
    LocationKw,
    BuiltinKw,
    InterpolateKw,
    InvariantKw,
    SizeKw,
    AlignKw,
    WorkgroupSizeKw,

    // WGSL 关键字 - 内置
    PositionKw,
    VertexIndexKw,
    InstanceIndexKw,
    FrontFacingKw,
    FragDepthKw,
    LocalInvocationIdKw,
    LocalInvocationIndexKw,
    GlobalInvocationIdKw,
    WorkgroupIdKw,
    NumWorkgroupsKw,
    SampleIndexKw,
    SampleMaskKw,

    // WGSL 关键字 - 其他
    StructKw,
    TypeKw,
    AliasKw,
    EnableKw,
    RequiresKw,
    OverrideKw,

    // 操作符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    LeftShift,
    RightShift,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    AmpersandAssign,
    PipeAssign,
    CaretAssign,
    LeftShiftAssign,
    RightShiftAssign,
    Increment,
    Decrement,

    // 比较操作
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    EqEq,
    BangEq,
    LtLt,
    GtGt,

    // 逻辑操作
    AmpersandAmpersand,
    PipePipe,
    Bang,

    // 赋值操作符
    Assign,
    Arrow,

    // 标点符号
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Colon,
    Question,
    At,
    Hash,
    Dollar,

    // 语法节点
    Root,
    Function,
    Struct,
    Variable,
    Block,
    TypeAlias,
}

impl TokenType for WgslSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for WgslSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
