/// 统一 SCSS 语法种类（包含节点与词法）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScssSyntaxKind {
    // 节点种类
    SourceFile,

    // 基础词法种类
    Whitespace,
    Newline,
    Comment,
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

    // SCSS 关键字
    Import,
    Include,
    Mixin,
    Function,
    Return,
    If,
    Else,
    For,
    While,
    Each,
    In,
    From,
    To,
    Through,
    Default,
    Important,
    Optional,
    Global,
    Null,
    True,
    False,

    // 临时保留的 Scala 关键字（为了兼容现有 lexer）
    Abstract,
    Case,
    Catch,
    Class,
    Def,
    Do,
    Extends,
    Final,
    Finally,
    ForSome,
    Implicit,
    Lazy,
    Match,
    New,
    Object,
    Override,
    Package,
    Private,
    Protected,
    Sealed,
    Super,
    This,
    Throw,
    Trait,
    Try,
    Type,
    Val,
    Var,
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
    Bang,       // ! (alternative name)
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
    Dollar,     // $

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

impl oak_core::SyntaxKind for ScssSyntaxKind {
    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment | Self::DocComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_trivia(&self) -> bool {
        self.is_comment() || self.is_whitespace()
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::SourceFile | Self::ErrorNode)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::SourceFile | Self::ErrorNode)
    }
}
