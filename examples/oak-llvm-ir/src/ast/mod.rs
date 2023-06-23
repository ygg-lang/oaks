#![doc = include_str!("readme.md")]
use oak_core::{
    Range,
    source::{SourceBuffer, ToSource},
};
#[cfg(feature = "oak-pretty-print")]
use oak_pretty_print::document::Document;
#[cfg(feature = "oak-pretty-print")]
use oak_pretty_print::to_doc::AsDocument;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// LLIR 根节点
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LLirRoot {
    /// Items in the module
    pub items: Vec<LLirItem>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for LLirRoot {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                buffer.push("\n");
            }
            item.to_source(buffer);
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for LLirRoot {
    fn as_document(&self) -> Document<'_> {
        Document::join(self.items.iter().map(|i| i.as_document()), Document::Line)
    }
}

/// LLIR Item
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LLirItem {
    /// Function definition
    Function(LLirFunction),
    /// Global variable
    Global(LLirGlobal),
}

impl ToSource for LLirItem {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            Self::Function(f) => f.to_source(buffer),
            Self::Global(g) => g.to_source(buffer),
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for LLirItem {
    fn as_document(&self) -> Document<'_> {
        match self {
            Self::Function(f) => f.as_document(),
            Self::Global(g) => g.as_document(),
        }
    }
}

/// LLIR Function
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LLirFunction {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<LLirParameter>,
    pub blocks: Vec<LLirBlock>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for LLirFunction {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("define ");
        buffer.push(&self.return_type);
        buffer.push(" @");
        buffer.push(&self.name);
        buffer.push("(");
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                buffer.push(", ");
            }
            param.to_source(buffer);
        }
        buffer.push(") {\n");
        for block in &self.blocks {
            block.to_source(buffer);
        }
        buffer.push("}\n");
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for LLirFunction {
    fn as_document(&self) -> Document<'_> {
        let mut parts = vec![Document::text("define "), Document::text(&self.return_type), Document::text(" @"), Document::text(&self.name), Document::text("(")];

        let params = Document::join(self.parameters.iter().map(|p| p.as_document()), Document::text(", "));
        parts.push(params);
        parts.push(Document::text(") {"));

        let blocks = Document::join(self.blocks.iter().map(|b| b.as_document()), Document::Line);
        parts.push(Document::indent(Document::concat(vec![Document::Line, blocks])));
        parts.push(Document::Line);
        parts.push(Document::text("}"));

        Document::group(Document::concat(parts))
    }
}

/// LLIR Parameter
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LLirParameter {
    pub ty: String,
    pub name: String,
}

impl ToSource for LLirParameter {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(&self.ty);
        buffer.push(" %");
        buffer.push(&self.name);
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for LLirParameter {
    fn as_document(&self) -> Document<'_> {
        Document::concat(vec![Document::text(&self.ty), Document::text(" %"), Document::text(&self.name)])
    }
}

/// LLIR Block
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LLirBlock {
    pub label: Option<String>,
    pub instructions: Vec<LLirInstruction>,
}

impl ToSource for LLirBlock {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        if let Some(label) = &self.label {
            buffer.push(label);
            buffer.push(":\n");
        }
        for inst in &self.instructions {
            buffer.push("  ");
            inst.to_source(buffer);
            buffer.push("\n");
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for LLirBlock {
    fn as_document(&self) -> Document<'_> {
        let mut parts = Vec::new();
        if let Some(label) = &self.label {
            parts.push(Document::text(label));
            parts.push(Document::text(":"));
            parts.push(Document::Line);
        }
        let insts = Document::join(self.instructions.iter().map(|i| i.as_document()), Document::Line);
        parts.push(Document::indent(insts));
        Document::concat(parts)
    }
}

/// LLIR Instruction
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LLirInstruction {
    pub result: Option<String>,
    pub opcode: String,
    pub operands: Vec<String>,
}

impl ToSource for LLirInstruction {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        if let Some(res) = &self.result {
            buffer.push("%");
            buffer.push(res);
            buffer.push(" = ");
        }
        buffer.push(&self.opcode);
        buffer.push(" ");
        for (i, op) in self.operands.iter().enumerate() {
            if i > 0 {
                buffer.push(", ");
            }
            buffer.push(op);
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for LLirInstruction {
    fn as_document(&self) -> Document<'_> {
        let mut parts = Vec::new();
        if let Some(res) = &self.result {
            parts.push(Document::text("%"));
            parts.push(Document::text(res));
            parts.push(Document::text(" = "));
        }
        parts.push(Document::text(&self.opcode));
        parts.push(Document::text(" "));
        let operands = Document::join(self.operands.iter().map(|o| Document::text(o)), Document::text(", "));
        parts.push(operands);
        Document::group(Document::concat(parts))
    }
}

/// LLIR Global Variable
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LLirGlobal {
    pub name: String,
    pub ty: String,
    pub value: String,
    pub is_constant: bool,
}

impl ToSource for LLirGlobal {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("@");
        buffer.push(&self.name);
        buffer.push(" = ");
        if self.is_constant {
            buffer.push("constant ");
        }
        else {
            buffer.push("global ");
        }
        buffer.push(&self.ty);
        buffer.push(" ");
        buffer.push(&self.value);
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for LLirGlobal {
    fn as_document(&self) -> Document<'_> {
        let mut parts = vec![Document::text("@"), Document::text(&self.name), Document::text(" = ")];
        if self.is_constant {
            parts.push(Document::text("constant "));
        }
        else {
            parts.push(Document::text("global "));
        }
        parts.push(Document::text(&self.ty));
        parts.push(Document::text(" "));
        parts.push(Document::text(&self.value));
        Document::group(Document::concat(parts))
    }
}
