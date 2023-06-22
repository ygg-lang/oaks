use crate::{config::FormatConfig, document::printer::Printer};
use alloc::{boxed::Box, string::String, vec::Vec};
use core::fmt;

pub mod printer;

/// Document 抽象，用于描述布局逻辑
#[derive(Clone)]
pub enum Doc {
    /// 空文档
    Nil,
    /// 纯文本
    Text(String),
    /// 连接多个文档
    Concat(Vec<Doc>),
    /// 组合文档，作为换行计算的最小单位
    Group(Box<Doc>),
    /// 增加缩进
    Indent(Box<Doc>),
    /// 强制换行
    Line,
    /// 软换行：如果 Group 展开则为换行，否则为空
    SoftLine,
    /// 软换行（带空格）：如果 Group 展开则为换行，否则为空格
    SoftLineSpace,
    /// 强制换行且会导致父级 Group 也展开
    HardLine,
}

/// 用于快速构建 Document 的宏
#[macro_export]
macro_rules! doc {
    // 缩进
    (indent $doc:expr) => {
        $crate::Doc::Indent(Box::new($doc))
    };
    // 组合
    (group $doc:expr) => {
        $crate::Doc::Group(Box::new($doc))
    };
    // 换行符
    (line) => {
        $crate::Doc::Line
    };
    (@line) => {
        $crate::Doc::Line
    };
    // 空
    (nil) => {
        $crate::Doc::Nil
    };
    // 软换行
    (soft_line) => {
        $crate::Doc::SoftLine
    };
    (@soft_line) => {
        $crate::Doc::SoftLine
    };
    // 软换行（带空格）
    (soft_line_space) => {
        $crate::Doc::SoftLineSpace
    };
    (@soft_line_space) => {
        $crate::Doc::SoftLineSpace
    };
    // 强制换行
    (hard_line) => {
        $crate::Doc::HardLine
    };
    (@hard_line) => {
        $crate::Doc::HardLine
    };
    // 列表连接
    ([ $($doc:tt),* $(,)? ]) => {
        $crate::Doc::Concat(vec![ $( $crate::doc!($doc) ),* ])
    };
    // 字符串字面量
    ($text:literal) => {
        $crate::Doc::Text($text.into())
    };
    // 嵌套 doc 调用或变量
    ($doc:expr) => {
        $doc
    };
}

impl fmt::Debug for Doc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(debug_assertions)]
        {
            match self {
                Doc::Nil => write!(f, "Nil"),
                Doc::Text(s) => write!(f, "Text({:?})", s),
                Doc::Concat(docs) => f.debug_list().entries(docs).finish(),
                Doc::Group(d) => f.debug_tuple("Group").field(d).finish(),
                Doc::Indent(d) => f.debug_tuple("Indent").field(d).finish(),
                Doc::Line => write!(f, "Line"),
                Doc::SoftLine => write!(f, "SoftLine"),
                Doc::SoftLineSpace => write!(f, "SoftLineSpace"),
                Doc::HardLine => write!(f, "HardLine"),
            }
        }
        #[cfg(not(debug_assertions))]
        {
            match self {
                Doc::Nil => write!(f, "Doc::Nil"),
                Doc::Text(_) => write!(f, "Doc::Text"),
                Doc::Concat(_) => write!(f, "Doc::Concat"),
                Doc::Group(_) => write!(f, "Doc::Group"),
                Doc::Indent(_) => write!(f, "Doc::Indent"),
                Doc::Line => write!(f, "Doc::Line"),
                Doc::SoftLine => write!(f, "Doc::SoftLine"),
                Doc::SoftLineSpace => write!(f, "Doc::SoftLineSpace"),
                Doc::HardLine => write!(f, "Doc::HardLine"),
            }
        }
    }
}

impl Doc {
    /// 渲染 Document 为字符串
    pub fn render(&self, config: FormatConfig) -> String {
        Printer::new(config).print(self)
    }

    /// 创建文本文档
    pub fn text<S: Into<String>>(text: S) -> Self {
        Doc::Text(text.into())
    }

    /// 连接多个文档
    pub fn concat(docs: Vec<Doc>) -> Self {
        Doc::Concat(docs)
    }

    /// 创建组合文档
    pub fn group(doc: Doc) -> Self {
        Doc::Group(Box::new(doc))
    }

    /// 创建缩进文档
    pub fn indent(doc: Doc) -> Self {
        Doc::Indent(Box::new(doc))
    }

    /// 辅助方法：将多个文档用指定分隔符连接
    pub fn join(docs: Vec<Doc>, separator: Doc) -> Self {
        let mut result = Vec::new();
        for (i, doc) in docs.into_iter().enumerate() {
            if i > 0 {
                result.push(separator.clone());
            }
            result.push(doc);
        }
        Doc::Concat(result)
    }
}

impl From<String> for Doc {
    fn from(s: String) -> Self {
        Doc::Text(s)
    }
}

impl From<&str> for Doc {
    fn from(s: &str) -> Self {
        Doc::Text(s.to_string())
    }
}
