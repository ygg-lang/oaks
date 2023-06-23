#![doc = include_str!("readme.md")]
use oak_core::source::{SourceBuffer, ToSource};
#[cfg(feature = "oak-pretty-print")]
use oak_pretty_print::{AsDocument, Document};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{string::String, vec::Vec};

/// JASM root node.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JasmRoot {
    pub class: JasmClass,
}

impl ToSource for JasmRoot {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        self.class.to_source(buffer)
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for JasmRoot {
    fn as_document(&self) -> Document<'_> {
        self.class.as_document()
    }
}

/// AST node for a JASM class declaration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JasmClass {
    /// Access modifiers (public, private, etc.).
    pub modifiers: Vec<String>,
    /// Class name.
    pub name: String,
    /// Version information (e.g., 65:0).
    pub version: Option<String>,
    /// List of methods.
    pub methods: Vec<JasmMethod>,
    /// List of fields.
    pub fields: Vec<JasmField>,
    /// Source file information.
    pub source_file: Option<String>,
}

impl ToSource for JasmClass {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        if let Some(source) = &self.source_file {
            buffer.push(".source ");
            buffer.push(source);
            buffer.push("\n")
        }
        buffer.push(".class ");
        for modifier in &self.modifiers {
            buffer.push(modifier);
            buffer.push(" ")
        }
        buffer.push(&self.name);
        buffer.push("\n");
        if let Some(version) = &self.version {
            buffer.push(".version ");
            buffer.push(version);
            buffer.push("\n")
        }
        buffer.push("\n");
        for field in &self.fields {
            field.to_source(buffer);
            buffer.push("\n")
        }
        for method in &self.methods {
            method.to_source(buffer);
            buffer.push("\n")
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for JasmClass {
    fn as_document(&self) -> Document<'_> {
        let mut docs = Vec::new();
        if let Some(source) = &self.source_file {
            docs.push(Document::Text(format!(".source {}\n", source).into()))
        }
        let mut class_line = vec![Document::Text(".class ".into())];
        for modifier in &self.modifiers {
            class_line.push(Document::Text(modifier.clone().into()));
            class_line.push(Document::Text(" ".into()))
        }
        class_line.push(Document::Text(self.name.clone().into()));
        docs.push(Document::Concat(class_line));
        docs.push(Document::Line);

        if let Some(version) = &self.version {
            docs.push(Document::Text(format!(".version {}\n", version).into()))
        }
        docs.push(Document::Line);

        for field in &self.fields {
            docs.push(field.as_document());
            docs.push(Document::Line)
        }
        for method in &self.methods {
            docs.push(method.as_document());
            docs.push(Document::Line)
        }
        Document::Concat(docs)
    }
}

/// AST node for a JASM method declaration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JasmMethod {
    /// Access modifiers (public, static, etc.).
    pub modifiers: Vec<String>,
    /// Method name and type descriptor (e.g., "main":"([Ljava/lang/String)V").
    pub name_and_descriptor: String,
    /// Stack size.
    pub stack_size: Option<u32>,
    /// Number of local variables.
    pub locals_count: Option<u32>,
    /// List of instructions.
    pub instructions: Vec<JasmInstruction>,
}

impl ToSource for JasmMethod {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(".method ");
        for modifier in &self.modifiers {
            buffer.push(modifier);
            buffer.push(" ")
        }
        buffer.push(&self.name_and_descriptor);
        buffer.push("\n");
        if let Some(stack) = self.stack_size {
            buffer.push("    .limit stack ");
            buffer.push(&stack.to_string());
            buffer.push("\n")
        }
        if let Some(locals) = self.locals_count {
            buffer.push("    .limit locals ");
            buffer.push(&locals.to_string());
            buffer.push("\n")
        }
        for inst in &self.instructions {
            buffer.push("    ");
            inst.to_source(buffer);
            buffer.push("\n")
        }
        buffer.push(".end method")
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for JasmMethod {
    fn as_document(&self) -> Document<'_> {
        let mut docs = Vec::new();
        let mut method_line = vec![Document::Text(".method ".into())];
        for modifier in &self.modifiers {
            method_line.push(Document::Text(modifier.clone().into()));
            method_line.push(Document::Text(" ".into()))
        }
        method_line.push(Document::Text(self.name_and_descriptor.clone().into()));
        docs.push(Document::Concat(method_line));
        docs.push(Document::Line);

        let mut body = Vec::new();
        if let Some(stack) = self.stack_size {
            body.push(Document::Text(format!(".limit stack {}\n", stack).into()))
        }
        if let Some(locals) = self.locals_count {
            body.push(Document::Text(format!(".limit locals {}\n", locals).into()))
        }
        for inst in &self.instructions {
            body.push(inst.as_document());
            body.push(Document::Line)
        }

        docs.push(Document::indent(Document::Concat(body)));
        docs.push(Document::Text(".end method".into()));
        Document::Concat(docs)
    }
}

/// AST node for a JASM field declaration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JasmField {
    /// Access modifiers (public, static, etc.).
    pub modifiers: Vec<String>,
    /// Field name and type descriptor (e.g., "value":"I").
    pub name_and_descriptor: String,
}

impl ToSource for JasmField {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(".field ");
        for modifier in &self.modifiers {
            buffer.push(modifier);
            buffer.push(" ")
        }
        buffer.push(&self.name_and_descriptor)
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for JasmField {
    fn as_document(&self) -> Document<'_> {
        let mut docs = vec![Document::Text(".field ".into())];
        for modifier in &self.modifiers {
            docs.push(Document::Text(modifier.clone().into()));
            docs.push(Document::Text(" ".into()))
        }
        docs.push(Document::Text(self.name_and_descriptor.clone().into()));
        Document::Concat(docs)
    }
}

/// AST node for a JASM instruction.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JasmInstruction {
    /// Simple instruction (e.g., aload_0, return).
    Simple(String),
    /// Instruction with an argument (e.g., ldc "Hello").
    WithArgument { instruction: String, argument: String },
    /// Method call instruction (e.g., invokespecial Method java/lang/Object."<init>":"()V").
    MethodCall { instruction: String, method_ref: String },
    /// Field access instruction (e.g., getstatic Field java/lang/System.out:"Ljava/io/PrintStream;").
    FieldAccess { instruction: String, field_ref: String },
}

impl ToSource for JasmInstruction {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            JasmInstruction::Simple(s) => buffer.push(s),
            JasmInstruction::WithArgument { instruction, argument } => {
                buffer.push(instruction);
                buffer.push(" ");
                buffer.push(argument)
            }
            JasmInstruction::MethodCall { instruction, method_ref } => {
                buffer.push(instruction);
                buffer.push(" ");
                buffer.push(method_ref)
            }
            JasmInstruction::FieldAccess { instruction, field_ref } => {
                buffer.push(instruction);
                buffer.push(" ");
                buffer.push(field_ref)
            }
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for JasmInstruction {
    fn as_document(&self) -> Document<'_> {
        match self {
            JasmInstruction::Simple(s) => Document::Text(s.clone().into()),
            JasmInstruction::WithArgument { instruction, argument } => Document::Concat(vec![Document::Text(instruction.clone().into()), Document::Text(" ".into()), Document::Text(argument.clone().into())]),
            JasmInstruction::MethodCall { instruction, method_ref } => Document::Concat(vec![Document::Text(instruction.clone().into()), Document::Text(" ".into()), Document::Text(method_ref.clone().into())]),
            JasmInstruction::FieldAccess { instruction, field_ref } => Document::Concat(vec![Document::Text(instruction.clone().into()), Document::Text(" ".into()), Document::Text(field_ref.clone().into())]),
        }
    }
}
