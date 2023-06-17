/// 统一Scala 语法种类（包含节点与词法单元）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalaSyntaxKind {
    // 节点种类
    SourceFile,

    // 基础词法种类
    Whitespace,
    Newline,
    Error,
    Eof,
    ErrorNode,

    // 标识符和字面量
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BooleanLiteral,

    // 关键字
    Abstract,
    Case,
    Catch,
    Class,
    Def,
    Do,
    Else,
    Extends,
    False,
    Final,
    Finally,
    For,
    ForSome,
    If,
    Implicit,
    Import,
    Lazy,
    Match,
    New,
    Null,
    Object,
    Override,
    Package,
    Private,
    Protected,
    Return,
    Sealed,
    Super,
    This,
    Throw,
    Trait,
    Try,
    True,
    Type,
    Val,
    Var,
    While,
    With,
    Yield,

    // 操作符
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Eq,         // =
    EqEq,       // ==
    Ne,         // !=
    Lt,         // <
    Le,         // <=
    Gt,         // >
    Ge,         // >=
    And,        // &
    Or,         // |
    Xor,        // ^
    AndAnd,     // &&
    OrOr,       // ||
    Not,        // !
    Tilde,      // ~
    LShift,     // <<
    RShift,     // >>
    URShift,    // >>>
    PlusEq,     // +=
    MinusEq,    // -=
    StarEq,     // *=
    SlashEq,    // /=
    PercentEq,  // %=
    AndEq,      // &=
    OrEq,       // |=
    XorEq,      // ^=
    LShiftEq,   // <<=
    RShiftEq,   // >>=
    URShiftEq,  // >>>=
    Arrow,      // =>
    LeftArrow,  // <-
    Colon,      // :
    ColonColon, // ::
    Semicolon,  // ;
    Dot,        // .
    Comma,      // ,
    Question,   // ?
    At,         // @
    Hash,       // #

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }

    // 注释
    LineComment,
    BlockComment,
    DocComment,
}
