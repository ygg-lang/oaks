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
                match token {
            crate::lexer::token_type::PrologTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::PrologTokenType::Newline => Self::Newline,
            crate::lexer::token_type::PrologTokenType::Comment => Self::Comment,
            crate::lexer::token_type::PrologTokenType::Atom => Self::Atom,
            crate::lexer::token_type::PrologTokenType::Integer => Self::Integer,
            crate::lexer::token_type::PrologTokenType::Float => Self::Float,
            crate::lexer::token_type::PrologTokenType::String => Self::String,
            crate::lexer::token_type::PrologTokenType::Variable => Self::Variable,
            crate::lexer::token_type::PrologTokenType::Unify => Self::Unify,
            crate::lexer::token_type::PrologTokenType::NotUnify => Self::NotUnify,
            crate::lexer::token_type::PrologTokenType::Equal => Self::Equal,
            crate::lexer::token_type::PrologTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::PrologTokenType::ArithEqual => Self::ArithEqual,
            crate::lexer::token_type::PrologTokenType::ArithNotEqual => Self::ArithNotEqual,
            crate::lexer::token_type::PrologTokenType::Less => Self::Less,
            crate::lexer::token_type::PrologTokenType::Greater => Self::Greater,
            crate::lexer::token_type::PrologTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::PrologTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::PrologTokenType::Is => Self::Is,
            crate::lexer::token_type::PrologTokenType::Plus => Self::Plus,
            crate::lexer::token_type::PrologTokenType::Minus => Self::Minus,
            crate::lexer::token_type::PrologTokenType::Multiply => Self::Multiply,
            crate::lexer::token_type::PrologTokenType::Divide => Self::Divide,
            crate::lexer::token_type::PrologTokenType::IntDivide => Self::IntDivide,
            crate::lexer::token_type::PrologTokenType::Modulo => Self::Modulo,
            crate::lexer::token_type::PrologTokenType::Power => Self::Power,
            crate::lexer::token_type::PrologTokenType::BitwiseAnd => Self::BitwiseAnd,
            crate::lexer::token_type::PrologTokenType::BitwiseOr => Self::BitwiseOr,
            crate::lexer::token_type::PrologTokenType::BitwiseXor => Self::BitwiseXor,
            crate::lexer::token_type::PrologTokenType::BitwiseNot => Self::BitwiseNot,
            crate::lexer::token_type::PrologTokenType::LeftShift => Self::LeftShift,
            crate::lexer::token_type::PrologTokenType::RightShift => Self::RightShift,
            crate::lexer::token_type::PrologTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::PrologTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::PrologTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::PrologTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::PrologTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::PrologTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::PrologTokenType::Comma => Self::Comma,
            crate::lexer::token_type::PrologTokenType::Dot => Self::Dot,
            crate::lexer::token_type::PrologTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::PrologTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::PrologTokenType::Cut => Self::Cut,
            crate::lexer::token_type::PrologTokenType::Question => Self::Question,
            crate::lexer::token_type::PrologTokenType::Colon => Self::Colon,
            crate::lexer::token_type::PrologTokenType::ColonMinus => Self::ColonMinus,
            crate::lexer::token_type::PrologTokenType::QuestionMinus => Self::QuestionMinus,
            crate::lexer::token_type::PrologTokenType::Functor => Self::Functor,
            crate::lexer::token_type::PrologTokenType::Clause => Self::Clause,
            crate::lexer::token_type::PrologTokenType::Rule => Self::Rule,
            crate::lexer::token_type::PrologTokenType::Fact => Self::Fact,
            crate::lexer::token_type::PrologTokenType::Query => Self::Query,
            crate::lexer::token_type::PrologTokenType::Directive => Self::Directive,
            crate::lexer::token_type::PrologTokenType::List => Self::List,
            crate::lexer::token_type::PrologTokenType::Structure => Self::Structure,
            crate::lexer::token_type::PrologTokenType::Root => Self::Root,
            crate::lexer::token_type::PrologTokenType::Error => Self::Error,
            crate::lexer::token_type::PrologTokenType::Eof => Self::Eof,
            _ => Self::Error,
        }
    }
}
