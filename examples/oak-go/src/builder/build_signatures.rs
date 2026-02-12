use crate::{ast, builder::GoBuilder, language::GoLanguage, lexer::token_type::GoTokenType, parser::element_type::GoElementType};
use oak_core::{
    OakError,
    source::Source,
    tree::red_tree::{RedNode, RedTree},
};

impl<'config> GoBuilder<'config> {
    pub(crate) fn extract_function(&self, node: RedNode<GoLanguage>, source: &oak_core::SourceText, cache: &mut impl oak_core::BuilderCache<GoLanguage>) -> Result<ast::Function, OakError> {
        // if let Some(cached) = cache.get_typed_node::<ast::Function>(&node.green) {
        //    return Ok(cached);
        // }

        let mut name = String::new();
        let mut receiver = None;
        let mut params = vec![];
        let mut return_types = vec![];
        let mut body = ast::Block { statements: vec![], span: (0..0).into() };
        let span = node.span();

        let mut param_list_count = 0;

        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) if leaf.kind == GoTokenType::Identifier => {
                    name = source.get_text_in(leaf.span).trim().to_string();
                }
                RedTree::Node(n) => match n.green.kind {
                    GoElementType::Receiver => {
                        let mut receiver_params = self.extract_params(n, source);
                        if !receiver_params.is_empty() {
                            receiver = Some(receiver_params.remove(0));
                        }
                    }
                    GoElementType::ParameterList => {
                        param_list_count += 1;
                        if param_list_count == 1 {
                            params = self.extract_params(n, source);
                        }
                        else {
                            for p in self.extract_params(n, source) {
                                return_types.push(p.param_type);
                            }
                        }
                    }
                    GoElementType::Block => {
                        body = self.extract_block(n, source)?;
                    }
                    GoElementType::Identifier => {
                        let id = source.get_text_in(n.span()).to_string();
                        if name.is_empty() {
                            name = id;
                        }
                        else {
                            return_types.push(id);
                        }
                    }
                    _ if n.green.kind.is_keyword() => {}
                    _ => {}
                },
                _ => {}
            }
        }

        let func = ast::Function { name, receiver, params, return_types, body, span };
        // cache.set_typed_node(&node.green, std::sync::Arc::new(func.clone()));
        Ok(func)
    }

    pub(crate) fn extract_params(&self, node: RedNode<GoLanguage>, source: &oak_core::SourceText) -> Vec<ast::Parameter> {
        let mut params = vec![];
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == GoElementType::ParameterList {
                    params.extend(self.extract_params(n, source));
                }
                else if n.green.kind == GoElementType::ParameterDecl {
                    let mut p_names = vec![];
                    let mut p_type = String::new();
                    for pd_child in n.children() {
                        match pd_child {
                            RedTree::Leaf(leaf) if leaf.kind == GoTokenType::Identifier => {
                                p_names.push(source.get_text_in(leaf.span).trim().to_string());
                            }
                            RedTree::Node(tn) if tn.green.kind == GoElementType::Identifier => {
                                p_type = source.get_text_in(tn.span()).trim().to_string();
                            }
                            RedTree::Leaf(leaf) if leaf.kind == GoTokenType::Star => {}
                            _ => {}
                        }
                    }
                    if p_names.len() > 1 && p_type.is_empty() {
                        p_type = p_names.pop().unwrap();
                    }

                    for name in p_names {
                        params.push(ast::Parameter { name: name.clone(), param_type: p_type.clone(), span: n.span() });
                    }
                    if params.is_empty() && !p_type.is_empty() {
                        params.push(ast::Parameter { name: String::new(), param_type: p_type, span: n.span() });
                    }
                }
            }
        }
        params
    }
}
