#![doc = include_str!("readme.md")]
use core::range::Range;
use oak_core::source::{SourceBuffer, ToSource};
#[cfg(feature = "oak-pretty-print")]
use oak_pretty_print::{AsDocument, Document};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// MSIL 抽象语法树的根节点
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MsilRoot {
    /// 指令、类、方法等项目列表
    pub items: Vec<Item>,
}

impl ToSource for MsilRoot {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        for item in &self.items {
            item.to_source(buffer);
            buffer.push("\n")
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for MsilRoot {
    fn as_document(&self) -> Document<'_> {
        Document::join(self.items.iter().map(|i| i.as_document()), Document::Line)
    }
}

/// MSIL 中的顶级项目
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Item {
    /// 程序集定义
    Assembly(Assembly),
    /// 模块定义
    Module(String),
    /// 类定义
    Class(Class),
    /// 外部程序集引用
    AssemblyExtern(String),
}

impl ToSource for Item {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            Item::Assembly(a) => a.to_source(buffer),
            Item::Module(m) => {
                buffer.push(".module ");
                buffer.push(m)
            }
            Item::Class(c) => c.to_source(buffer),
            Item::AssemblyExtern(a) => {
                buffer.push(".assembly extern ");
                buffer.push(a);
                buffer.push(" {}")
            }
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for Item {
    fn as_document(&self) -> Document<'_> {
        match self {
            Item::Assembly(a) => a.as_document(),
            Item::Module(m) => Document::Text(format!(".module {}", m).into()),
            Item::Class(c) => c.as_document(),
            Item::AssemblyExtern(a) => Document::Text(format!(".assembly extern {} {{}}", a).into()),
        }
    }
}

/// 程序集定义
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Assembly {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for Assembly {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(".assembly ");
        buffer.push(&self.name);
        buffer.push(" {}")
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for Assembly {
    fn as_document(&self) -> Document<'_> {
        Document::Text(format!(".assembly {} {{}}", self.name).into())
    }
}

/// 类定义
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Class {
    pub name: String,
    pub methods: Vec<Method>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for Class {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(".class public auto ansi beforefieldinit ");
        buffer.push(&self.name);
        buffer.push("\n{");
        for method in &self.methods {
            buffer.push("\n");
            method.to_source(buffer)
        }
        buffer.push("\n}")
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for Class {
    fn as_document(&self) -> Document<'_> {
        Document::Concat(vec![
            Document::Text(format!(".class public auto ansi beforefieldinit {}", self.name).into()),
            Document::Line,
            Document::Text("{".into()),
            Document::indent(Document::join(self.methods.iter().map(|m| m.as_document()), Document::Line)),
            Document::Text("}".into()),
        ])
    }
}

/// 方法定义
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Method {
    pub name: String,
    pub instructions: Vec<Instruction>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for Method {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(".method public hidebysig static void ");
        buffer.push(&self.name);
        buffer.push("() cil managed\n{");
        if !self.instructions.is_empty() {
            buffer.push("\n    .entrypoint");
            for inst in &self.instructions {
                buffer.push("\n    ");
                inst.to_source(buffer)
            }
        }
        buffer.push("\n}")
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for Method {
    fn as_document(&self) -> Document<'_> {
        let mut body = vec![Document::Text(".entrypoint".into()), Document::Line];
        body.extend(self.instructions.iter().map(|i| i.as_document()));

        Document::Concat(vec![
            Document::Text(format!(".method public hidebysig static void {}() cil managed", self.name).into()),
            Document::Line,
            Document::Text("{".into()),
            Document::indent(Document::join(body, Document::Line)),
            Document::Text("}".into()),
        ])
    }
}

/// MSIL 指令
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Instruction {
    /// 无参数指令
    Simple(String),
    /// 字符串参数指令 (ldstr)
    String(String),
    /// 调用指令 (call)
    Call(String),
}

impl ToSource for Instruction {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            Instruction::Simple(s) => buffer.push(s),
            Instruction::String(s) => {
                buffer.push("ldstr \"");
                buffer.push(s);
                buffer.push("\"")
            }
            Instruction::Call(s) => {
                buffer.push("call ");
                buffer.push(s)
            }
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for Instruction {
    fn as_document(&self) -> Document<'_> {
        match self {
            Instruction::Simple(s) => Document::Text(s.clone().into()),
            Instruction::String(s) => Document::Text(format!("ldstr \"{}\"", s).into()),
            Instruction::Call(s) => Document::Text(format!("call {}", s).into()),
        }
    }
}
