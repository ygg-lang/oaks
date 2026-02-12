use oak_core::{Token, TokenType, UniversalTokenRole};

/// Prolog token.
pub type PrologToken = Token<PrologTokenType>;

impl PrologTokenType {
    /// Check if this type is a token.
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    /// Check if this type is a structure element.
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

/// Prolog token types.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrologTokenType {
    // Whitespace and comments
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,

    // Literals
    /// Atom.
    Atom,
    /// Integer.
    Integer,
    /// Float.
    Float,
    /// String.
    String,
    /// Variable.
    Variable,

    // Operators
    /// Unification `=`.
    Unify, // =
    /// Not unification `\=`.
    NotUnify, // \=
    /// Equal `==`.
    Equal, // ==
    /// Not equal `\==`.
    NotEqual, // \==
    /// Arithmetic equal `=:=`.
    ArithEqual, // =:=
    /// Arithmetic not equal `=\=`.
    ArithNotEqual, // =\=
    /// Less than `<`.
    Less, // <
    /// Greater than `>`.
    Greater, // >
    /// Less or equal `=<`.
    LessEqual, // =<
    /// Greater or equal `>=`.
    GreaterEqual, // >=
    /// Is `is`.
    Is, // is
    /// Plus `+`.
    Plus, // +
    /// Minus `-`.
    Minus, // -
    /// Multiply `*`.
    Multiply, // *
    /// Divide `/`.
    Divide, // /
    /// Integer divide `//`.
    IntDivide, // //
    /// Modulo `mod`.
    Modulo, // mod
    /// Power `**`.
    Power, // **
    /// Bitwise and `/\`.
    BitwiseAnd, // /\
    /// Bitwise or `\/`.
    BitwiseOr, // \/
    /// Bitwise xor `xor`.
    BitwiseXor, // xor
    /// Bitwise not `\`.
    BitwiseNot, // \
    /// Left shift `<<`.
    LeftShift, // <<
    /// Right shift `>>`.
    RightShift, // >>

    // Punctuation
    /// Left parenthesis `(`.
    LeftParen, // (
    /// Right parenthesis `)`.
    RightParen, // )
    /// Left bracket `[`.
    LeftBracket, // [
    /// Right bracket `]`.
    RightBracket, // ]
    /// Left brace `{`.
    LeftBrace, // {
    /// Right brace `}`.
    RightBrace, // }
    /// Comma `,`.
    Comma, // ,
    /// Dot `.`.
    Dot, // .
    /// Pipe `|`.
    Pipe, // |
    /// Semicolon `;`.
    Semicolon, // ;
    /// Cut `!`.
    Cut, // !
    /// Question mark `?`.
    Question, // ?
    /// Colon `:`.
    Colon, // :
    /// Colon minus `:-`.
    ColonMinus, // :-
    /// Question minus `?-`.
    QuestionMinus, // ?-

    // Special constructs
    /// Functor.
    Functor,
    /// Clause.
    Clause,
    /// Rule.
    Rule,
    /// Fact.
    Fact,
    /// Query.
    Query,
    /// Directive.
    Directive,
    /// List.
    List,
    /// Structure.
    Structure,

    // Special
    /// Root.
    Root,
    /// Error.
    Error,
}
