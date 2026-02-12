use crate::{
    ast::*,
    language::LLvmLanguage,
    lexer::token_type::LLvmTokenType,
    parser::{LLirParser, element_type::LLvmElementType},
};
use oak_core::{Builder, BuilderCache, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

/// A builder for converting LLVM IR green trees into an AST.
pub struct LLirBuilder<'config> {
    /// Language configuration.
    pub config: &'config LLvmLanguage,
}

impl<'config> LLirBuilder<'config> {
    /// Creates a new `LLirBuilder` with the given configuration.
    pub fn new(config: &'config LLvmLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<LLvmLanguage> for LLirBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<LLvmLanguage>) -> OakDiagnostics<LLirRoot> {
        let parser = LLirParser::new(self.config);
        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(RedNode::new(&green_tree, 0), &source_text) {
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

impl<'config> LLirBuilder<'config> {
    fn build_root(&self, node: RedNode<LLvmLanguage>, source: &SourceText) -> Result<LLirRoot, OakError> {
        let mut items = vec![];
        for child in node.children() {
            if let RedTree::Node(node) = child {
                if node.kind::<LLvmElementType>() == LLvmElementType::Item {
                    for item_child in node.children() {
                        if let RedTree::Node(item_node) = item_child {
                            match item_node.kind::<LLvmElementType>() {
                                LLvmElementType::Global => {
                                    items.push(LLirItem::Global(self.build_global(item_node, source)?));
                                }
                                LLvmElementType::Function => {
                                    items.push(LLirItem::Function(self.build_function(item_node, source)?));
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        Ok(LLirRoot { items, span: node.span() })
    }

    fn build_global(&self, node: RedNode<LLvmLanguage>, source: &SourceText) -> Result<LLirGlobal, OakError> {
        let mut name = String::new();
        let mut ty = String::new();
        let mut value = String::new();
        let mut is_constant = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => {
                    let text = source.get_text_in(leaf.span());
                    match leaf.kind {
                        LLvmTokenType::GlobalVar => {
                            name = text.trim_start_matches('@').to_string();
                        }
                        LLvmTokenType::Keyword => {
                            if text == "global" || text == "constant" {
                                is_constant = text == "constant";
                            }
                        }
                        _ => {}
                    }
                }
                RedTree::Node(node) => {
                    if node.kind::<LLvmElementType>() == LLvmElementType::Type {
                        ty = source.get_text_in(node.span()).trim().to_string();
                    }
                    else if node.kind::<LLvmElementType>() == LLvmElementType::Identifier {
                        value = source.get_text_in(node.span()).trim().to_string();
                    }
                }
            }
        }

        if name.is_empty() {
            name = "unknown".to_string();
        }

        Ok(LLirGlobal { name, ty, value, is_constant })
    }

    fn build_function(&self, node: RedNode<LLvmLanguage>, source: &SourceText) -> Result<LLirFunction, OakError> {
        let mut name = String::new();
        let mut return_type = String::new();
        let mut parameters = vec![];
        let mut blocks = vec![];

        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => {
                    let text = source.get_text_in(leaf.span());
                    match leaf.kind {
                        LLvmTokenType::GlobalVar => {
                            name = text.trim_start_matches('@').to_string();
                        }
                        _ => {}
                    }
                }
                RedTree::Node(node) => match node.kind::<LLvmElementType>() {
                    LLvmElementType::Type if return_type.is_empty() => {
                        return_type = source.get_text_in(node.span()).trim().to_string();
                    }
                    LLvmElementType::Parameter => {
                        let mut p_ty = String::new();
                        let mut p_name = String::new();

                        for inner_child in node.children() {
                            match inner_child {
                                RedTree::Node(inner_node) if inner_node.kind::<LLvmElementType>() == LLvmElementType::Type => {
                                    p_ty = source.get_text_in(inner_node.span()).trim().to_string();
                                }
                                RedTree::Leaf(inner_leaf) if inner_leaf.kind == LLvmTokenType::LocalVar => {
                                    p_name = source.get_text_in(inner_leaf.span()).trim_start_matches('%').to_string();
                                }
                                _ => {}
                            }
                        }

                        if !p_ty.is_empty() {
                            parameters.push(LLirParameter { ty: p_ty, name: p_name });
                        }
                    }
                    LLvmElementType::Block => {
                        let mut instructions = vec![];
                        let mut label = None;

                        for b_child in node.children() {
                            match b_child {
                                RedTree::Leaf(leaf) => {
                                    let text = source.get_text_in(leaf.span());
                                    if (leaf.kind == LLvmTokenType::Identifier || leaf.kind == LLvmTokenType::LocalVar || leaf.kind == LLvmTokenType::Number || leaf.kind == LLvmTokenType::Keyword) && label.is_none() {
                                        // Check if the next token is a colon to be sure it's a label
                                        let mut is_label = false;
                                        let mut children = node.children();
                                        while let Some(c) = children.next() {
                                            if let RedTree::Leaf(l) = c {
                                                if l.span == leaf.span {
                                                    if let Some(RedTree::Leaf(next_l)) = children.next() {
                                                        if next_l.kind == LLvmTokenType::Colon {
                                                            is_label = true;
                                                        }
                                                    }
                                                    break;
                                                }
                                            }
                                        }

                                        if is_label {
                                            label = Some(text.trim_end_matches(':').to_string());
                                        }
                                    }
                                }
                                RedTree::Node(b_node) => {
                                    if b_node.kind::<LLvmElementType>() == LLvmElementType::Instruction {
                                        instructions.push(self.build_instruction(b_node, source)?);
                                    }
                                }
                            }
                        }
                        blocks.push(LLirBlock { label, instructions });
                    }
                    _ => {}
                },
            }
        }

        Ok(LLirFunction { name, return_type, parameters, blocks, span: node.span() })
    }

    fn build_instruction(&self, node: RedNode<LLvmLanguage>, source: &SourceText) -> Result<LLirInstruction, OakError> {
        let mut result = None;
        let mut opcode = String::new();
        let mut operands = vec![];

        for i_child in node.children() {
            if let RedTree::Leaf(leaf) = i_child {
                let text = source.get_text_in(leaf.span()).trim().to_string();
                if !text.is_empty() {
                    match leaf.kind {
                        LLvmTokenType::LocalVar if result.is_none() && opcode.is_empty() => {
                            result = Some(text.trim_start_matches('%').to_string());
                        }
                        LLvmTokenType::Keyword | LLvmTokenType::Identifier if opcode.is_empty() => {
                            opcode = text;
                        }
                        LLvmTokenType::Equal | LLvmTokenType::Comma => {}
                        _ => {
                            if !opcode.is_empty() {
                                operands.push(text);
                            }
                        }
                    }
                }
            }
        }

        if opcode.is_empty() {
            // Fallback for metadata or other weird instructions
            opcode = source.get_text_in(node.span()).trim().to_string();
        }

        Ok(LLirInstruction { result, opcode, operands })
    }
}
