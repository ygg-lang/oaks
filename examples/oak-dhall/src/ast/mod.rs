use core::range::Range;
use serde::{Deserialize, Serialize};

/// DHall AST 根节点
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DHallRoot {
    pub expressions: Vec<DHallExpr>,
}

/// DHall 表达式
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum DHallExpr {
    /// 标识符
    Identifier {
        name: String,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// 字面量
    Literal {
        value: String,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// 函数应用
    Application {
        func: Box<DHallExpr>,
        arg: Box<DHallExpr>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// Lambda 表达式
    Lambda {
        param: String,
        body: Box<DHallExpr>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}
