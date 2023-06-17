use oak_core::SyntaxKind;

/// C++ 语法节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum CppSyntaxKind {
    // 琐碎tokens
    Whitespace,
    Newline,
    Comment,

    // 字面量
    StringLiteral,
    CharacterLiteral,
    IntegerLiteral,
    FloatLiteral,
    BooleanLiteral,

    // 标识符和关键字
    Identifier,
    Keyword,

    // 操作符
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
    Dot,              // .
    Question,         // ?
    Colon,            // :
    Scope,            // ::

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Semicolon,    // ;

    // 预处理器
    Preprocessor,

    // 复合节点
    SourceFile,
    Error,
    Eof,
}

impl SyntaxKind for CppSyntaxKind {
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
        matches!(self, Self::Error)
    }
}
