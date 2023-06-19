use oak_core::SyntaxKind;
use serde::Serialize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
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

impl SyntaxKind for PrologSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(
            self,
            Self::Root | Self::Clause | Self::Rule | Self::Fact | Self::Query | Self::Directive | Self::List | Self::Structure
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            Self::Root | Self::Clause | Self::Rule | Self::Fact | Self::Query | Self::Directive | Self::List | Self::Structure
        )
    }
}
