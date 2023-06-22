use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::Serialize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Hash)]
pub enum PrologSyntaxKind {
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

impl PrologSyntaxKind {
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::Functor | Self::Clause | Self::Rule | Self::Fact | Self::Query | Self::Directive | Self::List | Self::Structure)
    }
}

impl TokenType for PrologSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for PrologSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            Self::Root => UniversalElementRole::Root,
            Self::Clause | Self::Directive | Self::Query => UniversalElementRole::Detail,
            _ => UniversalElementRole::None,
        }
    }
}
