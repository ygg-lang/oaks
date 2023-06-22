use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// Java 语言Token 类型
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum JavaSyntaxKind {
    // 关键
    Abstract,
    Assert,
    Boolean,
    Break,
    Byte,
    Case,
    Catch,
    Char,
    Class,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extends,
    Final,
    Finally,
    Float,
    For,
    If,
    Goto,
    Implements,
    Import,
    Instanceof,
    Int,
    Interface,
    Long,
    Native,
    New,
    Package,
    Private,
    Protected,
    Public,
    Return,
    Short,
    Static,
    Strictfp,
    Super,
    Switch,
    Synchronized,
    This,
    Throw,
    Throws,
    Transient,
    Try,
    Void,
    Volatile,
    While,
    // 字面
    IntegerLiteral,
    FloatingPointLiteral,
    BooleanLiteral,
    CharacterLiteral,
    StringLiteral,
    NullLiteral,
    // 运算
    Assign,                   // =
    GreaterThan,              // >
    LessThan,                 // <
    Bang,                     // !
    Tilde,                    // ~
    Question,                 // ?
    Colon,                    // :
    Equals,                   // ==
    LessThanEquals,           // <=
    GreaterThanEquals,        // >=
    BangEquals,               // !=
    AmpersandAmpersand,       // &&
    PipePipe,                 // ||
    PlusPlus,                 // ++
    MinusMinus,               // --
    Plus,                     // +
    Minus,                    // -
    Asterisk,                 // *
    Slash,                    // /
    Ampersand,                // &
    Pipe,                     // |
    Caret,                    // ^
    Percent,                  // %
    LeftShift,                // <<
    RightShift,               // >>
    UnsignedRightShift,       // >>>
    PlusEquals,               // +=
    MinusEquals,              // -=
    AsteriskEquals,           // *=
    SlashEquals,              // /=
    AmpersandEquals,          // &=
    PipeEquals,               // |=
    CaretEquals,              // ^=
    PercentEquals,            // %=
    LeftShiftEquals,          // <<=
    RightShiftEquals,         // >>=
    UnsignedRightShiftEquals, // >>>=
    // 分隔
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Semicolon,    // ;
    Comma,        // ,
    Dot,          // .
    Ellipsis,     // ...
    At,           // @
    DoubleColon,  // ::
    // 标识
    Identifier,
    // 注释
    LineComment,
    BlockComment,
    // 空白
    Whitespace,
    // 文件结束
    Eof,
    // 错误
    Error,
    // 非终端节点
    CompilationUnit,
    ClassDeclaration,
    InterfaceDeclaration,
    MethodDeclaration,
    FieldDeclaration,
    Parameter,
    BlockStatement,
    IfStatement,
    WhileStatement,
    ForStatement,
    ReturnStatement,
    ExpressionStatement,
    BinaryExpression,
    MethodCall,
    MemberSelect,
    LiteralExpression,
    VariableDeclaration,
}

impl JavaSyntaxKind {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Abstract
                | Self::Assert
                | Self::Boolean
                | Self::Break
                | Self::Byte
                | Self::Case
                | Self::Catch
                | Self::Char
                | Self::Class
                | Self::Const
                | Self::Continue
                | Self::Default
                | Self::Do
                | Self::Double
                | Self::Else
                | Self::Enum
                | Self::Extends
                | Self::Final
                | Self::Finally
                | Self::Float
                | Self::For
                | Self::If
                | Self::Goto
                | Self::Implements
                | Self::Import
                | Self::Instanceof
                | Self::Int
                | Self::Interface
                | Self::Long
                | Self::Native
                | Self::New
                | Self::Package
                | Self::Private
                | Self::Protected
                | Self::Public
                | Self::Return
                | Self::Short
                | Self::Static
                | Self::Strictfp
                | Self::Super
                | Self::Switch
                | Self::Synchronized
                | Self::This
                | Self::Throw
                | Self::Throws
                | Self::Transient
                | Self::Try
                | Self::Void
                | Self::Volatile
                | Self::While
        )
    }

    pub fn from_keyword(s: &str) -> Option<Self> {
        match s {
            "abstract" => Some(Self::Abstract),
            "assert" => Some(Self::Assert),
            "boolean" => Some(Self::Boolean),
            "break" => Some(Self::Break),
            "byte" => Some(Self::Byte),
            "case" => Some(Self::Case),
            "catch" => Some(Self::Catch),
            "char" => Some(Self::Char),
            "class" => Some(Self::Class),
            "const" => Some(Self::Const),
            "continue" => Some(Self::Continue),
            "default" => Some(Self::Default),
            "do" => Some(Self::Do),
            "double" => Some(Self::Double),
            "else" => Some(Self::Else),
            "enum" => Some(Self::Enum),
            "extends" => Some(Self::Extends),
            "final" => Some(Self::Final),
            "finally" => Some(Self::Finally),
            "float" => Some(Self::Float),
            "for" => Some(Self::For),
            "if" => Some(Self::If),
            "goto" => Some(Self::Goto),
            "implements" => Some(Self::Implements),
            "import" => Some(Self::Import),
            "instanceof" => Some(Self::Instanceof),
            "int" => Some(Self::Int),
            "interface" => Some(Self::Interface),
            "long" => Some(Self::Long),
            "native" => Some(Self::Native),
            "new" => Some(Self::New),
            "package" => Some(Self::Package),
            "private" => Some(Self::Private),
            "protected" => Some(Self::Protected),
            "public" => Some(Self::Public),
            "return" => Some(Self::Return),
            "short" => Some(Self::Short),
            "static" => Some(Self::Static),
            "strictfp" => Some(Self::Strictfp),
            "super" => Some(Self::Super),
            "switch" => Some(Self::Switch),
            "synchronized" => Some(Self::Synchronized),
            "this" => Some(Self::This),
            "throw" => Some(Self::Throw),
            "throws" => Some(Self::Throws),
            "transient" => Some(Self::Transient),
            "try" => Some(Self::Try),
            "void" => Some(Self::Void),
            "volatile" => Some(Self::Volatile),
            "while" => Some(Self::While),
            _ => None,
        }
    }
}

impl TokenType for JavaSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::FloatingPointLiteral | Self::BooleanLiteral | Self::CharacterLiteral | Self::StringLiteral | Self::NullLiteral => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Assign
            | Self::GreaterThan
            | Self::LessThan
            | Self::Bang
            | Self::Tilde
            | Self::Question
            | Self::Colon
            | Self::Equals
            | Self::LessThanEquals
            | Self::GreaterThanEquals
            | Self::BangEquals
            | Self::AmpersandAmpersand
            | Self::PipePipe
            | Self::PlusPlus
            | Self::MinusMinus
            | Self::Plus
            | Self::Minus
            | Self::Asterisk
            | Self::Slash
            | Self::Ampersand
            | Self::Pipe
            | Self::Caret
            | Self::Percent
            | Self::LeftShift
            | Self::RightShift
            | Self::UnsignedRightShift
            | Self::PlusEquals
            | Self::MinusEquals
            | Self::AsteriskEquals
            | Self::SlashEquals
            | Self::AmpersandEquals
            | Self::PipeEquals
            | Self::CaretEquals
            | Self::PercentEquals
            | Self::LeftShiftEquals
            | Self::RightShiftEquals
            | Self::UnsignedRightShiftEquals => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Semicolon | Self::Comma | Self::Dot | Self::Ellipsis | Self::At | Self::DoubleColon => UniversalTokenRole::Punctuation,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }
}

impl ElementType for JavaSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::CompilationUnit => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::CompilationUnit)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}
