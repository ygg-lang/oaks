/// Wolfram 根节点
#[derive(Clone, Debug)]
pub struct WolframRoot {
    pub expressions: Vec<WolframExpression>,
}

/// Wolfram 表达式
#[derive(Clone, Debug)]
pub enum WolframExpression {
    Identifier(String),
    Number(String),
    String(String),
    Call(WolframCall),
    Binary(WolframBinary),
    List(Vec<WolframExpression>),
}

/// Wolfram 函数调用 (e.g., f[x, y])
#[derive(Clone, Debug)]
pub struct WolframCall {
    pub head: Box<WolframExpression>,
    pub arguments: Vec<WolframExpression>,
}

/// Wolfram 二元运算 (e.g., a + b)
#[derive(Clone, Debug)]
pub struct WolframBinary {
    pub left: Box<WolframExpression>,
    pub operator: String,
    pub right: Box<WolframExpression>,
}
