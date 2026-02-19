//! AST builder for the JASM language.

use crate::{
    ast::*,
    language::JasmLanguage,
    lexer::token_type::JasmTokenType,
    parser::{JasmParser, element_type::JasmElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

/// AST builder for the JASM language.
#[derive(Clone)]
pub struct JasmBuilder<'config> {
    /// The language configuration.
    pub config: &'config JasmLanguage,
}

impl<'config> JasmBuilder<'config> {
    /// Creates a new `JasmBuilder`.
    pub fn new(config: &'config JasmLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<JasmLanguage> for JasmBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<JasmLanguage>) -> OakDiagnostics<JasmRoot> {
        let parser = JasmParser::new(self.config);

        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
                    Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error.clone());
                        OakDiagnostics { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> JasmBuilder<'config> {
    fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, JasmLanguage>, source: &SourceText) -> Result<JasmRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut class = None;

        for child in red_root.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == JasmElementType::Class {
                    class = Some(self.build_class(n, source)?);
                }
            }
        }

        Ok(JasmRoot { class: class.unwrap_or_else(|| JasmClass { modifiers: vec![], name: String::new(), version: None, methods: vec![], fields: vec![], source_file: None, super_class: None, interfaces: vec![], annotations: vec![], attributes: vec![] }) })
    }

    fn build_class(&self, node: RedNode<JasmLanguage>, source: &SourceText) -> Result<JasmClass, OakError> {
        let mut modifiers = vec![];
        let mut name = String::new();
        let mut methods = vec![];
        let mut fields = vec![];

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    let text = source.get_text_in(t.span.clone().into()).to_string();
                    match t.kind {
                        JasmTokenType::Public | JasmTokenType::Private | JasmTokenType::Protected | JasmTokenType::Static | JasmTokenType::Final | JasmTokenType::Abstract | JasmTokenType::Synthetic | JasmTokenType::Deprecated => {
                            modifiers.push(text);
                        }
                        JasmTokenType::Identifier => {
                            name = text;
                        }
                        _ => {}
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    JasmElementType::Method => {
                        methods.push(self.build_method(n, source)?);
                    }
                    JasmElementType::Field => {
                        fields.push(self.build_field(n, source)?);
                    }
                    _ => {}
                },
            }
        }

        Ok(JasmClass { modifiers, name, version: None, methods, fields, source_file: None, super_class: None, interfaces: vec![], annotations: vec![], attributes: vec![] })
    }

    fn build_method(&self, node: RedNode<JasmLanguage>, source: &SourceText) -> Result<JasmMethod, OakError> {
        let mut modifiers = vec![];
        let mut name = String::new();
        let mut descriptor = String::new();
        let mut instructions = vec![];
        let mut stack = None;
        let mut locals = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    let text = source.get_text_in(t.span.clone().into()).to_string();
                    match t.kind {
                        JasmTokenType::Public
                        | JasmTokenType::Private
                        | JasmTokenType::Protected
                        | JasmTokenType::Static
                        | JasmTokenType::Final
                        | JasmTokenType::Native
                        | JasmTokenType::Synchronized
                        | JasmTokenType::Synthetic
                        | JasmTokenType::Deprecated
                        | JasmTokenType::Varargs => {
                            modifiers.push(text);
                        }
                        JasmTokenType::Identifier => {
                            if name.is_empty() {
                                name = text;
                            }
                            else if descriptor.is_empty() {
                                descriptor = text;
                            }
                        }
                        JasmTokenType::Number => {
                            if let Ok(val) = text.parse::<u32>() {
                                if stack.is_none() {
                                    stack = Some(val);
                                }
                                else if locals.is_none() {
                                    locals = Some(val);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                RedTree::Node(n) => {
                    if n.green.kind == JasmElementType::Instruction {
                        instructions.push(self.build_instruction(n, source)?);
                    }
                }
            }
        }

        let name_and_descriptor = if descriptor.is_empty() { name } else { format!("{}{}", name, descriptor) };

        Ok(JasmMethod { modifiers, name_and_descriptor, stack_size: stack, locals_count: locals, instructions, exception_handlers: vec![], annotations: vec![], attributes: vec![] })
    }

    fn build_field(&self, node: RedNode<JasmLanguage>, source: &SourceText) -> Result<JasmField, OakError> {
        let mut modifiers = vec![];
        let mut name = String::new();
        let mut descriptor = String::new();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                let text = source.get_text_in(t.span.clone().into()).to_string();
                match t.kind {
                    JasmTokenType::Public | JasmTokenType::Private | JasmTokenType::Protected | JasmTokenType::Static | JasmTokenType::Final | JasmTokenType::Synthetic | JasmTokenType::Deprecated => {
                        modifiers.push(text);
                    }
                    JasmTokenType::Identifier => {
                        if name.is_empty() {
                            name = text;
                        }
                        else {
                            descriptor = text;
                        }
                    }
                    _ => {}
                }
            }
        }

        let name_and_descriptor = if descriptor.is_empty() { name } else { format!("{} {}", name, descriptor) };

        Ok(JasmField { modifiers, name_and_descriptor, annotations: vec![], attributes: vec![] })
    }

    fn build_instruction(&self, node: RedNode<JasmLanguage>, source: &SourceText) -> Result<JasmInstruction, OakError> {
        let mut opcode = String::new();
        let mut operands = vec![];

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                let text = source.get_text_in(t.span.clone().into()).to_string();
                let kind_val = t.kind as u16;
                if kind_val >= JasmTokenType::ALoad0 as u16 && kind_val <= JasmTokenType::Pop as u16 {
                    opcode = text;
                }
                else if matches!(t.kind, JasmTokenType::Identifier | JasmTokenType::Number | JasmTokenType::String) {
                    operands.push(text);
                }
            }
        }

        if operands.is_empty() {
            Ok(JasmInstruction::Simple(opcode))
        }
        else {
            let argument = operands.join(" ");
            if opcode.starts_with("invoke") {
                Ok(JasmInstruction::MethodCall { instruction: opcode, method_ref: argument })
            }
            else if opcode.starts_with("get") || opcode.starts_with("put") {
                Ok(JasmInstruction::FieldAccess { instruction: opcode, field_ref: argument })
            }
            else {
                Ok(JasmInstruction::WithArgument { instruction: opcode, argument })
            }
        }
    }
}
