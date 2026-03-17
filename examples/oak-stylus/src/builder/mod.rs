#![doc = include_str!("readme.md")]
use crate::{ast::*, language::StylusLanguage, parser::element_type::StylusElementType};
use oak_core::{
    Builder, BuilderCache, OakDiagnostics, OakError, TextEdit,
    source::{Source, SourceText},
    tree::GreenNode,
};

/// AST builder for the Stylus language.
#[derive(Clone)]
pub struct StylusBuilder<'config> {
    config: &'config StylusLanguage,
}

impl<'config> StylusBuilder<'config> {
    /// Create a new Stylus builder.
    pub fn new(config: &'config StylusLanguage) -> Self {
        Self { config }
    }

    /// Builds the Stylus AST root from a green tree.
    fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, StylusLanguage>, source: &SourceText, cache: &mut impl BuilderCache<StylusLanguage>) -> Result<StylusRoot, OakError> {
        let mut items = Vec::new();
        let mut current_offset = 0;

        for child in green_tree.children {
            match child {
                oak_core::tree::GreenTree::Node(node) => {
                    if let Some(item) = self.build_item(node, current_offset, source, cache)? {
                        items.push(item);
                    }
                    current_offset += node.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        let mut root = StylusRoot::new((0..current_offset).into());
        for item in items {
            root.add_item(item);
        }
        Ok(root)
    }

    /// Builds a Stylus item from a green node.
    fn build_item<'a>(&self, node: &'a GreenNode<'a, StylusLanguage>, offset: usize, source: &SourceText, cache: &mut impl BuilderCache<StylusLanguage>) -> Result<Option<StylusItem>, OakError> {
        use StylusElementType::*;

        match node.kind {
            Rule => Ok(Some(StylusItem::Rule(self.build_rule(node, offset, source, cache)?))),
            Comment => Ok(Some(StylusItem::Comment(self.build_comment(node, offset, source)?))),
            _ => Ok(None),
        }
    }

    /// Builds a Stylus rule from a green node.
    fn build_rule<'a>(&self, node: &'a GreenNode<'a, StylusLanguage>, offset: usize, source: &SourceText, cache: &mut impl BuilderCache<StylusLanguage>) -> Result<StylusRule, OakError> {
        let mut selector = String::new();
        let mut properties = Vec::new();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(child_node) => {
                    match child_node.kind {
                        StylusElementType::Selector => {
                            selector = source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string();
                        }
                        StylusElementType::Property => {
                            if let Some(prop) = self.build_property(child_node, current_offset, source)? {
                                properties.push(prop);
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_node.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        Ok(StylusRule { span: (offset..current_offset).into(), selector: selector.trim().to_string(), properties })
    }

    /// Builds a Stylus comment from a green node.
    fn build_comment<'a>(&self, node: &'a GreenNode<'a, StylusLanguage>, offset: usize, source: &SourceText) -> Result<StylusComment, OakError> {
        let text = source.slice((offset..offset + node.byte_length as usize).into()).to_string();
        Ok(StylusComment { span: (offset..offset + node.byte_length as usize).into(), text: text.trim().to_string() })
    }

    /// Builds a Stylus property from a green node.
    fn build_property<'a>(&self, node: &'a GreenNode<'a, StylusLanguage>, offset: usize, source: &SourceText) -> Result<Option<StylusProperty>, OakError> {
        let mut name = String::new();
        let mut value = String::new();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(child_node) => {
                    match child_node.kind {
                        StylusElementType::Property => {
                            name = source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string();
                        }
                        StylusElementType::Value => {
                            value = source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string();
                        }
                        _ => {}
                    }
                    current_offset += child_node.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        if !name.is_empty() { Ok(Some(StylusProperty { span: (offset..current_offset).into(), name: name.trim().to_string(), value: value.trim().to_string() })) } else { Ok(None) }
    }

    /// Builds a Stylus mixin from a green node.
    fn build_mixin<'a>(&self, node: &'a GreenNode<'a, StylusLanguage>, offset: usize, source: &SourceText, cache: &mut impl BuilderCache<StylusLanguage>) -> Result<StylusMixin, OakError> {
        let mut name = String::new();
        let mut params = Vec::new();
        let mut body = Vec::new();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(child_node) => {
                    match child_node.kind {
                        StylusElementType::Identifier => {
                            name = source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string();
                        }
                        StylusElementType::Block => {
                            if let Some(items) = self.build_block(child_node, current_offset, source, cache)? {
                                body.extend(items);
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_node.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        Ok(StylusMixin { span: (offset..current_offset).into(), name: name.trim().to_string(), params, body })
    }

    /// Builds a Stylus variable from a green node.
    fn build_variable<'a>(&self, node: &'a GreenNode<'a, StylusLanguage>, offset: usize, source: &SourceText) -> Result<StylusVariable, OakError> {
        let mut name = String::new();
        let mut value = String::new();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(child_node) => {
                    match child_node.kind {
                        StylusElementType::Identifier => {
                            name = source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string();
                        }
                        StylusElementType::Value => {
                            value = source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string();
                        }
                        _ => {}
                    }
                    current_offset += child_node.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        Ok(StylusVariable { span: (offset..current_offset).into(), name: name.trim().to_string(), value: value.trim().to_string() })
    }

    /// Builds a Stylus import from a green node.
    fn build_import<'a>(&self, node: &'a GreenNode<'a, StylusLanguage>, offset: usize, source: &SourceText) -> Result<StylusImport, OakError> {
        let mut path = String::new();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(child_node) => {
                    if child_node.kind == StylusElementType::String {
                        path = source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string();
                    }
                    current_offset += child_node.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        Ok(StylusImport { span: (offset..current_offset).into(), path: path.trim().to_string() })
    }

    /// Builds a Stylus function from a green node.
    fn build_function<'a>(&self, node: &'a GreenNode<'a, StylusLanguage>, offset: usize, source: &SourceText, cache: &mut impl BuilderCache<StylusLanguage>) -> Result<StylusFunction, OakError> {
        let mut name = String::new();
        let mut params = Vec::new();
        let mut body = Vec::new();
        let mut return_value = None;
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(child_node) => {
                    match child_node.kind {
                        StylusElementType::Identifier => {
                            name = source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string();
                        }
                        StylusElementType::Block => {
                            if let Some(items) = self.build_block(child_node, current_offset, source, cache)? {
                                body.extend(items);
                            }
                        }
                        StylusElementType::Value => {
                            return_value = Some(source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string().trim().to_string());
                        }
                        _ => {}
                    }
                    current_offset += child_node.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        Ok(StylusFunction { span: (offset..current_offset).into(), name: name.trim().to_string(), params, body, return_value })
    }

    /// Builds a Stylus if statement from a green node.
    fn build_if<'a>(&self, node: &'a GreenNode<'a, StylusLanguage>, offset: usize, source: &SourceText, cache: &mut impl BuilderCache<StylusLanguage>) -> Result<StylusIf, OakError> {
        let mut condition = String::new();
        let mut body = Vec::new();
        let mut else_clause: Option<Vec<StylusItem>> = None;
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(child_node) => {
                    match child_node.kind {
                        StylusElementType::Value => {
                            condition = source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string();
                        }
                        StylusElementType::Block => {
                            if let Some(items) = self.build_block(child_node, current_offset, source, cache)? {
                                if else_clause.is_none() {
                                    body.extend(items);
                                }
                                else {
                                    else_clause.as_mut().unwrap().extend(items);
                                }
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_node.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        Ok(StylusIf { span: (offset..current_offset).into(), condition: condition.trim().to_string(), body, else_clause })
    }

    /// Builds a Stylus for loop from a green node.
    fn build_for<'a>(&self, node: &'a GreenNode<'a, StylusLanguage>, offset: usize, source: &SourceText, cache: &mut impl BuilderCache<StylusLanguage>) -> Result<StylusFor, OakError> {
        let mut variable = String::new();
        let mut range = String::new();
        let mut body = Vec::new();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(child_node) => {
                    match child_node.kind {
                        StylusElementType::Identifier => {
                            variable = source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string();
                        }
                        StylusElementType::Value => {
                            range = source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string();
                        }
                        StylusElementType::Block => {
                            if let Some(items) = self.build_block(child_node, current_offset, source, cache)? {
                                body.extend(items);
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_node.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        Ok(StylusFor { span: (offset..current_offset).into(), variable: variable.trim().to_string(), range: range.trim().to_string(), body })
    }

    /// Builds a Stylus while loop from a green node.
    fn build_while<'a>(&self, node: &'a GreenNode<'a, StylusLanguage>, offset: usize, source: &SourceText, cache: &mut impl BuilderCache<StylusLanguage>) -> Result<StylusWhile, OakError> {
        let mut condition = String::new();
        let mut body = Vec::new();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(child_node) => {
                    match child_node.kind {
                        StylusElementType::Value => {
                            condition = source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string();
                        }
                        StylusElementType::Block => {
                            if let Some(items) = self.build_block(child_node, current_offset, source, cache)? {
                                body.extend(items);
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_node.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        Ok(StylusWhile { span: (offset..current_offset).into(), condition: condition.trim().to_string(), body })
    }

    /// Builds a Stylus parameter from a green node.
    fn build_param<'a>(&self, node: &'a GreenNode<'a, StylusLanguage>, offset: usize, source: &SourceText) -> Result<Option<StylusParam>, OakError> {
        let mut name = String::new();
        let mut default = None;
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(child_node) => {
                    match child_node.kind {
                        StylusElementType::Identifier => {
                            name = source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string();
                        }
                        StylusElementType::Value => {
                            default = Some(source.slice((current_offset..current_offset + child_node.byte_length as usize).into()).to_string().trim().to_string());
                        }
                        _ => {}
                    }
                    current_offset += child_node.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        if !name.is_empty() { Ok(Some(StylusParam { span: (offset..current_offset).into(), name: name.trim().to_string(), default })) } else { Ok(None) }
    }

    /// Builds a Stylus block from a green node.
    fn build_block<'a>(&self, node: &'a GreenNode<'a, StylusLanguage>, offset: usize, source: &SourceText, cache: &mut impl BuilderCache<StylusLanguage>) -> Result<Option<Vec<StylusItem>>, OakError> {
        let mut items = Vec::new();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::tree::GreenTree::Node(child_node) => {
                    if let Some(item) = self.build_item(child_node, current_offset, source, cache)? {
                        items.push(item);
                    }
                    current_offset += child_node.byte_length as usize;
                }
                oak_core::tree::GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        if !items.is_empty() { Ok(Some(items)) } else { Ok(None) }
    }
}

impl<'config> Builder<StylusLanguage> for StylusBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<StylusLanguage>) -> oak_core::builder::BuildOutput<StylusLanguage> {
        let parser = crate::parser::StylusParser::new(self.config);
        let lexer = crate::lexer::StylusLexer::new(&self.config);
        let mut parse_cache = oak_core::parser::session::ParseSession::<StylusLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let text = source.get_text_in((0..source.length()).into());
                let source_text = SourceText::new(text.to_string());
                match self.build_root(green_tree, &source_text, cache) {
                    Ok(ast) => OakDiagnostics { result: Ok(ast), diagnostics: parse_result.diagnostics },
                    Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
                }
            }
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
