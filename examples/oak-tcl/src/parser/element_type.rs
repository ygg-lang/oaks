use oak_core::{ElementType, Parser, UniversalElementRole};

/// Tcl element type definition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TclElementType {
    /// Root node
    Root,
    /// Command
    Command,
    /// Proc definition
    ProcDefinition,
    /// If command
    IfCommand,
    /// While command
    WhileCommand,
    /// For command
    ForCommand,
    /// ForEach command
    ForEachCommand,
    /// Set command
    SetCommand,
    /// Word
    Word,
    /// Simple word
    SimpleWord,
    /// Variable word
    VariableWord,
    /// Script word
    ScriptWord,
    /// Braced word
    BracedWord,

    /// Number
    Number,
    /// String literal
    StringLiteral,
    /// Identifier
    Identifier,

    /// if keyword
    If,
    /// else keyword
    Else,
    /// elseif keyword
    ElseIf,
    /// for keyword
    For,
    /// while keyword
    While,
    /// foreach keyword
    ForEach,
    /// proc keyword
    Proc,
    /// return keyword
    Return,
    /// break keyword
    Break,
    /// continue keyword
    Continue,
    /// set keyword
    Set,
    /// unset keyword
    Unset,
    /// global keyword
    Global,
    /// upvar keyword
    Upvar,
    /// variable keyword
    Variable,

    /// Plus (+)
    Plus,
    /// Minus (-)
    Minus,
    /// Star (*)
    Star,
    /// Slash (/)
    Slash,
    /// Percent (%)
    Percent,
    /// Equal (=)
    Equal,
    /// Not equal (!=)
    NotEqual,
    /// Less (<)
    Less,
    /// Greater (>)
    Greater,
    /// Less equal (<=)
    LessEqual,
    /// Greater equal (>=)
    GreaterEqual,
    /// Ampersand (&)
    Ampersand,
    /// Logical AND (&&)
    AmpersandAmpersand,
    /// Pipe (|)
    Pipe,
    /// Logical OR (||)
    PipePipe,
    /// Exclamation (!)
    Exclamation,

    /// Left parenthesis (()
    LeftParen,
    /// Right parenthesis ())
    RightParen,
    /// Left bracket ([)
    LeftBracket,
    /// Right bracket (])
    RightBracket,
    /// Left brace ({)
    LeftBrace,
    /// Right brace (})
    RightBrace,
    /// Semicolon (;)
    Semicolon,
    /// Comma (,)
    Comma,
    /// Dollar ($)
    Dollar,

    /// Whitespace
    Whitespace,
    /// Newline
    Newline,
    /// Comment
    Comment,
    /// Error
    Error,
    /// End of file
    Eof,
}

impl ElementType for TclElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            TclElementType::Root => UniversalElementRole::Root,
            TclElementType::Command => UniversalElementRole::Expression,
            TclElementType::Word | TclElementType::SimpleWord | TclElementType::VariableWord | TclElementType::ScriptWord | TclElementType::BracedWord => UniversalElementRole::Expression,
            TclElementType::Identifier => UniversalElementRole::Name,
            TclElementType::Number | TclElementType::StringLiteral => UniversalElementRole::Value,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::TclTokenType> for TclElementType {
    fn from(token: crate::lexer::token_type::TclTokenType) -> Self {
                match token {
            crate::lexer::token_type::TclTokenType::Root => Self::Root,
            crate::lexer::token_type::TclTokenType::Command => Self::Command,
            crate::lexer::token_type::TclTokenType::Word => Self::Word,
            crate::lexer::token_type::TclTokenType::SimpleWord => Self::SimpleWord,
            crate::lexer::token_type::TclTokenType::VariableWord => Self::VariableWord,
            crate::lexer::token_type::TclTokenType::ScriptWord => Self::ScriptWord,
            crate::lexer::token_type::TclTokenType::BracedWord => Self::BracedWord,
            crate::lexer::token_type::TclTokenType::Number => Self::Number,
            crate::lexer::token_type::TclTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::TclTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::TclTokenType::If => Self::If,
            crate::lexer::token_type::TclTokenType::Else => Self::Else,
            crate::lexer::token_type::TclTokenType::ElseIf => Self::ElseIf,
            crate::lexer::token_type::TclTokenType::For => Self::For,
            crate::lexer::token_type::TclTokenType::While => Self::While,
            crate::lexer::token_type::TclTokenType::ForEach => Self::ForEach,
            crate::lexer::token_type::TclTokenType::Proc => Self::Proc,
            crate::lexer::token_type::TclTokenType::Return => Self::Return,
            crate::lexer::token_type::TclTokenType::Break => Self::Break,
            crate::lexer::token_type::TclTokenType::Continue => Self::Continue,
            crate::lexer::token_type::TclTokenType::Set => Self::Set,
            crate::lexer::token_type::TclTokenType::Unset => Self::Unset,
            crate::lexer::token_type::TclTokenType::Global => Self::Global,
            crate::lexer::token_type::TclTokenType::Upvar => Self::Upvar,
            crate::lexer::token_type::TclTokenType::Variable => Self::Variable,
            crate::lexer::token_type::TclTokenType::Plus => Self::Plus,
            crate::lexer::token_type::TclTokenType::Minus => Self::Minus,
            crate::lexer::token_type::TclTokenType::Star => Self::Star,
            crate::lexer::token_type::TclTokenType::Slash => Self::Slash,
            crate::lexer::token_type::TclTokenType::Percent => Self::Percent,
            crate::lexer::token_type::TclTokenType::Equal => Self::Equal,
            crate::lexer::token_type::TclTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::TclTokenType::Less => Self::Less,
            crate::lexer::token_type::TclTokenType::Greater => Self::Greater,
            crate::lexer::token_type::TclTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::TclTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::TclTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::TclTokenType::AmpersandAmpersand => Self::AmpersandAmpersand,
            crate::lexer::token_type::TclTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::TclTokenType::PipePipe => Self::PipePipe,
            crate::lexer::token_type::TclTokenType::Exclamation => Self::Exclamation,
            crate::lexer::token_type::TclTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::TclTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::TclTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::TclTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::TclTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::TclTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::TclTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::TclTokenType::Comma => Self::Comma,
            crate::lexer::token_type::TclTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::TclTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::TclTokenType::Newline => Self::Newline,
            crate::lexer::token_type::TclTokenType::Comment => Self::Comment,
            crate::lexer::token_type::TclTokenType::Error => Self::Error,
            crate::lexer::token_type::TclTokenType::Eof => Self::Eof,
            _ => Self::Error,
        }
    }
}
