#![doc = include_str!("readme.md")]
use crate::{XmlElementType, XmlLanguage, XmlTokenType};
use core::range::Range;
use oak_core::{source::Source, tree::RedNode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// XML AST 根节点
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct XmlRoot {
    pub value: XmlValue,
}

pub type XmlNode<'a> = RedNode<'a, XmlLanguage>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum XmlValue {
    Element(XmlElement),
    Text(String),
    Comment(String),
    CData(String),
    ProcessingInstruction(XmlPI),
    Fragment(Vec<XmlValue>),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct XmlElement {
    pub name: String,
    pub attributes: Vec<XmlAttribute>,
    pub children: Vec<XmlValue>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct XmlAttribute {
    pub name: String,
    pub value: String,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct XmlPI {
    pub target: String,
    pub data: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl XmlValue {
    pub fn as_element(&self) -> Option<&XmlElement> {
        match self {
            XmlValue::Element(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            XmlValue::Text(s) => Some(s),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            XmlValue::Text(t) => t.clone(),
            XmlValue::Comment(c) => format!("<!--{}-->", c),
            XmlValue::CData(d) => format!("<![CDATA[{}]]>", d),
            XmlValue::ProcessingInstruction(pi) => {
                if let Some(ref data) = pi.data {
                    format!("<?{} {}?>", pi.target, data)
                }
                else {
                    format!("<?{}?>", pi.target)
                }
            }
            XmlValue::Fragment(fs) => {
                let mut s = String::new();
                for f in fs {
                    s.push_str(&f.to_string());
                }
                s
            }
            XmlValue::Element(e) => {
                let mut s = format!("<{}", e.name);
                for attr in &e.attributes {
                    s.push_str(&format!(" {}=\"{}\"", attr.name, attr.value));
                }
                if e.children.is_empty() {
                    s.push_str("/>");
                }
                else {
                    s.push('>');
                    for child in &e.children {
                        s.push_str(&child.to_string());
                    }
                    s.push_str(&format!("</{}>", e.name));
                }
                s
            }
        }
    }
}

pub trait XmlNodeExt<'a> {
    fn tag_name<'s, S: Source + ?Sized>(&self, source: &'s S) -> Option<Cow<'s, str>>;
    fn attributes<S: Source + ?Sized>(&self, source: &S) -> Vec<(String, String)>;
    fn xml_children(&self) -> impl Iterator<Item = RedNode<'a, XmlLanguage>>;
    fn xml_children_recursive(&self) -> impl Iterator<Item = RedNode<'a, XmlLanguage>>;
    fn text<S: Source + ?Sized>(&self, source: &S) -> String;
    fn read_attr<S: Source + ?Sized>(&self, source: &S, name: &str) -> Option<String>;
}

impl<'a> XmlNodeExt<'a> for RedNode<'a, XmlLanguage> {
    fn tag_name<'s, S: Source + ?Sized>(&self, source: &'s S) -> Option<Cow<'s, str>> {
        if self.green.kind != XmlElementType::Element {
            return None;
        }
        for child in self.children() {
            if let Some(node) = child.as_node() {
                if node.green.kind == XmlElementType::StartTag || node.green.kind == XmlElementType::SelfClosingTag {
                    for gc in node.children() {
                        if let Some(leaf) = gc.as_leaf() {
                            if leaf.kind == XmlTokenType::Identifier {
                                return Some(source.get_text_in(leaf.span));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn attributes<S: Source + ?Sized>(&self, source: &S) -> Vec<(String, String)> {
        let mut attrs = Vec::new();
        if self.green.kind != XmlElementType::Element {
            return attrs;
        }
        for child in self.children() {
            if let Some(node) = child.as_node() {
                if node.green.kind == XmlElementType::StartTag || node.green.kind == XmlElementType::SelfClosingTag {
                    for gc in node.children() {
                        if let Some(n) = gc.as_node() {
                            if n.green.kind == XmlElementType::Attribute {
                                let mut name = String::new();
                                let mut value = String::new();
                                for ggc in n.children() {
                                    if let Some(leaf) = ggc.as_leaf() {
                                        if leaf.kind == XmlTokenType::Identifier {
                                            name = source.get_text_in(leaf.span).into_owned();
                                        }
                                        else if leaf.kind == XmlTokenType::AttributeValue {
                                            let v = source.get_text_in(leaf.span);
                                            value = v.trim_matches('"').trim_matches('\'').to_string();
                                        }
                                    }
                                }
                                if !name.is_empty() {
                                    attrs.push((name, value));
                                }
                            }
                        }
                    }
                }
            }
        }
        attrs
    }

    fn xml_children(&self) -> impl Iterator<Item = RedNode<'a, XmlLanguage>> {
        self.children().filter_map(|c| c.as_node().filter(|node| node.green.kind == XmlElementType::Element))
    }

    fn xml_children_recursive(&self) -> impl Iterator<Item = RedNode<'a, XmlLanguage>> {
        let mut stack = Vec::new();
        for child in self.xml_children() {
            stack.push(child);
        }

        std::iter::from_fn(move || {
            let next = stack.pop()?;
            let children = next.xml_children().collect::<Vec<_>>();
            for child in children.into_iter().rev() {
                stack.push(child);
            }
            Some(next)
        })
    }

    fn text<S: Source + ?Sized>(&self, source: &S) -> String {
        let mut text = String::new();
        for child in self.children() {
            if let Some(leaf) = child.as_leaf() {
                if leaf.kind == XmlTokenType::Text {
                    text.push_str(&source.get_text_in(leaf.span));
                }
            }
            else if let Some(node) = child.as_node() {
                if node.green.kind == XmlElementType::Element {
                    text.push_str(&node.text(source));
                }
            }
        }
        text
    }

    fn read_attr<S: Source + ?Sized>(&self, source: &S, name: &str) -> Option<String> {
        self.attributes(source).into_iter().find(|(n, _)| n == name).map(|(_, v)| v)
    }
}
