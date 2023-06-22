#[derive(Debug, Clone)]
pub struct PrologRoot;

pub type PrologToken = crate::kind::PrologSyntaxKind;

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub clauses: Vec<Clause>,
}

#[derive(Debug, Clone)]
pub enum Clause {
    Rule(Rule),
    Fact(Fact),
    Query(Query),
    Directive(Directive),
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub head: Term,
    pub body: Term,
}

#[derive(Debug, Clone)]
pub struct Fact {
    pub term: Term,
}

#[derive(Debug, Clone)]
pub struct Query {
    pub term: Term,
}

#[derive(Debug, Clone)]
pub struct Directive {
    pub term: Term,
}

#[derive(Debug, Clone)]
pub enum Term {
    Atom(String),
    Variable(String),
    Number(String),
    String(String),
    Compound { functor: String, args: Vec<Term> },
    List(Vec<Term>),
}
