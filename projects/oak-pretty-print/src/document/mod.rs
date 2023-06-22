use crate::{config::FormatConfig, document::printer::Printer};
use alloc::{borrow::Cow, boxed::Box, string::String, vec::Vec};
use core::fmt;

pub mod printer;

/// Document 抽象，用于描述布局逻辑
#[derive(Clone, serde::Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum Document<'a> {
    /// 空文档
    Nil,
    /// 纯文本
    Text(Cow<'a, str>),
    /// 连接多个文档
    Concat(Vec<Document<'a>>),
    /// 组合文档，作为换行计算的最小单位
    Group(Box<Document<'a>>),
    /// 增加缩进
    Indent(Box<Document<'a>>),
    /// 强制换行
    Line,
    /// 软换行：如果 Group 展开则为换行，否则为空
    SoftLine,
    /// 软换行（带空格）：如果 Group 展开则为换行，否则为空格
    SoftLineSpace,
    /// 强制换行且会导致父级 Group 也展开
    HardLine,
}

impl<'a> fmt::Debug for Document<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(debug_assertions)]
        {
            match self {
                Document::Nil => write!(f, "Nil"),
                Document::Text(s) => write!(f, "Text({:?})", s),
                Document::Concat(docs) => f.debug_list().entries(docs).finish(),
                Document::Group(d) => f.debug_tuple("Group").field(d).finish(),
                Document::Indent(d) => f.debug_tuple("Indent").field(d).finish(),
                Document::Line => write!(f, "Line"),
                Document::SoftLine => write!(f, "SoftLine"),
                Document::SoftLineSpace => write!(f, "SoftLineSpace"),
                Document::HardLine => write!(f, "HardLine"),
            }
        }
        #[cfg(not(debug_assertions))]
        {
            match self {
                Document::Nil => write!(f, "Doc::Nil"),
                Document::Text(_) => write!(f, "Doc::Text"),
                Document::Concat(_) => write!(f, "Doc::Concat"),
                Document::Group(_) => write!(f, "Doc::Group"),
                Document::Indent(_) => write!(f, "Doc::Indent"),
                Document::Line => write!(f, "Doc::Line"),
                Document::SoftLine => write!(f, "Doc::SoftLine"),
                Document::SoftLineSpace => write!(f, "Doc::SoftLineSpace"),
                Document::HardLine => write!(f, "Doc::HardLine"),
            }
        }
    }
}

impl<'a> Document<'a> {
    /// 渲染 Document 为字符串
    pub fn render(&self, config: FormatConfig) -> String {
        Printer::new(config).print(self)
    }

    /// 创建文本文档
    pub fn text<S: Into<Cow<'a, str>>>(text: S) -> Self {
        Document::Text(text.into())
    }

    /// 连接多个文档
    pub fn concat(docs: Vec<Document<'a>>) -> Self {
        Document::Concat(docs)
    }

    /// 创建组合文档
    pub fn group(doc: Document<'a>) -> Self {
        Document::Group(Box::new(doc))
    }

    /// 创建缩进文档
    pub fn indent(doc: Document<'a>) -> Self {
        Document::Indent(Box::new(doc))
    }

    /// 辅助方法：将多个文档用指定分隔符连接
    pub fn join<I>(docs: I, separator: Document<'a>) -> Self
    where
        I: IntoIterator<Item = Document<'a>>,
    {
        let mut result = Vec::new();
        for (i, doc) in docs.into_iter().enumerate() {
            if i > 0 {
                result.push(separator.clone());
            }
            result.push(doc);
        }
        Document::Concat(result)
    }
}

impl<'a> From<Cow<'a, str>> for Document<'a> {
    fn from(s: Cow<'a, str>) -> Self {
        Document::Text(s)
    }
}

impl From<String> for Document<'_> {
    fn from(s: String) -> Self {
        Document::Text(s.into())
    }
}

impl<'a> From<&'a str> for Document<'a> {
    fn from(s: &'a str) -> Self {
        Document::Text(s.into())
    }
}

pub trait JoinDoc<'a> {
    fn join_doc(self, separator: Document<'a>) -> Vec<Document<'a>>;
}

impl<'a, I> JoinDoc<'a> for I
where
    I: IntoIterator<Item = Document<'a>>,
{
    fn join_doc(self, separator: Document<'a>) -> Vec<Document<'a>> {
        let mut result = Vec::new();
        for (i, doc) in self.into_iter().enumerate() {
            if i > 0 {
                result.push(separator.clone());
            }
            result.push(doc);
        }
        result
    }
}
