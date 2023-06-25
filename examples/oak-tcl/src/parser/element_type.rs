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
        unsafe { std::mem::transmute(token) }
    }
}
