use oak_core::{Token, TokenType, UniversalTokenRole};

/// WGSL token.
pub type WgslToken = Token<WgslTokenType>;

impl TokenType for WgslTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        match self {
            Self::Whitespace | Self::Newline | Self::Comment => true,
            _ => false,
        }
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::BoolLiteral => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::BoolKw
            | Self::I32Kw
            | Self::U32Kw
            | Self::F32Kw
            | Self::F16Kw
            | Self::Vec2Kw
            | Self::Vec3Kw
            | Self::Vec4Kw
            | Self::Mat2x2Kw
            | Self::Mat2x3Kw
            | Self::Mat2x4Kw
            | Self::Mat3x2Kw
            | Self::Mat3x3Kw
            | Self::Mat3x4Kw
            | Self::Mat4x2Kw
            | Self::Mat4x3Kw
            | Self::Mat4x4Kw
            | Self::ArrayKw
            | Self::PtrKw
            | Self::AtomicKw
            | Self::SamplerKw
            | Self::SamplerComparisonKw
            | Self::Texture1dKw
            | Self::Texture2dKw
            | Self::Texture2dArrayKw
            | Self::Texture3dKw
            | Self::TextureCubeKw
            | Self::TextureCubeArrayKw
            | Self::TextureMultisampled2dKw
            | Self::TextureDepth2dKw
            | Self::TextureDepthCubeKw
            | Self::TextureDepthMultisampled2dKw
            | Self::TextureStorage1dKw
            | Self::TextureStorage2dKw
            | Self::TextureStorage2dArrayKw
            | Self::TextureStorage3dKw
            | Self::FnKw
            | Self::VarKw
            | Self::LetKw
            | Self::ConstKw
            | Self::IfKw
            | Self::ElseKw
            | Self::SwitchKw
            | Self::CaseKw
            | Self::DefaultKw
            | Self::LoopKw
            | Self::ForKw
            | Self::WhileKw
            | Self::BreakKw
            | Self::ContinueKw
            | Self::ReturnKw
            | Self::DiscardKw
            | Self::FunctionKw
            | Self::PrivateKw
            | Self::WorkgroupKw
            | Self::UniformKw
            | Self::StorageKw
            | Self::ReadKw
            | Self::WriteKw
            | Self::ReadWriteKw
            | Self::VertexKw
            | Self::FragmentKw
            | Self::ComputeKw
            | Self::BindingKw
            | Self::GroupKw
            | Self::LocationKw
            | Self::BuiltinKw
            | Self::InterpolateKw
            | Self::InvariantKw
            | Self::SizeKw
            | Self::AlignKw
            | Self::WorkgroupSizeKw
            | Self::PositionKw
            | Self::VertexIndexKw
            | Self::InstanceIndexKw
            | Self::FrontFacingKw
            | Self::FragDepthKw
            | Self::LocalInvocationIdKw
            | Self::LocalInvocationIndexKw
            | Self::GlobalInvocationIdKw
            | Self::WorkgroupIdKw
            | Self::NumWorkgroupsKw
            | Self::SampleIndexKw
            | Self::SampleMaskKw
            | Self::StructKw
            | Self::TypeKw
            | Self::AliasKw
            | Self::EnableKw
            | Self::RequiresKw
            | Self::OverrideKw => UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Ampersand
            | Self::Pipe
            | Self::Caret
            | Self::Tilde
            | Self::LeftShift
            | Self::RightShift
            | Self::PlusAssign
            | Self::MinusAssign
            | Self::StarAssign
            | Self::SlashAssign
            | Self::PercentAssign
            | Self::AmpersandAssign
            | Self::PipeAssign
            | Self::CaretAssign
            | Self::LeftShiftAssign
            | Self::RightShiftAssign
            | Self::Increment
            | Self::Decrement
            | Self::Eq
            | Self::Ne
            | Self::Lt
            | Self::Le
            | Self::Gt
            | Self::Ge
            | Self::EqEq
            | Self::BangEq
            | Self::LtLt
            | Self::GtGt
            | Self::AmpersandAmpersand
            | Self::PipePipe
            | Self::Bang
            | Self::Assign
            | Self::Arrow => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Semicolon | Self::Comma | Self::Dot | Self::Colon | Self::Question | Self::At | Self::Hash | Self::Dollar => {
                UniversalTokenRole::Punctuation
            }
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

/// WGSL token type.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WgslTokenType {
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,
    /// Error.
    Error,
    /// End of file.
    Eof,
    /// Text.
    Text,

    /// Integer literal.
    IntegerLiteral,
    /// Float literal.
    FloatLiteral,
    /// String literal.
    StringLiteral,
    /// Boolean literal.
    BoolLiteral,
    /// Identifier.
    Identifier,

    /// `bool` keyword.
    BoolKw,
    /// `i32` keyword.
    I32Kw,
    /// `u32` keyword.
    U32Kw,
    /// `f32` keyword.
    F32Kw,
    /// `f16` keyword.
    F16Kw,
    /// `vec2` keyword.
    Vec2Kw,
    /// `vec3` keyword.
    Vec3Kw,
    /// `vec4` keyword.
    Vec4Kw,
    /// `mat2x2` keyword.
    Mat2x2Kw,
    /// `mat2x3` keyword.
    Mat2x3Kw,
    /// `mat2x4` keyword.
    Mat2x4Kw,
    /// `mat3x2` keyword.
    Mat3x2Kw,
    /// `mat3x3` keyword.
    Mat3x3Kw,
    /// `mat3x4` keyword.
    Mat3x4Kw,
    /// `mat4x2` keyword.
    Mat4x2Kw,
    /// `mat4x3` keyword.
    Mat4x3Kw,
    /// `mat4x4` keyword.
    Mat4x4Kw,
    /// `array` keyword.
    ArrayKw,
    /// `ptr` keyword.
    PtrKw,
    /// `atomic` keyword.
    AtomicKw,
    /// `sampler` keyword.
    SamplerKw,
    /// `sampler_comparison` keyword.
    SamplerComparisonKw,
    /// `texture_1d` keyword.
    Texture1dKw,
    /// `texture_2d` keyword.
    Texture2dKw,
    /// `texture_2d_array` keyword.
    Texture2dArrayKw,
    /// `texture_3d` keyword.
    Texture3dKw,
    /// `texture_cube` keyword.
    TextureCubeKw,
    /// `texture_cube_array` keyword.
    TextureCubeArrayKw,
    /// `texture_multisampled_2d` keyword.
    TextureMultisampled2dKw,
    /// `texture_depth_2d` keyword.
    TextureDepth2dKw,
    /// `texture_depth_cube` keyword.
    TextureDepthCubeKw,
    /// `texture_depth_multisampled_2d` keyword.
    TextureDepthMultisampled2dKw,
    /// `texture_storage_1d` keyword.
    TextureStorage1dKw,
    /// `texture_storage_2d` keyword.
    TextureStorage2dKw,
    /// `texture_storage_2d_array` keyword.
    TextureStorage2dArrayKw,
    /// `texture_storage_3d` keyword.
    TextureStorage3dKw,

    /// `fn` keyword.
    FnKw,
    /// `var` keyword.
    VarKw,
    /// `let` keyword.
    LetKw,
    /// `const` keyword.
    ConstKw,
    /// `if` keyword.
    IfKw,
    /// `else` keyword.
    ElseKw,
    /// `switch` keyword.
    SwitchKw,
    /// `case` keyword.
    CaseKw,
    /// `default` keyword.
    DefaultKw,
    /// `loop` keyword.
    LoopKw,
    /// `for` keyword.
    ForKw,
    /// `while` keyword.
    WhileKw,
    /// `break` keyword.
    BreakKw,
    /// `continue` keyword.
    ContinueKw,
    /// `return` keyword.
    ReturnKw,
    /// `discard` keyword.
    DiscardKw,

    /// `function` keyword.
    FunctionKw,
    /// `private` keyword.
    PrivateKw,
    /// `workgroup` keyword.
    WorkgroupKw,
    /// `uniform` keyword.
    UniformKw,
    /// `storage` keyword.
    StorageKw,

    /// `read` keyword.
    ReadKw,
    /// `write` keyword.
    WriteKw,
    /// `read_write` keyword.
    ReadWriteKw,

    /// `vertex` keyword.
    VertexKw,
    /// `fragment` keyword.
    FragmentKw,
    /// `compute` keyword.
    ComputeKw,
    /// `binding` keyword.
    BindingKw,
    /// `group` keyword.
    GroupKw,
    /// `location` keyword.
    LocationKw,
    /// `builtin` keyword.
    BuiltinKw,
    /// `interpolate` keyword.
    InterpolateKw,
    /// `invariant` keyword.
    InvariantKw,
    /// `size` keyword.
    SizeKw,
    /// `align` keyword.
    AlignKw,
    /// `workgroup_size` keyword.
    WorkgroupSizeKw,

    /// `position` keyword.
    PositionKw,
    /// `vertex_index` keyword.
    VertexIndexKw,
    /// `instance_index` keyword.
    InstanceIndexKw,
    /// `front_facing` keyword.
    FrontFacingKw,
    /// `frag_depth` keyword.
    FragDepthKw,
    /// `local_invocation_id` keyword.
    LocalInvocationIdKw,
    /// `local_invocation_index` keyword.
    LocalInvocationIndexKw,
    /// `global_invocation_id` keyword.
    GlobalInvocationIdKw,
    /// `workgroup_id` keyword.
    WorkgroupIdKw,
    /// `num_workgroups` keyword.
    NumWorkgroupsKw,
    /// `sample_index` keyword.
    SampleIndexKw,
    /// `sample_mask` keyword.
    SampleMaskKw,

    /// `struct` keyword.
    StructKw,
    /// `type` keyword.
    TypeKw,
    /// `alias` keyword.
    AliasKw,
    /// `enable` keyword.
    EnableKw,
    /// `requires` keyword.
    RequiresKw,
    /// `override` keyword.
    OverrideKw,

    /// Plus `+`.
    Plus,
    /// Minus `-`.
    Minus,
    /// Star `*`.
    Star,
    /// Slash `/`.
    Slash,
    /// Percent `%`.
    Percent,
    /// Ampersand `&`.
    Ampersand,
    /// Pipe `|`.
    Pipe,
    /// Caret `^`.
    Caret,
    /// Tilde `~`.
    Tilde,
    /// Left shift `<<`.
    LeftShift,
    /// Right shift `>>`.
    RightShift,
    /// Plus assign `+=`.
    PlusAssign,
    /// Minus assign `-=`.
    MinusAssign,
    /// Star assign `*=`.
    StarAssign,
    /// Slash assign `/=`.
    SlashAssign,
    /// Percent assign `%=`.
    PercentAssign,
    /// Ampersand assign `&=`.
    AmpersandAssign,
    /// Pipe assign `|=`.
    PipeAssign,
    /// Caret assign `^=`.
    CaretAssign,
    /// Left shift assign `<<=`.
    LeftShiftAssign,
    /// Right shift assign `>>=`.
    RightShiftAssign,
    /// Increment `++`.
    Increment,
    /// Decrement `--`.
    Decrement,

    /// Equal `=`.
    Eq,
    /// Not equal `!=`.
    Ne,
    /// Less than `<`.
    Lt,
    /// Less than or equal to `<=`.
    Le,
    /// Greater than `>`.
    Gt,
    /// Greater than or equal to `>=`.
    Ge,
    /// Double equal `==`.
    EqEq,
    /// Bang equal `!=`.
    BangEq,
    /// Less less `<<`.
    LtLt,
    /// Greater greater `>>`.
    GtGt,

    /// Double ampersand `&&`.
    AmpersandAmpersand,
    /// Double pipe `||`.
    PipePipe,
    /// Bang `!`.
    Bang,

    /// Assign `=`.
    Assign,
    /// Arrow `->`.
    Arrow,

    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Semicolon `;`.
    Semicolon,
    /// Comma `,`.
    Comma,
    /// Dot `.`.
    Dot,
    /// Colon `:`.
    Colon,
    /// Question mark `?`.
    Question,
    /// At symbol `@`.
    At,
    /// Hash symbol `#`.
    Hash,
    /// Dollar symbol `$`.
    Dollar,

    /// Root node.
    Root,
    /// Function node.
    Function,
    /// Struct node.
    Struct,
    /// Variable node.
    Variable,
    /// Block node.
    Block,
    /// Type alias node.
    TypeAlias,
}
