use oak_core::SyntaxKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WolframSyntaxKind {
    // 基础 tokens
    Whitespace,
    Newline,

    // 标识符和字面量
    Identifier,
    Integer,
    Real,
    String,

    // 关键字
    If,
    Then,
    Else,
    While,
    For,
    Do,
    Function,
    Module,
    Block,
    With,
    Table,
    Map,
    Apply,
    Select,
    Cases,
    Rule,
    RuleDelayed,
    Set,
    SetDelayed,
    Unset,
    Clear,
    ClearAll,
    Return,
    Break,
    Continue,
    True,
    False,
    Null,
    Export,
    Import,

    // 运算符
    Plus,         // +
    Minus,        // -
    Times,        // *
    Divide,       // /
    Power,        // ^
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=
    And,          // &&
    Or,           // ||
    Not,          // !

    // 赋值运算符
    Assign,       // =
    AddTo,        // +=
    SubtractFrom, // -=
    TimesBy,      // *=
    DivideBy,     // /=

    // 分隔符
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

    // 特殊符号
    Arrow,            // ->
    DoubleArrow,      // =>
    Question,         // ?
    Underscore,       // _
    DoubleUnderscore, // __
    TripleUnderscore, // ___
    Slot,             // #
    SlotSequence,     // ##

    // 注释
    Comment,

    // 文本
    Text,

    // 错误处理
    Error,

    // EOF
    Eof,
}

impl SyntaxKind for WolframSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, WolframSyntaxKind::Whitespace | WolframSyntaxKind::Newline | WolframSyntaxKind::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, WolframSyntaxKind::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, WolframSyntaxKind::Whitespace | WolframSyntaxKind::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, WolframSyntaxKind::Error | WolframSyntaxKind::Eof)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, WolframSyntaxKind::Error | WolframSyntaxKind::Eof)
    }
}
