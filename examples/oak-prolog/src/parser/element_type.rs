use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PrologElementType {
    // Whitespace and comments
    Whitespace,
    Newline,
    Comment,

    // Literals
    Atom,
    Integer,
    Float,
    String,
    Variable,

    // Operators
    Unify,         // =
    NotUnify,      // \=
    Equal,         // ==
    NotEqual,      // \==
    ArithEqual,    // =:=
    ArithNotEqual, // =\=
    Less,          // <
    Greater,       // >
    LessEqual,     // =<
    GreaterEqual,  // >=
    Is,            // is
    Plus,          // +
    Minus,         // -
    Multiply,      // *
    Divide,        // /
    IntDivide,     // //
    Modulo,        // mod
    Power,         // **
    BitwiseAnd,    // /\
    BitwiseOr,     // \/
    BitwiseXor,    // xor
    BitwiseNot,    // \
    LeftShift,     // <<
    RightShift,    // >>

    // Punctuation
    LeftParen,     // (
    RightParen,    // )
    LeftBracket,   // [
    RightBracket,  // ]
    LeftBrace,     // {
    RightBrace,    // }
    Comma,         // ,
    Dot,           // .
    Pipe,          // |
    Semicolon,     // ;
    Cut,           // !
    Question,      // ?
    Colon,         // :
    ColonMinus,    // :-
    QuestionMinus, // ?-

    // Special constructs
    Functor,
    Clause,
    Rule,
    Fact,
    Query,
    Directive,
    List,
    Structure,

    // Special
    Root,
    Error,
    Eof,
}

impl PrologElementType {
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::Functor | Self::Clause | Self::Rule | Self::Fact | Self::Query | Self::Directive | Self::List | Self::Structure)
    }
}

impl ElementType for PrologElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::PrologTokenType> for PrologElementType {
    fn from(token: crate::lexer::token_type::PrologTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
