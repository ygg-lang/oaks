#![doc = include_str!("readme.md")]

/// Prolog root node.
#[derive(Debug, Clone)]
pub struct PrologRoot;

/// Prolog token type alias.
pub type PrologToken = crate::lexer::PrologTokenType;

/// Prolog source file.
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// All clauses in the file.
    pub clauses: Vec<Clause>,
}

/// Prolog clause.
#[derive(Debug, Clone)]
pub enum Clause {
    /// Rule.
    Rule(Rule),
    /// Fact.
    Fact(Fact),
    /// Query.
    Query(Query),
    /// Directive.
    Directive(Directive),
}

/// Prolog rule.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Rule head.
    pub head: Term,
    /// Rule body.
    pub body: Term,
}

/// Prolog fact.
#[derive(Debug, Clone)]
pub struct Fact {
    /// Fact term.
    pub term: Term,
}

/// Prolog query.
#[derive(Debug, Clone)]
pub struct Query {
    /// Query term.
    pub term: Term,
}

/// Prolog directive.
#[derive(Debug, Clone)]
pub struct Directive {
    /// Directive term.
    pub term: Term,
}

/// Prolog term.
#[derive(Debug, Clone)]
pub enum Term {
    /// Atom.
    Atom(String),
    /// Variable.
    Variable(String),
    /// Number.
    Number(String),
    /// String.
    String(String),
    /// Compound term.
    Compound {
        /// Functor.
        functor: String,
        /// Arguments list.
        args: Vec<Term>,
    },
    /// List.
    List(Vec<Term>),
}
