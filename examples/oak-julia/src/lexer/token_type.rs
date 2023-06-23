use core::{fmt, str::FromStr};
use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type JuliaToken = Token<JuliaTokenType>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
#[repr(u8)]
pub enum JuliaTokenType {
    Root,
    // 关键字
    If,
    ElseIf,
    Else,
    For,
    While,
    In,
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
    Call,
    ArgumentList,
    Comment,
    Whitespace,
    Newline,
    Eof,
    Error,
    Invalid,
}

impl FromStr for JuliaTokenType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "if" => Ok(JuliaTokenType::If),
            "elseif" => Ok(JuliaTokenType::ElseIf),
            "else" => Ok(JuliaTokenType::Else),
            "for" => Ok(JuliaTokenType::For),
            "while" => Ok(JuliaTokenType::While),
            "in" => Ok(JuliaTokenType::In),
            "break" => Ok(JuliaTokenType::Break),
            "continue" => Ok(JuliaTokenType::Continue),
            "function" => Ok(JuliaTokenType::Function),
            "end" => Ok(JuliaTokenType::End),
            "begin" => Ok(JuliaTokenType::Begin),
            "module" => Ok(JuliaTokenType::Module),
            "using" => Ok(JuliaTokenType::Using),
            "import" => Ok(JuliaTokenType::Import),
            "export" => Ok(JuliaTokenType::Export),
            "const" => Ok(JuliaTokenType::Const),
            "local" => Ok(JuliaTokenType::Local),
            "global" => Ok(JuliaTokenType::Global),
            "true" => Ok(JuliaTokenType::True),
            "false" => Ok(JuliaTokenType::False),
            "nothing" => Ok(JuliaTokenType::Nothing),
            "return" => Ok(JuliaTokenType::Return),
            _ => Err(()),
        }
    }
}

impl JuliaTokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            JuliaTokenType::Root => "root",
            JuliaTokenType::If => "if",
            JuliaTokenType::ElseIf => "elseif",
            JuliaTokenType::Else => "else",
            JuliaTokenType::For => "for",
            JuliaTokenType::While => "while",
            JuliaTokenType::In => "in",
            JuliaTokenType::Break => "break",
            JuliaTokenType::Continue => "continue",
            JuliaTokenType::Function => "function",
            JuliaTokenType::End => "end",
            JuliaTokenType::Begin => "begin",
            JuliaTokenType::Module => "module",
            JuliaTokenType::Using => "using",
            JuliaTokenType::Import => "import",
            JuliaTokenType::Export => "export",
            JuliaTokenType::Const => "const",
            JuliaTokenType::Local => "local",
            JuliaTokenType::Global => "global",
            JuliaTokenType::True => "true",
            JuliaTokenType::False => "false",
            JuliaTokenType::Nothing => "nothing",
            JuliaTokenType::Return => "return",
            JuliaTokenType::Plus => "+",
            JuliaTokenType::Minus => "-",
            JuliaTokenType::Star => "*",
            JuliaTokenType::Slash => "/",
            JuliaTokenType::Percent => "%",
            JuliaTokenType::Caret => "^",
            JuliaTokenType::Equal => "==",
            JuliaTokenType::NotEqual => "!=",
            JuliaTokenType::LessThan => "<",
            JuliaTokenType::GreaterThan => ">",
            JuliaTokenType::LessEqual => "<=",
            JuliaTokenType::GreaterEqual => ">=",
            JuliaTokenType::Assign => "=",
            JuliaTokenType::PlusAssign => "+=",
            JuliaTokenType::MinusAssign => "-=",
            JuliaTokenType::StarAssign => "*=",
            JuliaTokenType::SlashAssign => "/=",
            JuliaTokenType::PercentAssign => "%=",
            JuliaTokenType::CaretAssign => "^=",
            JuliaTokenType::And => "&&",
            JuliaTokenType::Or => "||",
            JuliaTokenType::Not => "!",
            JuliaTokenType::Colon => ":",
            JuliaTokenType::Dot => ".",
            JuliaTokenType::Range => "..",
            JuliaTokenType::Arrow => "->",
            JuliaTokenType::FatArrow => "=>",
            JuliaTokenType::BitAnd => "&",
            JuliaTokenType::BitOr => "|",
            JuliaTokenType::BitXor => "^",
            JuliaTokenType::BitNot => "~",
            JuliaTokenType::LeftShift => "<<",
            JuliaTokenType::RightShift => ">>",
            JuliaTokenType::LeftParen => "(",
            JuliaTokenType::RightParen => ")",
            JuliaTokenType::LeftBracket => "[",
            JuliaTokenType::RightBracket => "]",
            JuliaTokenType::LeftBrace => "{",
            JuliaTokenType::RightBrace => "}",
            JuliaTokenType::Comma => ",",
            JuliaTokenType::Semicolon => ";",
            JuliaTokenType::IntegerLiteral => "integer",
            JuliaTokenType::FloatLiteral => "float",
            JuliaTokenType::StringLiteral => "string",
            JuliaTokenType::CharLiteral => "char",
            JuliaTokenType::BooleanLiteral => "boolean",
            JuliaTokenType::NothingLiteral => "nothing_lit",
            JuliaTokenType::Identifier => "identifier",
            JuliaTokenType::Call => "call",
            JuliaTokenType::ArgumentList => "argument_list",
            JuliaTokenType::Comment => "comment",
            JuliaTokenType::Whitespace => "whitespace",
            JuliaTokenType::Newline => "newline",
            JuliaTokenType::Eof => "eof",
            JuliaTokenType::Error => "error",
            JuliaTokenType::Invalid => "invalid",
        }
    }

    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }
}

impl fmt::Display for JuliaTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TokenType for JuliaTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}
