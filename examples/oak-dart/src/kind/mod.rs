use oak_core::SyntaxKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DartSyntaxKind {
    // 基础
    Whitespace,
    Newline,

    // 标识符和字面量
    Identifier,
    IntegerLiteral,
    DoubleLiteral,
    StringLiteral,
    BooleanLiteral,
    NullLiteral,

    // 关键字
    Abstract,
    As,
    Assert,
    Async,
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Covariant,
    Default,
    Deferred,
    Do,
    Dynamic,
    Else,
    Enum,
    Export,
    Extends,
    Extension,
    External,
    Factory,
    False,
    Final,
    Finally,
    For,
    Function,
    Get,
    Hide,
    If,
    Implements,
    Import,
    In,
    Interface,
    Is,
    Late,
    Library,
    Mixin,
    New,
    Null,
    On,
    Operator,
    Part,
    Required,
    Rethrow,
    Return,
    Set,
    Show,
    Static,
    Super,
    Switch,
    Sync,
    This,
    Throw,
    True,
    Try,
    Typedef,
    Var,
    Void,
    While,
    With,
    Yield,

    // 操作符
    Plus,                  // +
    Minus,                 // -
    Star,                  // *
    Slash,                 // /
    Percent,               // %
    TildeSlash,            // ~/
    Equal,                 // =
    EqualEqual,            // ==
    BangEqual,             // !=
    Less,                  // <
    Greater,               // >
    LessEqual,             // <=
    GreaterEqual,          // >=
    LeftShift,             // <<
    RightShift,            // >>
    Ampersand,             // &
    Pipe,                  // |
    Caret,                 // ^
    Tilde,                 // ~
    Bang,                  // !
    AmpersandAmpersand,    // &&
    PipePipe,              // ||
    Question,              // ?
    QuestionQuestion,      // ??
    PlusPlus,              // ++
    MinusMinus,            // --
    PlusEqual,             // +=
    MinusEqual,            // -=
    StarEqual,             // *=
    SlashEqual,            // /=
    PercentEqual,          // %=
    TildeSlashEqual,       // ~/=
    LeftShiftEqual,        // <<=
    RightShiftEqual,       // >>=
    AmpersandEqual,        // &=
    PipeEqual,             // |=
    CaretEqual,            // ^=
    QuestionQuestionEqual, // ??=
    Arrow,                 // =>
    Dot,                   // .
    DotDot,                // ..
    DotDotDot,             // ...
    QuestionDot,           // ?.

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Semicolon,    // ;
    Comma,        // ,
    Colon,        // :
    At,           // @
    Hash,         // #

    // 注释
    LineComment,
    BlockComment,
    DocComment,

    // 错误处理
    Error,

    // EOF
    Eof,
}

impl SyntaxKind for DartSyntaxKind {
    fn is_trivia(&self) -> bool {
        todo!()
    }

    fn is_comment(&self) -> bool {
        todo!()
    }

    fn is_whitespace(&self) -> bool {
        todo!()
    }

    fn is_token_type(&self) -> bool {
        todo!()
    }

    fn is_element_type(&self) -> bool {
        todo!()
    }
}
