use oak_core::{ElementType, UniversalElementRole};

/// Prolog element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrologElementType {
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
    /// Unify `=`.
    Unify, // =
    /// Not unify `\=`.
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
    /// Less than or equal `=<`.
    LessEqual, // =<
    /// Greater than or equal `>=`.
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
    /// Bitwise AND `/\`.
    BitwiseAnd, // /\
    /// Bitwise OR `\/`.
    BitwiseOr, // \/
    /// Bitwise XOR `xor`.
    BitwiseXor, // xor
    /// Bitwise NOT `\`.
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
    /// End of file.
    Eof,
}

impl PrologElementType {
    /// Checks if this type is a token.
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    /// Checks if this type is a structural element.
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
