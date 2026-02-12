use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Matlab language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum MatlabElementType {
    // Basic tokens (consistent with MatlabTokenType)
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Line comment.
    Comment,
    /// Block comment.
    BlockComment,

    // Identifiers and literals
    /// Identifier.
    Identifier,
    /// Number literal.
    Number,
    /// String literal.
    String,
    /// Character literal.
    Character,

    // Keywords
    /// `function` keyword.
    Function,
    /// `end` keyword.
    End,
    /// `if` keyword.
    If,
    /// `else` keyword.
    Else,
    /// `elseif` keyword.
    Elseif,
    /// `while` keyword.
    While,
    /// `for` keyword.
    For,
    /// `break` keyword.
    Break,
    /// `continue` keyword.
    Continue,
    /// `return` keyword.
    Return,
    /// `switch` keyword.
    Switch,
    /// `case` keyword.
    Case,
    /// `otherwise` keyword.
    Otherwise,
    /// `try` keyword.
    Try,
    /// `catch` keyword.
    Catch,
    /// `global` keyword.
    Global,
    /// `persistent` keyword.
    Persistent,
    /// `classdef` keyword.
    Classdef,
    /// `properties` keyword.
    Properties,
    /// `methods` keyword.
    Methods,
    /// `events` keyword.
    Events,

    // Operators
    /// `+` operator.
    Plus, // +
    /// `-` operator.
    Minus, // -
    /// `*` operator.
    Times, // *
    /// `/` operator.
    Divide, // /
    /// `^` operator.
    Power, // ^
    /// `\` operator.
    LeftDivide, // \
    /// `.*` operator.
    DotTimes, // .*
    /// `./` operator.
    DotDivide, // ./
    /// `.^` operator.
    DotPower, // .^
    /// `.\` operator.
    DotLeftDivide, // .\

    // Comparison operators
    /// `==` operator.
    Equal, // ==
    /// `~=` operator.
    NotEqual, // ~=
    /// `<` operator.
    Less, // <
    /// `>` operator.
    Greater, // >
    /// `<=` operator.
    LessEqual, // <=
    /// `>=` operator.
    GreaterEqual, // >=

    // Logical operators
    /// `&` operator.
    And, // &
    /// `|` operator.
    Or, // |
    /// `~` operator.
    Not, // ~
    /// `&&` operator.
    AndAnd, // &&
    /// `||` operator.
    OrOr, // ||

    // Assignment operators
    /// `=` operator.
    Assign, // =

    // Delimiters
    /// `(` delimiter.
    LeftParen, // (
    /// `)` delimiter.
    RightParen, // )
    /// `[` delimiter.
    LeftBracket, // [
    /// `]` delimiter.
    RightBracket, // ]
    /// `{` delimiter.
    LeftBrace, // {
    /// `}` delimiter.
    RightBrace, // }
    /// `;` delimiter.
    Semicolon, // ;
    /// `,` delimiter.
    Comma, // ,
    /// `.` delimiter.
    Dot, // .
    /// `:` delimiter.
    Colon, // :
    /// `?` delimiter.
    Question, // ?
    /// `@` delimiter.
    At, // @

    // Special operators
    /// `'` operator.
    Transpose, // '
    /// `.'` operator.
    DotTranspose, // .'

    // Generalized types
    /// General operator.
    Operator,
    /// General delimiter.
    Delimiter,

    // Error handling
    /// Error element.
    Error,

    // Document structure (Element)
    /// Script element.
    Script,
    /// Function definition element.
    FunctionDef,
    /// Class definition element.
    ClassDef,
    /// Block element.
    Block,
    /// Expression element.
    Expression,
    /// Statement element.
    Statement,

    // EOF
    /// End of stream.
    Eof,
}

impl MatlabElementType {
    /// Returns true if the element type is a token.
    pub fn is_token(&self) -> bool {
        (*self as u8) <= (Self::Eof as u8) && !self.is_element()
    }

    /// Returns true if the element type is an element.
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Script | Self::FunctionDef | Self::ClassDef | Self::Block | Self::Expression | Self::Statement)
    }
}

impl ElementType for MatlabElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::MatlabTokenType> for MatlabElementType {
    fn from(token: crate::lexer::token_type::MatlabTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
