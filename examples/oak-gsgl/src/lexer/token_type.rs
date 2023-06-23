use core::fmt;
use oak_core::{Token, TokenType, UniversalTokenRole};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type GsglToken = Token<GsglTokenType>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GsglTokenType {
    // Non-terminal nodes
    Root,
    SourceFile,
    FunctionDefinition,
    VariableDeclaration,
    StructDefinition,
    Block,
    IfStatement,
    ForStatement,
    WhileStatement,
    ReturnStatement,

    // Keywords
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

    // Data types
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

    // Identifiers and literals
    Identifier,
    Number,
    String,

    // Operators
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

    // Comparison operators
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // Logical operators
    And,
    Or,
    Not,

    // Bitwise operators
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    LeftShift,
    RightShift,

    // Punctuations
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
    Hash,
    At,

    // Preprocessors
    Preprocessor,
    Include,
    Define,
    Ifdef,
    Ifndef,
    Endif,
    Version,

    // Builtin functions
    Sin,
    Cos,
    Tan,
    Sqrt,
    Pow,
    Exp,
    Log,
    Abs,
    Sign,
    Floor,
    Ceil,
    Fract,
    Mod,
    Min,
    Max,
    Clamp,
    Mix,
    Step,
    Smoothstep,
    Length,
    Distance,
    DotProduct,
    Cross,
    Normalize,
    Faceforward,
    Reflect,
    Refract,

    // Special tokens
    Whitespace,
    Comment,
    Newline,
    Eof,
    Error,
}

impl GsglTokenType {
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

impl fmt::Display for GsglTokenType {
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
            Self::Include => "include",
            Self::Define => "define",
            Self::Ifdef => "ifdef",
            Self::Ifndef => "ifndef",
            Self::Endif => "endif",
            Self::Version => "version",
            Self::Sin => "sin",
            Self::Cos => "cos",
            Self::Tan => "tan",
            Self::Sqrt => "sqrt",
            Self::Pow => "pow",
            Self::Exp => "exp",
            Self::Log => "log",
            Self::Abs => "abs",
            Self::Sign => "sign",
            Self::Floor => "floor",
            Self::Ceil => "ceil",
            Self::Fract => "fract",
            Self::Mod => "mod",
            Self::Min => "min",
            Self::Max => "max",
            Self::Clamp => "clamp",
            Self::Mix => "mix",
            Self::Step => "step",
            Self::Smoothstep => "smoothstep",
            Self::Length => "length",
            Self::Distance => "distance",
            Self::DotProduct => "dot",
            Self::Cross => "cross",
            Self::Normalize => "normalize",
            Self::Faceforward => "faceforward",
            Self::Reflect => "reflect",
            Self::Refract => "refract",
            Self::Whitespace => "WHITESPACE",
            Self::Comment => "COMMENT",
            Self::Newline => "NEWLINE",
            Self::Eof => "EOF",
            Self::Error => "ERROR",
            _ => "UNKNOWN",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Debug for GsglTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl TokenType for GsglTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::Newline)
    }

    fn role(&self) -> Self::Role {
        if self.is_keyword() {
            return UniversalTokenRole::Keyword;
        }
        if self.is_type() {
            return UniversalTokenRole::Keyword;
        }
        if self.is_operator() {
            return UniversalTokenRole::Operator;
        }

        match self {
            Self::Identifier => UniversalTokenRole::Name,
            Self::Number => UniversalTokenRole::Literal,
            Self::String => UniversalTokenRole::Literal,
            Self::True | Self::False => UniversalTokenRole::Literal,
            Self::Comment => UniversalTokenRole::Comment,
            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }
}
