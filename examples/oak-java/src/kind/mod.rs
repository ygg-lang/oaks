use oak_core::SyntaxKind;

/// Java 语言Token 类型
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
}

impl SyntaxKind for JavaSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::LineComment | Self::BlockComment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment)
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

impl JavaSyntaxKind {
    pub fn is_keyword(s: &str) -> bool {
        matches!(
            s,
            "abstract"
                | "assert"
                | "boolean"
                | "break"
                | "byte"
                | "case"
                | "catch"
                | "char"
                | "class"
                | "const"
                | "continue"
                | "default"
                | "do"
                | "double"
                | "else"
                | "enum"
                | "extends"
                | "final"
                | "finally"
                | "float"
                | "for"
                | "if"
                | "goto"
                | "implements"
                | "import"
                | "instanceof"
                | "int"
                | "interface"
                | "long"
                | "native"
                | "new"
                | "package"
                | "private"
                | "protected"
                | "public"
                | "return"
                | "short"
                | "static"
                | "strictfp"
                | "super"
                | "switch"
                | "synchronized"
                | "this"
                | "throw"
                | "throws"
                | "transient"
                | "try"
                | "void"
                | "volatile"
                | "while"
        )
    }

    pub fn from_keyword_str(s: &str) -> Option<JavaSyntaxKind> {
        match s {
            "abstract" => Some(JavaSyntaxKind::Abstract),
            "assert" => Some(JavaSyntaxKind::Assert),
            "boolean" => Some(JavaSyntaxKind::Boolean),
            "break" => Some(JavaSyntaxKind::Break),
            "byte" => Some(JavaSyntaxKind::Byte),
            "case" => Some(JavaSyntaxKind::Case),
            "catch" => Some(JavaSyntaxKind::Catch),
            "char" => Some(JavaSyntaxKind::Char),
            "class" => Some(JavaSyntaxKind::Class),
            "const" => Some(JavaSyntaxKind::Const),
            "continue" => Some(JavaSyntaxKind::Continue),
            "default" => Some(JavaSyntaxKind::Default),
            "do" => Some(JavaSyntaxKind::Do),
            "double" => Some(JavaSyntaxKind::Double),
            "else" => Some(JavaSyntaxKind::Else),
            "enum" => Some(JavaSyntaxKind::Enum),
            "extends" => Some(JavaSyntaxKind::Extends),
            "final" => Some(JavaSyntaxKind::Final),
            "finally" => Some(JavaSyntaxKind::Finally),
            "float" => Some(JavaSyntaxKind::Float),
            "for" => Some(JavaSyntaxKind::For),
            "if" => Some(JavaSyntaxKind::If),
            "goto" => Some(JavaSyntaxKind::Goto),
            "implements" => Some(JavaSyntaxKind::Implements),
            "import" => Some(JavaSyntaxKind::Import),
            "instanceof" => Some(JavaSyntaxKind::Instanceof),
            "int" => Some(JavaSyntaxKind::Int),
            "interface" => Some(JavaSyntaxKind::Interface),
            "long" => Some(JavaSyntaxKind::Long),
            "native" => Some(JavaSyntaxKind::Native),
            "new" => Some(JavaSyntaxKind::New),
            "package" => Some(JavaSyntaxKind::Package),
            "private" => Some(JavaSyntaxKind::Private),
            "protected" => Some(JavaSyntaxKind::Protected),
            "public" => Some(JavaSyntaxKind::Public),
            "return" => Some(JavaSyntaxKind::Return),
            "short" => Some(JavaSyntaxKind::Short),
            "static" => Some(JavaSyntaxKind::Static),
            "strictfp" => Some(JavaSyntaxKind::Strictfp),
            "super" => Some(JavaSyntaxKind::Super),
            "switch" => Some(JavaSyntaxKind::Switch),
            "synchronized" => Some(JavaSyntaxKind::Synchronized),
            "this" => Some(JavaSyntaxKind::This),
            "throw" => Some(JavaSyntaxKind::Throw),
            "throws" => Some(JavaSyntaxKind::Throws),
            "transient" => Some(JavaSyntaxKind::Transient),
            "try" => Some(JavaSyntaxKind::Try),
            "void" => Some(JavaSyntaxKind::Void),
            "volatile" => Some(JavaSyntaxKind::Volatile),
            "while" => Some(JavaSyntaxKind::While),
            "true" => Some(JavaSyntaxKind::BooleanLiteral),
            "false" => Some(JavaSyntaxKind::BooleanLiteral),
            "null" => Some(JavaSyntaxKind::NullLiteral),
            _ => None,
        }
    }
}
