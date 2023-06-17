use core::fmt;
use oak_core::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CSyntaxKind {
    // 标点符号
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Semicolon,    // ;
    Colon,        // :
    Dot,          // .
    Question,     // ?

    // 运算
    Plus,             // +
    Minus,            // -
    Star,             // *
    Slash,            // /
    Percent,          // %
    Assign,           // =
    PlusAssign,       // +=
    MinusAssign,      // -=
    StarAssign,       // *=
    SlashAssign,      // /=
    PercentAssign,    // %=
    Equal,            // ==
    NotEqual,         // !=
    Less,             // <
    Greater,          // >
    LessEqual,        // <=
    GreaterEqual,     // >=
    LogicalAnd,       // &&
    LogicalOr,        // ||
    LogicalNot,       // !
    BitAnd,           // &
    BitOr,            // |
    BitXor,           // ^
    BitNot,           // ~
    LeftShift,        // <<
    RightShift,       // >>
    AndAssign,        // &=
    OrAssign,         // |=
    XorAssign,        // ^=
    LeftShiftAssign,  // <<=
    RightShiftAssign, // >>=
    Increment,        // ++
    Decrement,        // --
    Arrow,            // ->

    // 关键
    Auto,
    Register,
    Static,
    Extern,
    Typedef,
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    Unsigned,
    Struct,
    Union,
    Enum,
    Const,
    Volatile,
    Restrict,
    If,
    Else,
    Switch,
    Case,
    Default,
    For,
    While,
    Do,
    Break,
    Continue,
    Goto,
    Return,
    Sizeof,
    Inline,
    Bool,
    Complex,
    Imaginary,
    Alignas,
    Alignof,
    Atomic,
    StaticAssert,
    ThreadLocal,
    Generic,
    Noreturn,

    // 字面
    IntegerLiteral,
    FloatLiteral,
    CharLiteral,
    StringLiteral,

    // 其他
    Identifier,
    Whitespace,
    Comment,
    PreprocessorDirective,
    CodeFence,
    CodeLanguage,
    BlockquoteMarker,
    HorizontalRule,
    Underscore,
    Backtick,
    Tilde,
    Hash,
    Pipe,
    Dash,
    Exclamation,
    Escape,
    Text,
    Image,
    Link,
    TaskMarker,
    ListMarker,
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    InlineCode,
    Strikethrough,
    Error,
    Eof,
}

impl SyntaxKind for CSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error | Self::Eof)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Error | Self::Eof)
    }
}

impl fmt::Display for CSyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
