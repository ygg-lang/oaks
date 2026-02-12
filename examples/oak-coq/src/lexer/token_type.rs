use oak_core::{Token, TokenType, UniversalTokenRole};

/// Type alias for a Coq token.
pub type CoqToken = Token<CoqTokenType>;

/// Token types for the Coq language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum CoqTokenType {
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,
    // Literals
    /// String literal.
    StringLiteral,
    /// Number literal.
    NumberLiteral,
    /// Identifier.
    Identifier,

    // Keywords
    /// `Theorem` keyword.
    Theorem,
    /// `Lemma` keyword.
    Lemma,
    /// `Remark` keyword.
    Remark,
    /// `Fact` keyword.
    Fact,
    /// `Corollary` keyword.
    Corollary,
    /// `Proposition` keyword.
    Proposition,
    /// `Definition` keyword.
    Definition,
    /// `Example` keyword.
    Example,
    /// `Fixpoint` keyword.
    Fixpoint,
    /// `CoFixpoint` keyword.
    CoFixpoint,
    /// `Inductive` keyword.
    Inductive,
    /// `CoInductive` keyword.
    CoInductive,
    /// `Record` keyword.
    Record,
    /// `Structure` keyword.
    Structure,
    /// `Variant` keyword.
    Variant,
    /// `Module` keyword.
    Module,
    /// `Section` keyword.
    Section,
    /// `End` keyword.
    End,
    /// `Require` keyword.
    Require,
    /// `Import` keyword.
    Import,
    /// `Export` keyword.
    Export,
    /// `Proof` keyword.
    Proof,
    /// `Qed` keyword.
    Qed,
    /// `Defined` keyword.
    Defined,
    /// `Admitted` keyword.
    Admitted,
    /// `If` keyword.
    If,
    /// `Then` keyword.
    Then,
    /// `Else` keyword.
    Else,
    /// `Type` keyword.
    Type,
    /// `Prop` keyword.
    Prop,
    /// `Set` keyword.
    Set,
    /// `Check` keyword.
    Check,
    /// `Print` keyword.
    Print,
    /// `Search` keyword.
    Search,
    /// `Locate` keyword.
    Locate,
    /// `About` keyword.
    About,
    /// `Match` keyword.
    Match,
    /// `With` keyword.
    With,
    /// `Forall` symbol or keyword.
    Forall,
    /// `Exists` symbol or keyword.
    Exists,
    /// `Fun` keyword.
    Fun,
    /// `Let` keyword.
    Let,
    /// `In` keyword.
    In,
    // Additional Keywords from token_type.rs
    /// `Class` keyword.
    Class,
    /// `Instance` keyword.
    Instance,
    /// `Intros` tactic keyword.
    Intros,
    /// `Simpl` tactic keyword.
    Simpl,
    /// `Reflexivity` tactic keyword.
    Reflexivity,
    /// `Rewrite` tactic keyword.
    Rewrite,
    /// `Apply` tactic keyword.
    Apply,
    /// `Exact` tactic keyword.
    Exact,
    /// `Assumption` tactic keyword.
    Assumption,
    /// `Auto` tactic keyword.
    Auto,
    /// `Trivial` tactic keyword.
    Trivial,
    /// `Discriminate` tactic keyword.
    Discriminate,
    /// `Injection` tactic keyword.
    Injection,
    /// `Inversion` tactic keyword.
    Inversion,
    /// `Destruct` tactic keyword.
    Destruct,
    /// `Induction` tactic keyword.
    Induction,
    /// `Generalize` tactic keyword.
    Generalize,
    /// `Clear` tactic keyword.
    Clear,
    /// `Unfold` tactic keyword.
    Unfold,
    /// `Fold` tactic keyword.
    Fold,
    /// `Compute` tactic keyword.
    Compute,
    /// `Eval` tactic keyword.
    Eval,
    /// `Show` tactic keyword.
    Show,
    /// `Goal` keyword.
    Goal,
    /// `Goals` keyword.
    Goals,
    /// `Undo` command keyword.
    Undo,
    /// `Restart` command keyword.
    Restart,
    /// `Admit` tactic keyword.
    Admit,
    /// `Abort` command keyword.
    Abort,
    /// `Parameter` keyword.
    Parameter,
    /// `Axiom` keyword.
    Axiom,
    /// `Variable` keyword.
    Variable,
    /// `Hypothesis` keyword.
    Hypothesis,
    /// `Chapter` keyword.
    Chapter,
    /// `Open` keyword.
    Open,
    /// `Close` keyword.
    Close,
    /// `Scope` keyword.
    Scope,
    /// `Notation` keyword.
    Notation,
    /// `Infix` keyword.
    Infix,
    /// `Reserved` keyword.
    Reserved,
    /// `Bind` keyword.
    Bind,
    /// `Delimit` keyword.
    Delimit,
    /// `Arguments` keyword.
    Arguments,
    /// `Implicit` keyword.
    Implicit,
    /// `Coercion` keyword.
    Coercion,
    /// `Identity` keyword.
    Identity,
    /// `Canonical` keyword.
    Canonical,

    // Operators and delimiters
    /// Arrow `->`.
    Arrow,
    /// Double arrow `=>`.
    DoubleArrow,
    /// Colon `:`.
    Colon,
    /// Semicolon `;`.
    Semicolon,
    /// Comma `,`.
    Comma,
    /// Dot `.`.
    Dot,
    /// Pipe `|`.
    Pipe,
    /// Underscore `_`.
    Underscore,
    /// Equal `=`.
    Equal,
    /// Plus `+`.
    Plus,
    /// Minus `-`.
    Minus,
    /// Star `*`.
    Star,
    /// Slash `/`.
    Slash,
    /// Percent `%`.
    Percent,
    /// Less than `<`.
    Less,
    /// Greater than `>`.
    Greater,
    /// Less than or equal to `<=`.
    LessEqual,
    /// Greater than or equal to `>=`.
    GreaterEqual,
    /// Not equal `<>`.
    NotEqual,
    /// Tilde `~`.
    Tilde,
    /// At `@`.
    At,
    /// Question mark `?`.
    Question,
    /// Exclamation mark `!`.
    Exclamation,
    /// Ampersand `&`.
    Ampersand,
    /// Hash `#`.
    Hash,
    /// Dollar `$`.
    Dollar,
    /// Backslash `\`.
    Backslash,
    /// Caret `^`.
    Caret,
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    // Additional Operators from lexer/mod.rs
    /// Double colon `::`.
    DoubleColon,
    /// Double colon equal `::=`.
    DoubleColonEqual,
    /// Colon equal `:=`.
    ColonEqual,
    /// Turnstile `|-`.
    Turnstile,
    /// Logical AND `/\`.
    And,
    /// Logical OR `\/`.
    Or,
    /// Left arrow `<-`.
    LeftArrow,

    // Elements
    /// Root node.
    Root,
    /// Declaration node.
    Declaration,
    /// Statement node.
    Statement,
    /// Expression node.
    Expression,
    /// Lexing error.
    Error,
    /// End of stream.
    Eof,
}

impl TokenType for CoqTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}
