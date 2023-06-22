#[derive(Debug, Clone)]
pub struct BashRoot {
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
