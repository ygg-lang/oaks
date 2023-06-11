/// Token 类型 trait
pub trait TokenKind {
    /// 获取 tokens 的类型
    fn token_type(&self) -> TokenType;
}

/// Token 类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    /// 关键字
    Keyword,
    /// 标识符
    Identifier,
    /// 字符串
    String,
    /// 数字
    Number,
    /// 注释
    Comment,
    /// 分隔符
    Delimiter,
    /// 操作符
    Operator,
    /// 错误
    Error,
    /// 其他
    Other,
}
