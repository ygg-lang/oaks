use oak_core::{ElementType, UniversalElementRole};
use std::{fmt, str::FromStr};

/// Element types for the Julia language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JuliaElementType {
    /// Root node of the parse tree.
    Root,
    // Keywords
    /// `if` keyword.
    If,
    /// `elseif` keyword.
    ElseIf,
    /// `else` keyword.
    Else,
    /// `for` keyword.
    For,
    /// `while` keyword.
    While,
    /// `break` keyword.
    Break,
    /// `continue` keyword.
    Continue,
    /// `function` keyword.
    Function,
    /// `end` keyword.
    End,
    /// `begin` keyword.
    Begin,
    /// `module` keyword.
    Module,
    /// `using` keyword.
    Using,
    /// `import` keyword.
    Import,
    /// `export` keyword.
    Export,
    /// `const` keyword.
    Const,
    /// `local` keyword.
    Local,
    /// `global` keyword.
    Global,
    /// `true` keyword.
    True,
    /// `false` keyword.
    False,
    /// `nothing` keyword.
    Nothing,
    /// `return` keyword.
    Return,

    // Operators
    /// `+` operator.
    Plus,
    /// `-` operator.
    Minus,
    /// `*` operator.
    Star,
    /// `/` operator.
    Slash,
    /// `%` operator.
    Percent,
    /// `^` operator.
    Caret,
    /// `==` operator.
    Equal,
    /// `!=` operator.
    NotEqual,
    /// `<` operator.
    LessThan,
    /// `>` operator.
    GreaterThan,
    /// `<=` operator.
    LessEqual,
    /// `>=` operator.
    GreaterEqual,
    /// `=` operator.
    Assign,
    /// `+=` operator.
    PlusAssign,
    /// `-=` operator.
    MinusAssign,
    /// `*=` operator.
    StarAssign,
    /// `/=` operator.
    SlashAssign,
    /// `%=` operator.
    PercentAssign,
    /// `^=` operator.
    CaretAssign,
    /// `&&` operator.
    And,
    /// `||` operator.
    Or,
    /// `!` operator.
    Not,
    /// `:` operator.
    Colon,
    /// `.` operator.
    Dot,
    /// `:` operator (range).
    Range,
    /// `->` operator.
    Arrow,
    /// `=>` operator.
    FatArrow,
    /// `&` operator.
    BitAnd,
    /// `|` operator.
    BitOr,
    /// `xor` or `⊻` operator.
    BitXor,
    /// `~` operator.
    BitNot,
    /// `<<` operator.
    LeftShift,
    /// `>>` operator.
    RightShift,

    // Delimiters
    /// `(` delimiter.
    LeftParen,
    /// `)` delimiter.
    RightParen,
    /// `[` delimiter.
    LeftBracket,
    /// `]` delimiter.
    RightBracket,
    /// `{` delimiter.
    LeftBrace,
    /// `}` delimiter.
    RightBrace,
    /// `,` delimiter.
    Comma,
    /// `;` delimiter.
    Semicolon,

    // Literals
    /// Integer literal.
    IntegerLiteral,
    /// Floating-point literal.
    FloatLiteral,
    /// String literal.
    StringLiteral,
    /// Character literal.
    CharLiteral,
    /// Boolean literal.
    BooleanLiteral,
    /// Nothing literal.
    NothingLiteral,

    // Other
    /// Identifier.
    Identifier,
    /// Function call.
    Call,
    /// List of arguments.
    ArgumentList,
    /// Comment.
    Comment,
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// End of file.
    Eof,
    /// Error element.
    Error,
    /// Invalid element.
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
    /// Returns the string representation of the element type.
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

    /// Returns true if the element is a trivia element (whitespace, comment, or newline).
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
                match token {
            crate::lexer::token_type::JuliaTokenType::Root => Self::Root,
            crate::lexer::token_type::JuliaTokenType::If => Self::If,
            crate::lexer::token_type::JuliaTokenType::ElseIf => Self::ElseIf,
            crate::lexer::token_type::JuliaTokenType::Else => Self::Else,
            crate::lexer::token_type::JuliaTokenType::For => Self::For,
            crate::lexer::token_type::JuliaTokenType::While => Self::While,
            crate::lexer::token_type::JuliaTokenType::In => Self::Error,
            crate::lexer::token_type::JuliaTokenType::Break => Self::Break,
            crate::lexer::token_type::JuliaTokenType::Continue => Self::Continue,
            crate::lexer::token_type::JuliaTokenType::Function => Self::Function,
            crate::lexer::token_type::JuliaTokenType::End => Self::End,
            crate::lexer::token_type::JuliaTokenType::Begin => Self::Begin,
            crate::lexer::token_type::JuliaTokenType::Module => Self::Module,
            crate::lexer::token_type::JuliaTokenType::Using => Self::Using,
            crate::lexer::token_type::JuliaTokenType::Import => Self::Import,
            crate::lexer::token_type::JuliaTokenType::Export => Self::Export,
            crate::lexer::token_type::JuliaTokenType::Const => Self::Const,
            crate::lexer::token_type::JuliaTokenType::Local => Self::Local,
            crate::lexer::token_type::JuliaTokenType::Global => Self::Global,
            crate::lexer::token_type::JuliaTokenType::True => Self::True,
            crate::lexer::token_type::JuliaTokenType::False => Self::False,
            crate::lexer::token_type::JuliaTokenType::Nothing => Self::Nothing,
            crate::lexer::token_type::JuliaTokenType::Return => Self::Return,
            crate::lexer::token_type::JuliaTokenType::Plus => Self::Plus,
            crate::lexer::token_type::JuliaTokenType::Minus => Self::Minus,
            crate::lexer::token_type::JuliaTokenType::Star => Self::Star,
            crate::lexer::token_type::JuliaTokenType::Slash => Self::Slash,
            crate::lexer::token_type::JuliaTokenType::Percent => Self::Percent,
            crate::lexer::token_type::JuliaTokenType::Caret => Self::Caret,
            crate::lexer::token_type::JuliaTokenType::Equal => Self::Equal,
            crate::lexer::token_type::JuliaTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::JuliaTokenType::LessThan => Self::LessThan,
            crate::lexer::token_type::JuliaTokenType::GreaterThan => Self::GreaterThan,
            crate::lexer::token_type::JuliaTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::JuliaTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::JuliaTokenType::Assign => Self::Assign,
            crate::lexer::token_type::JuliaTokenType::PlusAssign => Self::PlusAssign,
            crate::lexer::token_type::JuliaTokenType::MinusAssign => Self::MinusAssign,
            crate::lexer::token_type::JuliaTokenType::StarAssign => Self::StarAssign,
            crate::lexer::token_type::JuliaTokenType::SlashAssign => Self::SlashAssign,
            crate::lexer::token_type::JuliaTokenType::PercentAssign => Self::PercentAssign,
            crate::lexer::token_type::JuliaTokenType::CaretAssign => Self::CaretAssign,
            crate::lexer::token_type::JuliaTokenType::And => Self::And,
            crate::lexer::token_type::JuliaTokenType::Or => Self::Or,
            crate::lexer::token_type::JuliaTokenType::Not => Self::Not,
            crate::lexer::token_type::JuliaTokenType::Colon => Self::Colon,
            crate::lexer::token_type::JuliaTokenType::Dot => Self::Dot,
            crate::lexer::token_type::JuliaTokenType::Range => Self::Range,
            crate::lexer::token_type::JuliaTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::JuliaTokenType::FatArrow => Self::FatArrow,
            crate::lexer::token_type::JuliaTokenType::BitAnd => Self::BitAnd,
            crate::lexer::token_type::JuliaTokenType::BitOr => Self::BitOr,
            crate::lexer::token_type::JuliaTokenType::BitXor => Self::BitXor,
            crate::lexer::token_type::JuliaTokenType::BitNot => Self::BitNot,
            crate::lexer::token_type::JuliaTokenType::LeftShift => Self::LeftShift,
            crate::lexer::token_type::JuliaTokenType::RightShift => Self::RightShift,
            crate::lexer::token_type::JuliaTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::JuliaTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::JuliaTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::JuliaTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::JuliaTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::JuliaTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::JuliaTokenType::Comma => Self::Comma,
            crate::lexer::token_type::JuliaTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::JuliaTokenType::IntegerLiteral => Self::IntegerLiteral,
            crate::lexer::token_type::JuliaTokenType::FloatLiteral => Self::FloatLiteral,
            crate::lexer::token_type::JuliaTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::JuliaTokenType::CharLiteral => Self::CharLiteral,
            crate::lexer::token_type::JuliaTokenType::BooleanLiteral => Self::BooleanLiteral,
            crate::lexer::token_type::JuliaTokenType::NothingLiteral => Self::NothingLiteral,
            crate::lexer::token_type::JuliaTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::JuliaTokenType::Call => Self::Call,
            crate::lexer::token_type::JuliaTokenType::ArgumentList => Self::ArgumentList,
            crate::lexer::token_type::JuliaTokenType::Comment => Self::Comment,
            crate::lexer::token_type::JuliaTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::JuliaTokenType::Newline => Self::Newline,
            crate::lexer::token_type::JuliaTokenType::Eof => Self::Eof,
            crate::lexer::token_type::JuliaTokenType::Error => Self::Error,
            crate::lexer::token_type::JuliaTokenType::Invalid => Self::Invalid,
            _ => Self::Error,
        }
    }
}
