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
                match token {
            crate::lexer::token_type::MatlabTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::MatlabTokenType::Newline => Self::Newline,
            crate::lexer::token_type::MatlabTokenType::Comment => Self::Comment,
            crate::lexer::token_type::MatlabTokenType::BlockComment => Self::BlockComment,
            crate::lexer::token_type::MatlabTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::MatlabTokenType::Number => Self::Number,
            crate::lexer::token_type::MatlabTokenType::String => Self::String,
            crate::lexer::token_type::MatlabTokenType::Character => Self::Character,
            crate::lexer::token_type::MatlabTokenType::Function => Self::Function,
            crate::lexer::token_type::MatlabTokenType::End => Self::End,
            crate::lexer::token_type::MatlabTokenType::If => Self::If,
            crate::lexer::token_type::MatlabTokenType::Else => Self::Else,
            crate::lexer::token_type::MatlabTokenType::Elseif => Self::Elseif,
            crate::lexer::token_type::MatlabTokenType::While => Self::While,
            crate::lexer::token_type::MatlabTokenType::For => Self::For,
            crate::lexer::token_type::MatlabTokenType::Break => Self::Break,
            crate::lexer::token_type::MatlabTokenType::Continue => Self::Continue,
            crate::lexer::token_type::MatlabTokenType::Return => Self::Return,
            crate::lexer::token_type::MatlabTokenType::Switch => Self::Switch,
            crate::lexer::token_type::MatlabTokenType::Case => Self::Case,
            crate::lexer::token_type::MatlabTokenType::Otherwise => Self::Otherwise,
            crate::lexer::token_type::MatlabTokenType::Try => Self::Try,
            crate::lexer::token_type::MatlabTokenType::Catch => Self::Catch,
            crate::lexer::token_type::MatlabTokenType::Global => Self::Global,
            crate::lexer::token_type::MatlabTokenType::Persistent => Self::Persistent,
            crate::lexer::token_type::MatlabTokenType::Classdef => Self::Classdef,
            crate::lexer::token_type::MatlabTokenType::Properties => Self::Properties,
            crate::lexer::token_type::MatlabTokenType::Methods => Self::Methods,
            crate::lexer::token_type::MatlabTokenType::Events => Self::Events,
            crate::lexer::token_type::MatlabTokenType::Plus => Self::Plus,
            crate::lexer::token_type::MatlabTokenType::Minus => Self::Minus,
            crate::lexer::token_type::MatlabTokenType::Times => Self::Times,
            crate::lexer::token_type::MatlabTokenType::Divide => Self::Divide,
            crate::lexer::token_type::MatlabTokenType::Power => Self::Power,
            crate::lexer::token_type::MatlabTokenType::LeftDivide => Self::LeftDivide,
            crate::lexer::token_type::MatlabTokenType::DotTimes => Self::DotTimes,
            crate::lexer::token_type::MatlabTokenType::DotDivide => Self::DotDivide,
            crate::lexer::token_type::MatlabTokenType::DotPower => Self::DotPower,
            crate::lexer::token_type::MatlabTokenType::DotLeftDivide => Self::DotLeftDivide,
            crate::lexer::token_type::MatlabTokenType::Equal => Self::Equal,
            crate::lexer::token_type::MatlabTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::MatlabTokenType::Less => Self::Less,
            crate::lexer::token_type::MatlabTokenType::Greater => Self::Greater,
            crate::lexer::token_type::MatlabTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::MatlabTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::MatlabTokenType::And => Self::And,
            crate::lexer::token_type::MatlabTokenType::Or => Self::Or,
            crate::lexer::token_type::MatlabTokenType::Not => Self::Not,
            crate::lexer::token_type::MatlabTokenType::AndAnd => Self::AndAnd,
            crate::lexer::token_type::MatlabTokenType::OrOr => Self::OrOr,
            crate::lexer::token_type::MatlabTokenType::Assign => Self::Assign,
            crate::lexer::token_type::MatlabTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::MatlabTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::MatlabTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::MatlabTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::MatlabTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::MatlabTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::MatlabTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::MatlabTokenType::Comma => Self::Comma,
            crate::lexer::token_type::MatlabTokenType::Dot => Self::Dot,
            crate::lexer::token_type::MatlabTokenType::Colon => Self::Colon,
            crate::lexer::token_type::MatlabTokenType::Question => Self::Question,
            crate::lexer::token_type::MatlabTokenType::At => Self::At,
            crate::lexer::token_type::MatlabTokenType::Transpose => Self::Transpose,
            crate::lexer::token_type::MatlabTokenType::DotTranspose => Self::DotTranspose,
            crate::lexer::token_type::MatlabTokenType::Operator => Self::Operator,
            crate::lexer::token_type::MatlabTokenType::Delimiter => Self::Delimiter,
            crate::lexer::token_type::MatlabTokenType::Error => Self::Error,
            crate::lexer::token_type::MatlabTokenType::Script => Self::Script,
            crate::lexer::token_type::MatlabTokenType::FunctionDef => Self::FunctionDef,
            crate::lexer::token_type::MatlabTokenType::ClassDef => Self::ClassDef,
            crate::lexer::token_type::MatlabTokenType::Block => Self::Block,
            crate::lexer::token_type::MatlabTokenType::Expression => Self::Expression,
            crate::lexer::token_type::MatlabTokenType::Statement => Self::Statement,
            crate::lexer::token_type::MatlabTokenType::Eof => Self::Eof,
            _ => Self::Error,
        }
    }
}
