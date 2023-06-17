use core::fmt;
use oak_core::SyntaxKind;

/// Lua 语法节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum LuaSyntaxKind {
    // 根节
    Root,
    SourceFile,

    // 关键
    And,
    Break,
    Do,
    Else,
    Elseif,
    End,
    False,
    For,
    Function,
    Goto,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,

    // 标识符和字面
    Identifier,
    Number,
    String,

    // 操作
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Hash,
    Ampersand,
    Tilde,
    Pipe,
    LtLt,
    GtGt,
    SlashSlash,
    EqEq,
    TildeEq,
    LtEq,
    GtEq,
    Lt,
    Gt,
    Eq,

    // 标点符号
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    ColonColon,
    Semicolon,
    Colon,
    Comma,
    Dot,
    DotDot,
    DotDotDot,

    // 注释和空
    Comment,
    Whitespace,

    // 错误和结束符
    Error,
    Eof,
}

impl LuaSyntaxKind {
    /// 检查是否为关键
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            Self::And
                | Self::Break
                | Self::Do
                | Self::Else
                | Self::Elseif
                | Self::End
                | Self::False
                | Self::For
                | Self::Function
                | Self::Goto
                | Self::If
                | Self::In
                | Self::Local
                | Self::Nil
                | Self::Not
                | Self::Or
                | Self::Repeat
                | Self::Return
                | Self::Then
                | Self::True
                | Self::Until
                | Self::While
        )
    }

    /// 检查是否为操作
    pub fn is_operator(self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::Caret
                | Self::Hash
                | Self::Ampersand
                | Self::Tilde
                | Self::Pipe
                | Self::LtLt
                | Self::GtGt
                | Self::SlashSlash
                | Self::EqEq
                | Self::TildeEq
                | Self::LtEq
                | Self::GtEq
                | Self::Lt
                | Self::Gt
                | Self::Eq
        )
    }

    /// 检查是否为标点符号
    pub fn is_punctuation(self) -> bool {
        matches!(
            self,
            Self::LeftParen
                | Self::RightParen
                | Self::LeftBrace
                | Self::RightBrace
                | Self::LeftBracket
                | Self::RightBracket
                | Self::ColonColon
                | Self::Semicolon
                | Self::Colon
                | Self::Comma
                | Self::Dot
                | Self::DotDot
                | Self::DotDotDot
        )
    }

    /// 检查是否为字面
    pub fn is_literal(self) -> bool {
        matches!(self, Self::Number | Self::String | Self::True | Self::False | Self::Nil)
    }

    /// 检查是否为琐碎内容（空白、注释等
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

impl fmt::Display for LuaSyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Root => "ROOT",
            Self::SourceFile => "SOURCE_FILE",
            Self::And => "and",
            Self::Break => "break",
            Self::Do => "do",
            Self::Else => "else",
            Self::Elseif => "elseif",
            Self::End => "end",
            Self::False => "false",
            Self::For => "for",
            Self::Function => "function",
            Self::Goto => "goto",
            Self::If => "if",
            Self::In => "in",
            Self::Local => "local",
            Self::Nil => "nil",
            Self::Not => "not",
            Self::Or => "or",
            Self::Repeat => "repeat",
            Self::Return => "return",
            Self::Then => "then",
            Self::True => "true",
            Self::Until => "until",
            Self::While => "while",
            Self::Identifier => "IDENTIFIER",
            Self::Number => "NUMBER",
            Self::String => "STRING",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Star => "*",
            Self::Slash => "/",
            Self::Percent => "%",
            Self::Caret => "^",
            Self::Hash => "#",
            Self::Ampersand => "&",
            Self::Tilde => "~",
            Self::Pipe => "|",
            Self::LtLt => "<<",
            Self::GtGt => ">>",
            Self::SlashSlash => "//",
            Self::EqEq => "==",
            Self::TildeEq => "~=",
            Self::LtEq => "<=",
            Self::GtEq => ">=",
            Self::Lt => "<",
            Self::Gt => ">",
            Self::Eq => "=",
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
            Self::LeftBracket => "[",
            Self::RightBracket => "]",
            Self::ColonColon => "::",
            Self::Semicolon => ";",
            Self::Colon => ":",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::DotDot => "..",
            Self::DotDotDot => "...",
            Self::Comment => "COMMENT",
            Self::Whitespace => "WHITESPACE",
            Self::Error => "ERROR",
            Self::Eof => "EOF",
        };
        write!(f, "{}", name)
    }
}

impl SyntaxKind for LuaSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Comment | Self::Whitespace)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Root | Self::SourceFile)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Root | Self::SourceFile)
    }
}
