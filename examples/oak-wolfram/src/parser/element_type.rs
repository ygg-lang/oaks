use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WolframElementType {
    Root,
    // 表达式
    Expression,
    // 函数调用 f[x]
    Call,
    // 参数列表 [x, y]
    Arguments,
    // 列表 {a, b}
    List,
    // 符号/标识符
    Symbol,
    // 字面量
    Literal,
    // 二元表达式 x + y
    BinaryExpr,
    // 前缀表达式 !x
    PrefixExpr,
    // 后缀表达式 x!
    PostfixExpr,
    // 错误
    Error,
}

impl fmt::Display for WolframElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ElementType for WolframElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::WolframTokenType> for WolframElementType {
    fn from(token: crate::lexer::token_type::WolframTokenType) -> Self {
        match token {
            crate::lexer::token_type::WolframTokenType::Root => Self::Root,
            crate::lexer::token_type::WolframTokenType::Identifier => Self::Symbol,
            crate::lexer::token_type::WolframTokenType::Integer | crate::lexer::token_type::WolframTokenType::Real | crate::lexer::token_type::WolframTokenType::String => Self::Literal,
            _ => Self::Error,
        }
    }
}
