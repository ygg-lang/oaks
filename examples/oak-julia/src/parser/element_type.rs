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
        unsafe { std::mem::transmute(token) }
    }
}
