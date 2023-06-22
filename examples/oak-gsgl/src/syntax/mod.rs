use core::fmt;
use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};

/// GSGL 语法节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum GsglSyntaxKind {
    // 根节点
    Root,
    SourceFile,

    // 关键字
    Shader,
    Vertex,
    Fragment,
    Geometry,
    Compute,
    Uniform,
    Attribute,
    Varying,
    In,
    Out,
    Inout,
    Const,
    Struct,
    If,
    Else,
    For,
    While,
    Do,
    Break,
    Continue,
    Return,
    Discard,
    True,
    False,

    // 数据类型
    Float,
    Int,
    Bool,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
    Sampler2D,
    SamplerCube,
    Void,

    // 标识符和字面量
    Identifier,
    Number,
    String,

    // 操作符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,

    // 比较操作符
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // 逻辑操作符
    And,
    Or,
    Not,

    // 位操作符
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    LeftShift,
    RightShift,

    // 分隔符
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

    // 预处理器
    Preprocessor,
    Include,
    Define,
    Ifdef,
    Ifndef,
    Endif,
    Version,

    // 内置函数
    Sin,
    Cos,
    Tan,
    Sqrt,
    Pow,
    Abs,
    Min,
    Max,
    Clamp,
    Mix,
    Step,
    Smoothstep,
    Length,
    Distance,
    DotFunc,
    Cross,
    Normalize,
    Reflect,
    Refract,
    Texture2D,
    TextureCube,

    // 空白和注释
    Whitespace,
    Newline,
    Comment,

    // 错误和结束
    Error,
    Eof,

    // 语法结构
    FunctionDecl,
    VariableDecl,
    StructDecl,
    Block,
    Statement,
    Expression,
    Parameter,
    Argument,
    FieldAccess,
    ArrayAccess,
    FunctionCall,
    BinaryExpr,
    UnaryExpr,
    AssignmentExpr,
    ConditionalExpr,
    Literal,
}

impl GsglSyntaxKind {
    /// 检查是否为关键字
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

    /// 检查是否为数据类型
    pub fn is_type(self) -> bool {
        matches!(self, Self::Float | Self::Int | Self::Bool | Self::Vec2 | Self::Vec3 | Self::Vec4 | Self::Mat2 | Self::Mat3 | Self::Mat4 | Self::Sampler2D | Self::SamplerCube | Self::Void)
    }

    /// 检查是否为操作符
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

impl fmt::Display for GsglSyntaxKind {
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
            Self::DotFunc => "dot",
            Self::Cross => "cross",
            Self::Normalize => "normalize",
            Self::Reflect => "reflect",
            Self::Refract => "refract",
            Self::Texture2D => "texture2D",
            Self::TextureCube => "textureCube",
            Self::Whitespace => "WHITESPACE",
            Self::Newline => "NEWLINE",
            Self::Comment => "COMMENT",
            Self::Error => "ERROR",
            Self::Eof => "EOF",
            Self::FunctionDecl => "FUNCTION_DECL",
            Self::VariableDecl => "VARIABLE_DECL",
            Self::StructDecl => "STRUCT_DECL",
            Self::Block => "BLOCK",
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
        };
        write!(f, "{}", name)
    }
}

impl TokenType for GsglSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for GsglSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::SourceFile => UniversalElementRole::Root,
            Self::FunctionDecl | Self::VariableDecl | Self::StructDecl => UniversalElementRole::Definition,
            Self::Statement => UniversalElementRole::Statement,
            Self::Expression | Self::BinaryExpr | Self::UnaryExpr | Self::AssignmentExpr | Self::ConditionalExpr => UniversalElementRole::Expression,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::Root | Self::SourceFile)
    }
}
