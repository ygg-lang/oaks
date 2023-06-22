use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// Coq 语言的语法种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum CoqSyntaxKind {
    /// Whitespace characters (spaces, tabs)
    Whitespace,
    /// Newline characters
    Newline,
    /// Comments in Coq code
    Comment,
    // Literals
    /// String literal values
    StringLiteral,
    /// Numeric literal values
    NumberLiteral,
    /// Variable and function identifiers
    Identifier,

    // Keywords
    /// Theorem declaration keyword
    Theorem,
    /// Lemma declaration keyword
    Lemma,
    /// Remark declaration keyword
    Remark,
    /// Fact declaration keyword
    Fact,
    /// Corollary declaration keyword
    Corollary,
    /// Proposition declaration keyword
    Proposition,
    /// Definition declaration keyword
    Definition,
    /// Example declaration keyword
    Example,
    /// Fixpoint (recursive function) declaration keyword
    Fixpoint,
    /// CoFixpoint declaration keyword
    CoFixpoint,
    /// Inductive type declaration keyword
    Inductive,
    /// CoInductive declaration keyword
    CoInductive,
    /// Record type declaration keyword
    Record,
    /// Structure declaration keyword
    Structure,
    /// Variant declaration keyword
    Variant,
    /// Module declaration keyword
    Module,
    /// Section declaration keyword
    Section,
    /// End keyword
    End,
    /// Require keyword
    Require,
    /// Import keyword
    Import,
    /// Export keyword
    Export,
    /// Proof mode entry keyword
    Proof,
    /// Proof completion keyword
    Qed,
    /// Defined keyword
    Defined,
    /// Admitted keyword
    Admitted,
    /// Abort keyword
    Abort,
    /// Match keyword
    Match,
    /// With keyword
    With,
    /// Forall keyword
    Forall,
    /// Exists keyword
    Exists,
    /// Fun keyword
    Fun,
    /// Let keyword
    Let,
    /// In keyword
    In,
    /// If keyword
    If,
    /// Then keyword
    Then,
    /// Else keyword
    Else,
    /// Type keyword
    Type,
    /// Prop keyword
    Prop,
    /// Set keyword
    Set,
    /// Check command keyword
    Check,
    /// Print command keyword
    Print,
    /// Search command keyword
    Search,
    /// Locate command keyword
    Locate,
    /// About command keyword
    About,

    // Additional Keywords from token_type.rs
    Class,
    Instance,
    Intros,
    Simpl,
    Reflexivity,
    Rewrite,
    Apply,
    Exact,
    Assumption,
    Auto,
    Trivial,
    Discriminate,
    Injection,
    Inversion,
    Destruct,
    Induction,
    Generalize,
    Clear,
    Unfold,
    Fold,
    Compute,
    Eval,
    Show,
    Goal,
    Goals,
    Undo,
    Restart,
    Admit,
    Parameter,
    Axiom,
    Variable,
    Hypothesis,
    Chapter,
    Open,
    Close,
    Scope,
    Notation,
    Infix,
    Reserved,
    Bind,
    Delimit,
    Arguments,
    Implicit,
    Coercion,
    Identity,
    Canonical,

    // Operators and delimiters
    /// Arrow operator (->)
    Arrow,
    /// Double arrow operator (=>)
    DoubleArrow,
    /// Colon delimiter (:)
    Colon,
    /// Semicolon delimiter (;)
    Semicolon,
    /// Comma delimiter (,)
    Comma,
    /// Dot delimiter (.)
    Dot,
    /// Pipe delimiter (|)
    Pipe,
    /// Underscore wildcard (_)
    Underscore,
    /// Equality operator (=)
    Equal,
    /// Plus operator (+)
    Plus,
    /// Minus operator (-)
    Minus,
    /// Multiplication operator (*)
    Star,
    /// Division operator (/)
    Slash,
    /// Modulo operator (%)
    Percent,
    /// Less than operator (<)
    Less,
    /// Greater than operator (>)
    Greater,
    /// Less than or equal operator (<=)
    LessEqual,
    /// Greater than or equal operator (>=)
    GreaterEqual,
    /// Not equal operator (<>)
    NotEqual,
    /// Tilde operator (~)
    Tilde,
    /// At symbol (@)
    At,
    /// Question mark (?)
    Question,
    /// Exclamation mark (!)
    Exclamation,
    /// Ampersand operator (&)
    Ampersand,
    /// Hash symbol (#)
    Hash,
    /// Dollar symbol ($)
    Dollar,
    /// Backslash operator (\)
    Backslash,
    /// Caret operator (^)
    Caret,
    /// Left parenthesis (()
    LeftParen,
    /// Right parenthesis ())
    RightParen,
    /// Left bracket ([)
    LeftBracket,
    /// Right bracket (])
    RightBracket,
    /// Left brace ({)
    LeftBrace,
    /// Right brace (})
    RightBrace,

    // Additional Operators from lexer/mod.rs
    DoubleColon,
    DoubleColonEqual,
    ColonEqual,
    Turnstile,
    And,
    Or,
    LeftArrow,

    // Elements
    Root,
    Declaration,
    Statement,
    Expression,

    /// Error token
    Error,
    /// End of file
    Eof,
}

impl TokenType for CoqSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::StringLiteral | Self::NumberLiteral => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Theorem
            | Self::Lemma
            | Self::Remark
            | Self::Fact
            | Self::Corollary
            | Self::Proposition
            | Self::Definition
            | Self::Example
            | Self::Fixpoint
            | Self::CoFixpoint
            | Self::Inductive
            | Self::CoInductive
            | Self::Record
            | Self::Structure
            | Self::Variant
            | Self::Module
            | Self::Section
            | Self::End
            | Self::Require
            | Self::Import
            | Self::Export
            | Self::Proof
            | Self::Qed
            | Self::Defined
            | Self::Admitted
            | Self::Abort
            | Self::Match
            | Self::With
            | Self::Forall
            | Self::Exists
            | Self::Fun
            | Self::Let
            | Self::In
            | Self::If
            | Self::Then
            | Self::Else
            | Self::Type
            | Self::Prop
            | Self::Set
            | Self::Check
            | Self::Print
            | Self::Search
            | Self::Locate
            | Self::About
            | Self::Class
            | Self::Instance
            | Self::Intros
            | Self::Simpl
            | Self::Reflexivity
            | Self::Rewrite
            | Self::Apply
            | Self::Exact
            | Self::Assumption
            | Self::Auto
            | Self::Trivial
            | Self::Discriminate
            | Self::Injection
            | Self::Inversion
            | Self::Destruct
            | Self::Induction
            | Self::Generalize
            | Self::Clear
            | Self::Unfold
            | Self::Fold
            | Self::Compute
            | Self::Eval
            | Self::Show
            | Self::Goal
            | Self::Goals
            | Self::Undo
            | Self::Restart
            | Self::Admit
            | Self::Parameter
            | Self::Axiom
            | Self::Variable
            | Self::Hypothesis
            | Self::Chapter
            | Self::Open
            | Self::Close
            | Self::Scope
            | Self::Notation
            | Self::Infix
            | Self::Reserved
            | Self::Bind
            | Self::Delimit
            | Self::Arguments
            | Self::Implicit
            | Self::Coercion
            | Self::Identity
            | Self::Canonical => UniversalTokenRole::Keyword,
            Self::Arrow
            | Self::DoubleArrow
            | Self::Colon
            | Self::Semicolon
            | Self::Comma
            | Self::Dot
            | Self::Pipe
            | Self::Underscore
            | Self::Equal
            | Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::NotEqual
            | Self::Tilde
            | Self::At
            | Self::Question
            | Self::Exclamation
            | Self::Ampersand
            | Self::Hash
            | Self::Dollar
            | Self::Backslash
            | Self::Caret
            | Self::LeftParen
            | Self::RightParen
            | Self::LeftBracket
            | Self::RightBracket
            | Self::LeftBrace
            | Self::RightBrace
            | Self::DoubleColon
            | Self::DoubleColonEqual
            | Self::ColonEqual
            | Self::Turnstile
            | Self::And
            | Self::Or
            | Self::LeftArrow => UniversalTokenRole::Operator,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for CoqSyntaxKind {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Declaration => UniversalElementRole::Definition,
            Self::Statement => UniversalElementRole::Container,
            Self::Expression => UniversalElementRole::Typing,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::Container,
        }
    }
}
