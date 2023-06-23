use crate::{
    ast::*,
    language::LLvmLanguage,
    lexer::token_type::LLvmTokenType,
    parser::{LLirParser, element_type::LLvmElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, GreenTree, OakDiagnostics, OakError, Parser, TextEdit, source::Source};

pub struct LLirBuilder<'config> {
    config: &'config LLvmLanguage,
}

impl<'config> LLirBuilder<'config> {
    pub fn new(config: &'config LLvmLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<LLvmLanguage> for LLirBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<LLvmLanguage>) -> OakDiagnostics<LLirRoot> {
        let parser = LLirParser::new(self.config);
        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(green_tree) => match self.build_root(green_tree, source) {
                Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                Err(build_error) => {
                    let mut diagnostics = parse_result.diagnostics;
                    diagnostics.push(build_error.clone());
                    OakDiagnostics { result: Err(build_error), diagnostics }
                }
            },
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> LLirBuilder<'config> {
    fn build_root<S: Source + ?Sized>(&self, green_tree: &GreenNode<LLvmLanguage>, source: &S) -> Result<LLirRoot, OakError> {
        let mut items = vec![];
        let mut offset = 0;
        for child in green_tree.children() {
            let len = child.len() as usize;
            if let GreenTree::Node(node) = child {
                if node.kind() == LLvmElementType::Item {
                    let mut item_offset = offset;
                    for item_child in node.children() {
                        let item_len = item_child.len() as usize;
                        if let GreenTree::Node(item_node) = item_child {
                            match item_node.kind() {
                                LLvmElementType::Global => {
                                    items.push(LLirItem::Global(self.build_global(item_node, source, item_offset)?));
                                }
                                LLvmElementType::Function => {
                                    items.push(LLirItem::Function(self.build_function(item_node, source, item_offset)?));
                                }
                                _ => {}
                            }
                        }
                        item_offset += item_len;
                    }
                }
            }
            offset += len;
        }

        Ok(LLirRoot { items, span: (0..green_tree.text_len() as usize).into() })
    }

    fn build_global<S: Source + ?Sized>(&self, green_node: &GreenNode<LLvmLanguage>, source: &S, mut offset: usize) -> Result<LLirGlobal, OakError> {
        let mut name = String::new();
        let mut ty = String::new();
        let mut value = String::new();
        let mut is_constant = false;

        for child in green_node.children() {
            let len = child.len() as usize;
            match child {
                GreenTree::Leaf(leaf) => {
                    let text = source.get_text_in((offset..offset + len).into());
                    match leaf.kind() {
                        LLvmTokenType::GlobalVar => {
                            name = text.trim_start_matches('@').to_string();
                        }
                        LLvmTokenType::Keyword => {
                            if text == "constant" {
                                is_constant = true;
                            }
                        }
                        _ => {}
                    }
                }
                GreenTree::Node(node) => {
                    if node.kind() == LLvmElementType::Identifier {
                        let text = source.get_text_in((offset..offset + len).into());
                        // Simple split by space to get type and value
                        let parts: Vec<&str> = text.split_whitespace().collect();
                        if parts.len() >= 1 {
                            ty = parts[0].to_string();
                        }
                        if parts.len() >= 2 {
                            value = parts[1..].join(" ");
                        }
                    }
                }
            }
            offset += len;
        }

        if name.is_empty() {
            name = "unknown".to_string();
        }

        Ok(LLirGlobal { name, ty, value, is_constant })
    }

    fn build_function<S: Source + ?Sized>(&self, green_node: &GreenNode<LLvmLanguage>, source: &S, mut offset: usize) -> Result<LLirFunction, OakError> {
        let mut name = String::new();
        let mut return_type = String::new();
        let mut parameters = vec![];
        let mut blocks = vec![];

        for child in green_node.children() {
            let len = child.len() as usize;
            match child {
                GreenTree::Leaf(leaf) => {
                    let text = source.get_text_in((offset..offset + len).into());
                    match leaf.kind() {
                        LLvmTokenType::GlobalVar => {
                            name = text.trim_start_matches('@').to_string();
                        }
                        _ => {}
                    }
                }
                GreenTree::Node(node) => {
                    match node.kind() {
                        LLvmElementType::Identifier if return_type.is_empty() => {
                            return_type = source.get_text_in((offset..offset + len).into()).trim().to_string();
                        }
                        LLvmElementType::Parameter => {
                            let mut p_offset = offset;
                            for p_child in node.children() {
                                let p_len = p_child.len() as usize;
                                if let GreenTree::Node(p_node) = p_child {
                                    if p_node.kind() == LLvmElementType::Parameter {
                                        let p_text = source.get_text_in((p_offset..p_offset + p_len).into()).trim().to_string();
                                        if !p_text.is_empty() {
                                            // Split parameter into type and name
                                            let parts: Vec<&str> = p_text.split_whitespace().collect();
                                            if parts.len() >= 2 {
                                                parameters.push(LLirParameter { ty: parts[0].to_string(), name: parts[1].trim_start_matches('%').to_string() });
                                            }
                                            else {
                                                parameters.push(LLirParameter { ty: p_text.clone(), name: "".to_string() });
                                            }
                                        }
                                    }
                                }
                                p_offset += p_len;
                            }
                        }
                        LLvmElementType::Block => {
                            let mut b_offset = offset;
                            let mut instructions = vec![];
                            let mut label = None;

                            for b_child in node.children() {
                                let b_len = b_child.len() as usize;
                                match b_child {
                                    GreenTree::Leaf(leaf) => {
                                        let text = source.get_text_in((b_offset..b_offset + b_len).into());
                                        match leaf.kind() {
                                            LLvmTokenType::Keyword if label.is_none() => {
                                                label = Some(text.to_string());
                                            }
                                            LLvmTokenType::Colon => {
                                                // Keep the label if we found it
                                            }
                                            _ => {}
                                        }
                                    }
                                    GreenTree::Node(b_node) => {
                                        if b_node.kind() == LLvmElementType::Instruction {
                                            let inst_text = source.get_text_in((b_offset..b_offset + b_len).into()).trim().to_string();
                                            if !inst_text.is_empty() {
                                                // Simple instruction parsing: result = opcode operands
                                                if inst_text.contains('=') {
                                                    let parts: Vec<&str> = inst_text.splitn(2, '=').collect();
                                                    let result = Some(parts[0].trim().trim_start_matches('%').to_string());
                                                    let rest = parts[1].trim();
                                                    let rest_parts: Vec<&str> = rest.split_whitespace().collect();
                                                    if !rest_parts.is_empty() {
                                                        let opcode = rest_parts[0].to_string();
                                                        let operands = rest_parts[1..].iter().map(|s| s.trim_matches(',').to_string()).collect();
                                                        instructions.push(LLirInstruction { result, opcode, operands });
                                                    }
                                                }
                                                else {
                                                    let parts: Vec<&str> = inst_text.split_whitespace().collect();
                                                    if !parts.is_empty() {
                                                        let opcode = parts[0].to_string();
                                                        let operands = parts[1..].iter().map(|s| s.trim_matches(',').to_string()).collect();
                                                        instructions.push(LLirInstruction { result: None, opcode, operands });
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                b_offset += b_len;
                            }
                            blocks.push(LLirBlock { label, instructions });
                        }
                        _ => {}
                    }
                }
            }
            offset += len;
        }

        Ok(LLirFunction { name, return_type, parameters, blocks, span: (0..green_node.text_len() as usize).into() })
    }
}
