use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Coq AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CoqElementType {
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
    /// Parsing error.
    Error,
    /// End of stream.
    Eof,
}

impl ElementType for CoqElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::CoqTokenType> for CoqElementType {
    fn from(token: crate::lexer::token_type::CoqTokenType) -> Self {
        match token {
            crate::lexer::token_type::CoqTokenType::Whitespace => CoqElementType::Whitespace,
            crate::lexer::token_type::CoqTokenType::Newline => CoqElementType::Newline,
            crate::lexer::token_type::CoqTokenType::Comment => CoqElementType::Comment,
            crate::lexer::token_type::CoqTokenType::StringLiteral => CoqElementType::StringLiteral,
            crate::lexer::token_type::CoqTokenType::NumberLiteral => CoqElementType::NumberLiteral,
            crate::lexer::token_type::CoqTokenType::Identifier => CoqElementType::Identifier,
            crate::lexer::token_type::CoqTokenType::Theorem => CoqElementType::Theorem,
            crate::lexer::token_type::CoqTokenType::Lemma => CoqElementType::Lemma,
            crate::lexer::token_type::CoqTokenType::Remark => CoqElementType::Remark,
            crate::lexer::token_type::CoqTokenType::Fact => CoqElementType::Fact,
            crate::lexer::token_type::CoqTokenType::Corollary => CoqElementType::Corollary,
            crate::lexer::token_type::CoqTokenType::Proposition => CoqElementType::Proposition,
            crate::lexer::token_type::CoqTokenType::Definition => CoqElementType::Definition,
            crate::lexer::token_type::CoqTokenType::Example => CoqElementType::Example,
            crate::lexer::token_type::CoqTokenType::Fixpoint => CoqElementType::Fixpoint,
            crate::lexer::token_type::CoqTokenType::CoFixpoint => CoqElementType::CoFixpoint,
            crate::lexer::token_type::CoqTokenType::Inductive => CoqElementType::Inductive,
            crate::lexer::token_type::CoqTokenType::CoInductive => CoqElementType::CoInductive,
            crate::lexer::token_type::CoqTokenType::Record => CoqElementType::Record,
            crate::lexer::token_type::CoqTokenType::Structure => CoqElementType::Structure,
            crate::lexer::token_type::CoqTokenType::Variant => CoqElementType::Variant,
            crate::lexer::token_type::CoqTokenType::Module => CoqElementType::Module,
            crate::lexer::token_type::CoqTokenType::Section => CoqElementType::Section,
            crate::lexer::token_type::CoqTokenType::End => CoqElementType::End,
            crate::lexer::token_type::CoqTokenType::Require => CoqElementType::Require,
            crate::lexer::token_type::CoqTokenType::Import => CoqElementType::Import,
            crate::lexer::token_type::CoqTokenType::Export => CoqElementType::Export,
            crate::lexer::token_type::CoqTokenType::Proof => CoqElementType::Proof,
            crate::lexer::token_type::CoqTokenType::Qed => CoqElementType::Qed,
            crate::lexer::token_type::CoqTokenType::Defined => CoqElementType::Defined,
            crate::lexer::token_type::CoqTokenType::Admitted => CoqElementType::Admitted,
            crate::lexer::token_type::CoqTokenType::If => CoqElementType::If,
            crate::lexer::token_type::CoqTokenType::Then => CoqElementType::Then,
            crate::lexer::token_type::CoqTokenType::Else => CoqElementType::Else,
            crate::lexer::token_type::CoqTokenType::Type => CoqElementType::Type,
            crate::lexer::token_type::CoqTokenType::Prop => CoqElementType::Prop,
            crate::lexer::token_type::CoqTokenType::Set => CoqElementType::Set,
            crate::lexer::token_type::CoqTokenType::Check => CoqElementType::Check,
            crate::lexer::token_type::CoqTokenType::Print => CoqElementType::Print,
            crate::lexer::token_type::CoqTokenType::Search => CoqElementType::Search,
            crate::lexer::token_type::CoqTokenType::Locate => CoqElementType::Locate,
            crate::lexer::token_type::CoqTokenType::About => CoqElementType::About,
            crate::lexer::token_type::CoqTokenType::Match => CoqElementType::Match,
            crate::lexer::token_type::CoqTokenType::With => CoqElementType::With,
            crate::lexer::token_type::CoqTokenType::Forall => CoqElementType::Forall,
            crate::lexer::token_type::CoqTokenType::Exists => CoqElementType::Exists,
            crate::lexer::token_type::CoqTokenType::Fun => CoqElementType::Fun,
            crate::lexer::token_type::CoqTokenType::Let => CoqElementType::Let,
            crate::lexer::token_type::CoqTokenType::In => CoqElementType::In,
            crate::lexer::token_type::CoqTokenType::Class => CoqElementType::Class,
            crate::lexer::token_type::CoqTokenType::Instance => CoqElementType::Instance,
            crate::lexer::token_type::CoqTokenType::Intros => CoqElementType::Intros,
            crate::lexer::token_type::CoqTokenType::Simpl => CoqElementType::Simpl,
            crate::lexer::token_type::CoqTokenType::Reflexivity => CoqElementType::Reflexivity,
            crate::lexer::token_type::CoqTokenType::Rewrite => CoqElementType::Rewrite,
            crate::lexer::token_type::CoqTokenType::Apply => CoqElementType::Apply,
            crate::lexer::token_type::CoqTokenType::Exact => CoqElementType::Exact,
            crate::lexer::token_type::CoqTokenType::Assumption => CoqElementType::Assumption,
            crate::lexer::token_type::CoqTokenType::Auto => CoqElementType::Auto,
            crate::lexer::token_type::CoqTokenType::Trivial => CoqElementType::Trivial,
            crate::lexer::token_type::CoqTokenType::Discriminate => CoqElementType::Discriminate,
            crate::lexer::token_type::CoqTokenType::Injection => CoqElementType::Injection,
            crate::lexer::token_type::CoqTokenType::Inversion => CoqElementType::Inversion,
            crate::lexer::token_type::CoqTokenType::Destruct => CoqElementType::Destruct,
            crate::lexer::token_type::CoqTokenType::Induction => CoqElementType::Induction,
            crate::lexer::token_type::CoqTokenType::Generalize => CoqElementType::Generalize,
            crate::lexer::token_type::CoqTokenType::Clear => CoqElementType::Clear,
            crate::lexer::token_type::CoqTokenType::Unfold => CoqElementType::Unfold,
            crate::lexer::token_type::CoqTokenType::Fold => CoqElementType::Fold,
            crate::lexer::token_type::CoqTokenType::Compute => CoqElementType::Compute,
            crate::lexer::token_type::CoqTokenType::Eval => CoqElementType::Eval,
            crate::lexer::token_type::CoqTokenType::Show => CoqElementType::Show,
            crate::lexer::token_type::CoqTokenType::Goal => CoqElementType::Goal,
            crate::lexer::token_type::CoqTokenType::Goals => CoqElementType::Goals,
            crate::lexer::token_type::CoqTokenType::Undo => CoqElementType::Undo,
            crate::lexer::token_type::CoqTokenType::Restart => CoqElementType::Restart,
            crate::lexer::token_type::CoqTokenType::Admit => CoqElementType::Admit,
            crate::lexer::token_type::CoqTokenType::Abort => CoqElementType::Abort,
            crate::lexer::token_type::CoqTokenType::Parameter => CoqElementType::Parameter,
            crate::lexer::token_type::CoqTokenType::Axiom => CoqElementType::Axiom,
            crate::lexer::token_type::CoqTokenType::Variable => CoqElementType::Variable,
            crate::lexer::token_type::CoqTokenType::Hypothesis => CoqElementType::Hypothesis,
            crate::lexer::token_type::CoqTokenType::Chapter => CoqElementType::Chapter,
            crate::lexer::token_type::CoqTokenType::Open => CoqElementType::Open,
            crate::lexer::token_type::CoqTokenType::Close => CoqElementType::Close,
            crate::lexer::token_type::CoqTokenType::Scope => CoqElementType::Scope,
            crate::lexer::token_type::CoqTokenType::Notation => CoqElementType::Notation,
            crate::lexer::token_type::CoqTokenType::Infix => CoqElementType::Infix,
            crate::lexer::token_type::CoqTokenType::Reserved => CoqElementType::Reserved,
            crate::lexer::token_type::CoqTokenType::Bind => CoqElementType::Bind,
            crate::lexer::token_type::CoqTokenType::Delimit => CoqElementType::Delimit,
            crate::lexer::token_type::CoqTokenType::Arguments => CoqElementType::Arguments,
            crate::lexer::token_type::CoqTokenType::Implicit => CoqElementType::Implicit,
            crate::lexer::token_type::CoqTokenType::Coercion => CoqElementType::Coercion,
            crate::lexer::token_type::CoqTokenType::Identity => CoqElementType::Identity,
            crate::lexer::token_type::CoqTokenType::Canonical => CoqElementType::Canonical,
            crate::lexer::token_type::CoqTokenType::Arrow => CoqElementType::Arrow,
            crate::lexer::token_type::CoqTokenType::DoubleArrow => CoqElementType::DoubleArrow,
            crate::lexer::token_type::CoqTokenType::Colon => CoqElementType::Colon,
            crate::lexer::token_type::CoqTokenType::Semicolon => CoqElementType::Semicolon,
            crate::lexer::token_type::CoqTokenType::Comma => CoqElementType::Comma,
            crate::lexer::token_type::CoqTokenType::Dot => CoqElementType::Dot,
            crate::lexer::token_type::CoqTokenType::Pipe => CoqElementType::Pipe,
            crate::lexer::token_type::CoqTokenType::Underscore => CoqElementType::Underscore,
            crate::lexer::token_type::CoqTokenType::Equal => CoqElementType::Equal,
            crate::lexer::token_type::CoqTokenType::Plus => CoqElementType::Plus,
            crate::lexer::token_type::CoqTokenType::Minus => CoqElementType::Minus,
            crate::lexer::token_type::CoqTokenType::Star => CoqElementType::Star,
            crate::lexer::token_type::CoqTokenType::Slash => CoqElementType::Slash,
            crate::lexer::token_type::CoqTokenType::Percent => CoqElementType::Percent,
            crate::lexer::token_type::CoqTokenType::Less => CoqElementType::Less,
            crate::lexer::token_type::CoqTokenType::Greater => CoqElementType::Greater,
            crate::lexer::token_type::CoqTokenType::LessEqual => CoqElementType::LessEqual,
            crate::lexer::token_type::CoqTokenType::GreaterEqual => CoqElementType::GreaterEqual,
            crate::lexer::token_type::CoqTokenType::NotEqual => CoqElementType::NotEqual,
            crate::lexer::token_type::CoqTokenType::Tilde => CoqElementType::Tilde,
            crate::lexer::token_type::CoqTokenType::At => CoqElementType::At,
            crate::lexer::token_type::CoqTokenType::Question => CoqElementType::Question,
            crate::lexer::token_type::CoqTokenType::Exclamation => CoqElementType::Exclamation,
            crate::lexer::token_type::CoqTokenType::Ampersand => CoqElementType::Ampersand,
            crate::lexer::token_type::CoqTokenType::Hash => CoqElementType::Hash,
            crate::lexer::token_type::CoqTokenType::Dollar => CoqElementType::Dollar,
            crate::lexer::token_type::CoqTokenType::Backslash => CoqElementType::Backslash,
            crate::lexer::token_type::CoqTokenType::Caret => CoqElementType::Caret,
            crate::lexer::token_type::CoqTokenType::LeftParen => CoqElementType::LeftParen,
            crate::lexer::token_type::CoqTokenType::RightParen => CoqElementType::RightParen,
            crate::lexer::token_type::CoqTokenType::LeftBracket => CoqElementType::LeftBracket,
            crate::lexer::token_type::CoqTokenType::RightBracket => CoqElementType::RightBracket,
            crate::lexer::token_type::CoqTokenType::LeftBrace => CoqElementType::LeftBrace,
            crate::lexer::token_type::CoqTokenType::RightBrace => CoqElementType::RightBrace,
            crate::lexer::token_type::CoqTokenType::DoubleColon => CoqElementType::DoubleColon,
            crate::lexer::token_type::CoqTokenType::DoubleColonEqual => CoqElementType::DoubleColonEqual,
            crate::lexer::token_type::CoqTokenType::ColonEqual => CoqElementType::ColonEqual,
            crate::lexer::token_type::CoqTokenType::Turnstile => CoqElementType::Turnstile,
            crate::lexer::token_type::CoqTokenType::And => CoqElementType::And,
            crate::lexer::token_type::CoqTokenType::Or => CoqElementType::Or,
            crate::lexer::token_type::CoqTokenType::LeftArrow => CoqElementType::LeftArrow,
            crate::lexer::token_type::CoqTokenType::Root => CoqElementType::Root,
            crate::lexer::token_type::CoqTokenType::Declaration => CoqElementType::Declaration,
            crate::lexer::token_type::CoqTokenType::Statement => CoqElementType::Statement,
            crate::lexer::token_type::CoqTokenType::Expression => CoqElementType::Expression,
            crate::lexer::token_type::CoqTokenType::Error => CoqElementType::Error,
            crate::lexer::token_type::CoqTokenType::Eof => CoqElementType::Eof,
        }
    }
}
