use crate::{ast::*, language::SassLanguage, parser::SassParser};
use oak_core::{Builder, BuilderCache, ElementType, OakDiagnostics, OakError, Parser, Range, SourceText, TextEdit, builder::BuildOutput, source::Source, tree::GreenNode};

/// AST builder for the Sass language.
#[derive(Clone)]
pub struct SassBuilder<'config> {
    config: &'config SassLanguage,
}

impl<'config> SassBuilder<'config> {
    /// Creates a new Sass builder with the given configuration.
    pub fn new(config: &'config SassLanguage) -> Self {
        Self { config }
    }

    /// Builds the Sass AST root from a green tree.
    fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, SassLanguage>, source: &SourceText, cache: &mut impl BuilderCache<SassLanguage>) -> Result<SassRoot, OakError> {
        let mut statements = Vec::new();
        let mut current_offset = 0;

        for child in green_tree.children {
            match child {
                oak_core::tree::GreenTree::Node(node) => {
                    if let Some(stmt) = self.build_statement(node, current_offset, source, cache)? {
                        statements.push(stmt);
                    }
                    current_offset += node.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        Ok(SassRoot { statements, span: (0..current_offset).into() })
    }

    fn build_statement<'a>(&self, node: &'a GreenNode<'a, SassLanguage>, offset: usize, source: &SourceText, cache: &mut impl BuilderCache<SassLanguage>) -> Result<Option<SassStatement>, OakError> {
        if let Some(cached) = cache.get_typed_node::<SassStatement>(node) {
            return Ok(Some(cached.clone()));
        }

        use crate::parser::element_type::SassElementType;

        let result = match node.kind {
            SassElementType::Variable => Some(SassStatement::VariableDefinition(self.build_variable_definition(node, offset, source)?)),
            SassElementType::Selector => Some(SassStatement::RuleSet(self.build_rule_set(node, offset, source, cache)?)),
            SassElementType::Mixin | SassElementType::Function => Some(SassStatement::AtRule(self.build_at_rule(node, offset, source, cache)?)),
            SassElementType::ObjectEntry => Some(SassStatement::Property(self.build_property(node, offset, source)?)),
            _ => {
                if node.kind.is_error() {
                    Some(SassStatement::Error(ErrorNode { message: format!("Unexpected node kind: {:?}", node.kind), span: (offset..offset + node.byte_length as usize).into() }))
                }
                else if node.kind == SassElementType::Object {
                    Some(SassStatement::RuleSet(self.build_rule_set(node, offset, source, cache)?))
                }
                else {
                    None
                }
            }
        };

        if let Some(ref stmt) = result {
            cache.set_typed_node::<SassStatement>(node, stmt.clone());
        }

        Ok(result)
    }

    fn build_variable_definition<'a>(&self, node: &'a GreenNode<'a, SassLanguage>, offset: usize, _source: &SourceText) -> Result<VariableDefinition, OakError> {
        let mut name: Range<usize> = (offset..offset).into();
        let mut value: Range<usize> = (offset..offset).into();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(n) => {
                    if n.kind == crate::parser::element_type::SassElementType::Value {
                        value = (current_offset..current_offset + n.byte_length as usize).into();
                    }
                    current_offset += n.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    if leaf.kind == crate::lexer::token_type::SassTokenType::Variable {
                        name = (current_offset..current_offset + leaf.length as usize).into();
                    }
                    current_offset += leaf.length as usize;
                }
            }
        }

        Ok(VariableDefinition { name, value, span: (offset..offset + node.byte_length as usize).into() })
    }

    fn build_rule_set<'a>(&self, node: &'a GreenNode<'a, SassLanguage>, offset: usize, source: &SourceText, cache: &mut impl BuilderCache<SassLanguage>) -> Result<RuleSet, OakError> {
        let mut selector = (offset..offset).into();
        let mut children = Vec::new();
        let properties = Vec::new(); // Keep for backward compatibility if needed, but we prefer children
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(n) => {
                    use crate::parser::element_type::SassElementType;
                    match n.kind {
                        SassElementType::Selector => {
                            selector = (current_offset..current_offset + n.byte_length as usize).into();
                        }
                        SassElementType::Object | SassElementType::Root => {
                            // Nested rules or properties
                            let mut sub_offset = current_offset;
                            for sub_child in n.children {
                                if let oak_core::tree::GreenTree::Node(sn) = sub_child {
                                    if let Some(stmt) = self.build_statement(sn, sub_offset, source, cache)? {
                                        children.push(stmt);
                                    }
                                }
                                sub_offset += sub_child.len() as usize;
                            }
                        }
                        _ => {
                            if let Some(stmt) = self.build_statement(n, current_offset, source, cache)? {
                                children.push(stmt);
                            }
                        }
                    }
                    current_offset += n.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        Ok(RuleSet { selector, children, properties, span: (offset..offset + node.byte_length as usize).into() })
    }

    fn build_property<'a>(&self, node: &'a GreenNode<'a, SassLanguage>, offset: usize, _source: &SourceText) -> Result<Property, OakError> {
        let mut name: Range<usize> = (offset..offset).into();
        let mut value: Range<usize> = (offset..offset).into();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(n) => {
                    if n.kind == crate::parser::element_type::SassElementType::Value {
                        value = (current_offset..current_offset + n.byte_length as usize).into();
                    }
                    current_offset += n.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    if leaf.kind == crate::lexer::token_type::SassTokenType::Identifier && name.is_empty() {
                        name = (current_offset..current_offset + leaf.length as usize).into();
                    }
                    current_offset += leaf.length as usize;
                }
            }
        }

        Ok(Property { name, value, span: (offset..offset + node.byte_length as usize).into() })
    }

    fn build_at_rule<'a>(&self, node: &'a GreenNode<'a, SassLanguage>, offset: usize, source: &SourceText, cache: &mut impl BuilderCache<SassLanguage>) -> Result<AtRule, OakError> {
        let mut name: Range<usize> = (offset..offset).into();
        let mut params = None;
        let mut body = None;
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(n) => {
                    if n.kind == crate::parser::element_type::SassElementType::Object {
                        let mut sub_statements = Vec::new();
                        let mut sub_offset = current_offset;
                        for sub_child in n.children {
                            if let oak_core::tree::GreenTree::Node(sn) = sub_child {
                                if let Some(stmt) = self.build_statement(sn, sub_offset, source, cache)? {
                                    sub_statements.push(stmt);
                                }
                            }
                            sub_offset += sub_child.len() as usize;
                        }
                        body = Some(sub_statements);
                    }
                    else if params.is_none() && n.kind == crate::parser::element_type::SassElementType::Value {
                        params = Some((current_offset..current_offset + n.byte_length as usize).into());
                    }
                    current_offset += n.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    if name.is_empty() && (leaf.kind == crate::lexer::token_type::SassTokenType::Identifier || leaf.kind == crate::lexer::token_type::SassTokenType::Mixin) {
                        name = (current_offset..current_offset + leaf.length as usize).into();
                    }
                    current_offset += leaf.length as usize;
                }
            }
        }

        Ok(AtRule { name, params, body, span: (offset..offset + node.byte_length as usize).into() })
    }
}

impl<'config> Builder<SassLanguage> for SassBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<SassLanguage>) -> BuildOutput<SassLanguage> {
        let parser = SassParser::new(self.config);
        let mut parse_cache = oak_core::parser::ParseSession::<SassLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text, cache) {
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
