pub type BashToken = crate::kind::BashSyntaxKind;

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub elements: Vec<Element>,
}

#[derive(Debug, Clone)]
pub enum Element {
    Command(String),
    Variable(String),
    String(String),
    Comment(String),
    Operator(String),
    Keyword(String),
    Text(String),
    Whitespace(String),
    Newline,
}
