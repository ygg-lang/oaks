use crate::{
    ast::*,
    language::JasminLanguage,
    lexer::token_type::JasminTokenType,
    parser::{JasminParser, element_type::JasminElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

/// Jasmin AST builder.
pub struct JasminBuilder<'config> {
    config: &'config JasminLanguage,
}

impl<'config> JasminBuilder<'config> {
    /// Creates a new `JasminBuilder` with the given configuration.
    pub fn new(config: &'config JasminLanguage) -> Self {
        Self { config }
    }

    fn build_root(&self, green_tree: &GreenNode<JasminLanguage>, source: &SourceText) -> Result<JasminRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut class = JasminClass { modifiers: vec![], name: "Unknown".to_string(), version: None, methods: vec![], fields: vec![], source_file: None };

        for child in red_root.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    JasminElementType::Class => {
                        let c = self.build_class(n, source)?;
                        class.modifiers = c.modifiers;
                        class.name = c.name;
                        class.version = c.version;
                    }
                    JasminElementType::Method => {
                        class.methods.push(self.build_method(n, source)?);
                    }
                    JasminElementType::Field => {
                        class.fields.push(self.build_field(n, source)?);
                    }
                    JasminElementType::SourceFileKw => {
                        for child in n.children() {
                            if let RedTree::Leaf(t) = child {
                                if t.kind == JasminTokenType::StringNode || t.kind == JasminTokenType::IdentifierToken {
                                    class.source_file = Some(source.get_text_in(t.span.clone().into()).to_string());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(JasminRoot { class })
    }

    fn build_class(&self, node: RedNode<JasminLanguage>, source: &SourceText) -> Result<JasminClass, OakError> {
        let mut modifiers = vec![];
        let mut name = String::new();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                let text = source.get_text_in(t.span.clone().into()).to_string();
                match t.kind {
                    JasminTokenType::Public
                    | JasminTokenType::Private
                    | JasminTokenType::Protected
                    | JasminTokenType::Static
                    | JasminTokenType::Final
                    | JasminTokenType::Abstract
                    | JasminTokenType::Synthetic
                    | JasminTokenType::Deprecated
                    | JasminTokenType::Interface
                    | JasminTokenType::Enum
                    | JasminTokenType::Annotation => {
                        modifiers.push(text);
                    }
                    JasminTokenType::IdentifierToken | JasminTokenType::TypeDescriptor => {
                        name = text;
                    }
                    _ => {}
                }
            }
        }

        Ok(JasminClass { modifiers, name, version: None, methods: vec![], fields: vec![], source_file: None })
    }

    fn build_method(&self, node: RedNode<JasminLanguage>, source: &SourceText) -> Result<JasminMethod, OakError> {
        let mut modifiers = vec![];
        let mut name_and_descriptor = String::new();
        let mut stack_size = None;
        let mut locals_count = None;
        let mut instructions = vec![];

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    let text = source.get_text_in(t.span.clone().into()).to_string();
                    match t.kind {
                        JasminTokenType::Public
                        | JasminTokenType::Private
                        | JasminTokenType::Protected
                        | JasminTokenType::Static
                        | JasminTokenType::Final
                        | JasminTokenType::Native
                        | JasminTokenType::Synchronized
                        | JasminTokenType::Abstract
                        | JasminTokenType::Synthetic
                        | JasminTokenType::Deprecated
                        | JasminTokenType::Varargs
                        | JasminTokenType::Bridge
                        | JasminTokenType::Strictfp => {
                            modifiers.push(text);
                        }
                        JasminTokenType::IdentifierToken | JasminTokenType::TypeDescriptor => {
                            if name_and_descriptor.is_empty() {
                                name_and_descriptor = text;
                            }
                            else if !name_and_descriptor.contains('(') {
                                name_and_descriptor.push(' ');
                                name_and_descriptor.push_str(&text);
                            }
                        }
                        JasminTokenType::Number => {
                            // Captured by instructions usually, but if it's top-level in method
                            if let Ok(val) = text.parse::<u32>() {
                                if stack_size.is_none() {
                                    stack_size = Some(val);
                                }
                                else if locals_count.is_none() {
                                    locals_count = Some(val);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                RedTree::Node(n) => {
                    if n.green.kind == JasminElementType::Instruction {
                        instructions.push(self.build_instruction(n, source)?);
                    }
                }
            }
        }

        Ok(JasminMethod { modifiers, name_and_descriptor, stack_size, locals_count, instructions })
    }

    fn build_instruction(&self, node: RedNode<JasminLanguage>, source: &SourceText) -> Result<JasminInstruction, OakError> {
        let mut tokens = vec![];
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                let text = source.get_text_in(t.span.clone().into()).to_string();
                if !text.trim().is_empty() {
                    tokens.push((t.kind, text));
                }
            }
        }

        if tokens.is_empty() {
            return Ok(JasminInstruction::Simple("nop".to_string()));
        }

        let (_first_kind, first_text) = &tokens[0];

        // Simple heuristic for Jasmin instructions
        if tokens.len() == 1 {
            return Ok(JasminInstruction::Simple(first_text.clone()));
        }

        if tokens.len() >= 2 {
            let instr = first_text.clone();
            let arg = tokens[1..].iter().map(|(_, t)| t.as_str()).collect::<Vec<_>>().join(" ");

            match instr.as_str() {
                "invokevirtual" | "invokespecial" | "invokestatic" | "invokeinterface" | "invokedynamic" => Ok(JasminInstruction::MethodCall { instruction: instr, method_ref: arg }),
                "getstatic" | "putstatic" | "getfield" | "putfield" => Ok(JasminInstruction::FieldAccess { instruction: instr, field_ref: arg }),
                _ => Ok(JasminInstruction::WithArgument { instruction: instr, argument: arg }),
            }
        }
        else {
            Ok(JasminInstruction::Simple(first_text.clone()))
        }
    }

    fn build_field(&self, node: RedNode<JasminLanguage>, source: &SourceText) -> Result<JasminField, OakError> {
        let mut modifiers = vec![];
        let mut name_and_descriptor = String::new();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                let text = source.get_text_in(t.span.clone().into()).to_string();
                match t.kind {
                    JasminTokenType::Public
                    | JasminTokenType::Private
                    | JasminTokenType::Protected
                    | JasminTokenType::Static
                    | JasminTokenType::Final
                    | JasminTokenType::Volatile
                    | JasminTokenType::Transient
                    | JasminTokenType::Synthetic
                    | JasminTokenType::Deprecated
                    | JasminTokenType::Enum => {
                        modifiers.push(text);
                    }
                    JasminTokenType::IdentifierToken | JasminTokenType::TypeDescriptor => {
                        if name_and_descriptor.is_empty() {
                            name_and_descriptor = text;
                        }
                        else {
                            name_and_descriptor.push_str(":");
                            name_and_descriptor.push_str(&text);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(JasminField { modifiers, name_and_descriptor })
    }
}

impl<'config> Builder<JasminLanguage> for JasminBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<JasminLanguage>) -> oak_core::builder::BuildOutput<JasminLanguage> {
        let parser = JasminParser::new(self.config);
        let lexer = crate::lexer::JasminLexer::new(&self.config);
        let mut parse_cache = oak_core::parser::session::ParseSession::<JasminLanguage>::default();
        lexer.lex(source, edits, &mut parse_cache);
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
                    Ok(ast_root) => oak_core::errors::OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(e) => oak_core::errors::OakDiagnostics { result: Err(e.clone()), diagnostics: vec![e] },
                }
            }
            Err(e) => oak_core::errors::OakDiagnostics { result: Err(e.clone()), diagnostics: vec![e] },
        }
    }
}
