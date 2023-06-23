use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type PrologToken = Token<PrologTokenType>;

impl PrologTokenType {
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::Functor | Self::Clause | Self::Rule | Self::Fact | Self::Query | Self::Directive | Self::List | Self::Structure)
    }
}

impl TokenType for PrologTokenType {
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PrologTokenType {
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
