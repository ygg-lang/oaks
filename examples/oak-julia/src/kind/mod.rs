use core::{fmt, range::Range};
use oak_core::SyntaxKind;

/// Julia 令牌
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaToken {
    pub kind: JuliaSyntaxKind,
    pub span: Range<usize>,
}

use core::str::FromStr;

/// Julia 令牌种类
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum JuliaSyntaxKind {
    // 关键字
    If,
    ElseIf,
    Else,
    For,
    While,
    Break,
    Continue,
    Function,
    End,
    Begin,
    Module,
    Using,
    Import,
    Export,
    Const,
    Local,
    Global,
    True,
    False,
    Nothing,
    Return,

    // 操作符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    CaretAssign,
    And,
    Or,
    Not,
    Colon,
    Dot,
    Range,
    Arrow,
    FatArrow,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    LeftShift,
    RightShift,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,

    // 字面量
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BooleanLiteral,
    NothingLiteral,

    // 其他
    Identifier,
    Comment,
    Whitespace,
    Newline,
    Eof,
    Error,
    Invalid,
}

impl FromStr for JuliaSyntaxKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "if" => Ok(JuliaSyntaxKind::If),
            "elseif" => Ok(JuliaSyntaxKind::ElseIf),
            "else" => Ok(JuliaSyntaxKind::Else),
            "for" => Ok(JuliaSyntaxKind::For),
            "while" => Ok(JuliaSyntaxKind::While),
            "break" => Ok(JuliaSyntaxKind::Break),
            "continue" => Ok(JuliaSyntaxKind::Continue),
            "function" => Ok(JuliaSyntaxKind::Function),
            "end" => Ok(JuliaSyntaxKind::End),
            "begin" => Ok(JuliaSyntaxKind::Begin),
            "module" => Ok(JuliaSyntaxKind::Module),
            "using" => Ok(JuliaSyntaxKind::Using),
            "import" => Ok(JuliaSyntaxKind::Import),
            "export" => Ok(JuliaSyntaxKind::Export),
            "const" => Ok(JuliaSyntaxKind::Const),
            "local" => Ok(JuliaSyntaxKind::Local),
            "global" => Ok(JuliaSyntaxKind::Global),
            "true" => Ok(JuliaSyntaxKind::True),
            "false" => Ok(JuliaSyntaxKind::False),
            "nothing" => Ok(JuliaSyntaxKind::Nothing),
            "return" => Ok(JuliaSyntaxKind::Return),
            _ => Err(()),
        }
    }
}

impl JuliaSyntaxKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            JuliaSyntaxKind::If => "if",
            JuliaSyntaxKind::ElseIf => "elseif",
            JuliaSyntaxKind::Else => "else",
            JuliaSyntaxKind::For => "for",
            JuliaSyntaxKind::While => "while",
            JuliaSyntaxKind::Break => "break",
            JuliaSyntaxKind::Continue => "continue",
            JuliaSyntaxKind::Function => "function",
            JuliaSyntaxKind::End => "end",
            JuliaSyntaxKind::Begin => "begin",
            JuliaSyntaxKind::Module => "module",
            JuliaSyntaxKind::Using => "using",
            JuliaSyntaxKind::Import => "import",
            JuliaSyntaxKind::Export => "export",
            JuliaSyntaxKind::Const => "const",
            JuliaSyntaxKind::Local => "local",
            JuliaSyntaxKind::Global => "global",
            JuliaSyntaxKind::True => "true",
            JuliaSyntaxKind::False => "false",
            JuliaSyntaxKind::Nothing => "nothing",
            JuliaSyntaxKind::Return => "return",
            JuliaSyntaxKind::Plus => "+",
            JuliaSyntaxKind::Minus => "-",
            JuliaSyntaxKind::Star => "*",
            JuliaSyntaxKind::Slash => "/",
            JuliaSyntaxKind::Percent => "%",
            JuliaSyntaxKind::Caret => "^",
            JuliaSyntaxKind::Equal => "==",
            JuliaSyntaxKind::NotEqual => "!=",
            JuliaSyntaxKind::LessThan => "<",
            JuliaSyntaxKind::GreaterThan => ">",
            JuliaSyntaxKind::LessEqual => "<=",
            JuliaSyntaxKind::GreaterEqual => ">=",
            JuliaSyntaxKind::Assign => "=",
            JuliaSyntaxKind::PlusAssign => "+=",
            JuliaSyntaxKind::MinusAssign => "-=",
            JuliaSyntaxKind::StarAssign => "*=",
            JuliaSyntaxKind::SlashAssign => "/=",
            JuliaSyntaxKind::PercentAssign => "%=",
            JuliaSyntaxKind::CaretAssign => "^=",
            JuliaSyntaxKind::And => "&&",
            JuliaSyntaxKind::Or => "||",
            JuliaSyntaxKind::Not => "!",
            JuliaSyntaxKind::Colon => ":",
            JuliaSyntaxKind::Dot => ".",
            JuliaSyntaxKind::Range => "..",
            JuliaSyntaxKind::Arrow => "->",
            JuliaSyntaxKind::FatArrow => "=>",
            JuliaSyntaxKind::BitAnd => "&",
            JuliaSyntaxKind::BitOr => "|",
            JuliaSyntaxKind::BitXor => "^",
            JuliaSyntaxKind::BitNot => "~",
            JuliaSyntaxKind::LeftShift => "<<",
            JuliaSyntaxKind::RightShift => ">>",
            JuliaSyntaxKind::LeftParen => "(",
            JuliaSyntaxKind::RightParen => ")",
            JuliaSyntaxKind::LeftBracket => "[",
            JuliaSyntaxKind::RightBracket => "]",
            JuliaSyntaxKind::LeftBrace => "{",
            JuliaSyntaxKind::RightBrace => "}",
            JuliaSyntaxKind::Comma => ",",
            JuliaSyntaxKind::Semicolon => ";",
            JuliaSyntaxKind::IntegerLiteral => "integer",
            JuliaSyntaxKind::FloatLiteral => "float",
            JuliaSyntaxKind::StringLiteral => "string",
            JuliaSyntaxKind::CharLiteral => "char",
            JuliaSyntaxKind::BooleanLiteral => "boolean",
            JuliaSyntaxKind::NothingLiteral => "nothing",
            JuliaSyntaxKind::Identifier => "identifier",
            JuliaSyntaxKind::Comment => "comment",
            JuliaSyntaxKind::Whitespace => "whitespace",
            JuliaSyntaxKind::Newline => "newline",
            JuliaSyntaxKind::Eof => "eof",
            JuliaSyntaxKind::Error => "error",
            JuliaSyntaxKind::Invalid => "invalid",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "if" => Some(Self::If),
            "elseif" => Some(Self::ElseIf),
            "else" => Some(Self::Else),
            "for" => Some(Self::For),
            "while" => Some(Self::While),
            "break" => Some(Self::Break),
            "continue" => Some(Self::Continue),
            "function" => Some(Self::Function),
            "end" => Some(Self::End),
            "begin" => Some(Self::Begin),
            "module" => Some(Self::Module),
            "using" => Some(Self::Using),
            "import" => Some(Self::Import),
            "export" => Some(Self::Export),
            "const" => Some(Self::Const),
            "local" => Some(Self::Local),
            "global" => Some(Self::Global),
            "true" => Some(Self::True),
            "false" => Some(Self::False),
            "nothing" => Some(Self::Nothing),
            "return" => Some(Self::Return),
            _ => None,
        }
    }
}

impl fmt::Display for JuliaSyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl SyntaxKind for JuliaSyntaxKind {
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
        true // Julia doesn't have element types in this simple implementation
    }

    fn is_element_type(&self) -> bool {
        false // Julia doesn't have element types in this simple implementation
    }
}
