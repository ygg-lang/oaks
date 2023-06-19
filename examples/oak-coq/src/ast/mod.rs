use core::range::Range;

/// Coq 语言的根节点
#[derive(Debug, PartialEq, Clone)]
pub struct CoqRoot {
    pub vernaculars: Vec<Vernacular>,
}

/// Coq 语句
#[derive(Debug, PartialEq, Clone)]
pub enum Vernacular {
    Definition { name: String, body: String, span: Range<usize> },
    Theorem { name: String, statement: String, proof: String, span: Range<usize> },
    Inductive { name: String, constructors: Vec<String>, span: Range<usize> },
    Fixpoint { name: String, body: String, span: Range<usize> },
    Check { term: String, span: Range<usize> },
    Print { name: String, span: Range<usize> },
}

impl CoqRoot {
    pub fn new() -> Self {
        Self { vernaculars: Vec::new() }
    }

    pub fn with_vernaculars(vernaculars: Vec<Vernacular>) -> Self {
        Self { vernaculars }
    }
}

impl Default for CoqRoot {
    fn default() -> Self {
        Self::new()
    }
}
