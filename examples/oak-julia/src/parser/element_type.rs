use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JuliaElementType {
    Root,
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
    Call,
    ArgumentList,
    Comment,
    Whitespace,
    Newline,
    Eof,
    Error,
    Invalid,
}

impl FromStr for JuliaElementType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "if" => Ok(JuliaElementType::If),
            "elseif" => Ok(JuliaElementType::ElseIf),
            "else" => Ok(JuliaElementType::Else),
            "for" => Ok(JuliaElementType::For),
            "while" => Ok(JuliaElementType::While),
            "break" => Ok(JuliaElementType::Break),
            "continue" => Ok(JuliaElementType::Continue),
            "function" => Ok(JuliaElementType::Function),
            "end" => Ok(JuliaElementType::End),
            "begin" => Ok(JuliaElementType::Begin),
            "module" => Ok(JuliaElementType::Module),
            "using" => Ok(JuliaElementType::Using),
            "import" => Ok(JuliaElementType::Import),
            "export" => Ok(JuliaElementType::Export),
            "const" => Ok(JuliaElementType::Const),
            "local" => Ok(JuliaElementType::Local),
            "global" => Ok(JuliaElementType::Global),
            "true" => Ok(JuliaElementType::True),
            "false" => Ok(JuliaElementType::False),
            "nothing" => Ok(JuliaElementType::Nothing),
            "return" => Ok(JuliaElementType::Return),
            _ => Err(()),
        }
    }
}

impl JuliaElementType {
    pub fn as_str(&self) -> &'static str {
        match self {
            JuliaElementType::Root => "root",
            JuliaElementType::If => "if",
            JuliaElementType::ElseIf => "elseif",
            JuliaElementType::Else => "else",
            JuliaElementType::For => "for",
            JuliaElementType::While => "while",
            JuliaElementType::Break => "break",
            JuliaElementType::Continue => "continue",
            JuliaElementType::Function => "function",
            JuliaElementType::End => "end",
            JuliaElementType::Begin => "begin",
            JuliaElementType::Module => "module",
            JuliaElementType::Using => "using",
            JuliaElementType::Import => "import",
            JuliaElementType::Export => "export",
            JuliaElementType::Const => "const",
            JuliaElementType::Local => "local",
            JuliaElementType::Global => "global",
            JuliaElementType::True => "true",
            JuliaElementType::False => "false",
            JuliaElementType::Nothing => "nothing",
            JuliaElementType::Return => "return",
            JuliaElementType::Plus => "+",
            JuliaElementType::Minus => "-",
            JuliaElementType::Star => "*",
            JuliaElementType::Slash => "/",
            JuliaElementType::Percent => "%",
            JuliaElementType::Caret => "^",
            JuliaElementType::Equal => "==",
            JuliaElementType::NotEqual => "!=",
            JuliaElementType::LessThan => "<",
            JuliaElementType::GreaterThan => ">",
            JuliaElementType::LessEqual => "<=",
            JuliaElementType::GreaterEqual => ">=",
            JuliaElementType::Assign => "=",
            JuliaElementType::PlusAssign => "+=",
            JuliaElementType::MinusAssign => "-=",
            JuliaElementType::StarAssign => "*=",
            JuliaElementType::SlashAssign => "/=",
            JuliaElementType::PercentAssign => "%=",
            JuliaElementType::CaretAssign => "^=",
            JuliaElementType::And => "&&",
            JuliaElementType::Or => "||",
            JuliaElementType::Not => "!",
            JuliaElementType::Colon => ":",
            JuliaElementType::Dot => ".",
            JuliaElementType::Range => "..",
            JuliaElementType::Arrow => "->",
            JuliaElementType::FatArrow => "=>",
            JuliaElementType::BitAnd => "&",
            JuliaElementType::BitOr => "|",
            JuliaElementType::BitXor => "^",
            JuliaElementType::BitNot => "~",
            JuliaElementType::LeftShift => "<<",
            JuliaElementType::RightShift => ">>",
            JuliaElementType::LeftParen => "(",
            JuliaElementType::RightParen => ")",
            JuliaElementType::LeftBracket => "[",
            JuliaElementType::RightBracket => "]",
            JuliaElementType::LeftBrace => "{",
            JuliaElementType::RightBrace => "}",
            JuliaElementType::Comma => ",",
            JuliaElementType::Semicolon => ";",
            JuliaElementType::IntegerLiteral => "integer",
            JuliaElementType::FloatLiteral => "float",
            JuliaElementType::StringLiteral => "string",
            JuliaElementType::CharLiteral => "char",
            JuliaElementType::BooleanLiteral => "boolean",
            JuliaElementType::NothingLiteral => "nothing_lit",
            JuliaElementType::Identifier => "identifier",
            JuliaElementType::Call => "call",
            JuliaElementType::ArgumentList => "argument_list",
            JuliaElementType::Comment => "comment",
            JuliaElementType::Whitespace => "whitespace",
            JuliaElementType::Newline => "newline",
            JuliaElementType::Eof => "eof",
            JuliaElementType::Error => "error",
            JuliaElementType::Invalid => "invalid",
        }
    }

    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }
}

impl fmt::Display for JuliaElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ElementType for JuliaElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::JuliaTokenType> for JuliaElementType {
    fn from(token: crate::lexer::token_type::JuliaTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
