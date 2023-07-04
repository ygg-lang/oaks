/// Jinja Builder module
///
/// This module defines the builder for Jinja templates, used to construct the AST
/// from the parsed green tree.
use oak_core::{
    BuilderCache, GreenNode, GreenTree, OakDiagnostics, OakError, SourceText,
    builder::{BuildOutput, Builder as BuilderTrait},
    source::{Source, TextEdit},
};

use crate::{
    ast::{JinjaElement, JinjaRoot},
    language::JinjaLanguage,
    lexer::JinjaLexer,
    parser::{JinjaParser, element_type::JinjaElementType},
};

/// Builder for Jinja templates
///
/// Converts a parsed green tree into a typed [`JinjaRoot`] AST.
#[derive(Debug, Clone)]
pub struct JinjaBuilder<'a> {
    /// The language instance
    language: &'a JinjaLanguage,
}

impl<'a> JinjaBuilder<'a> {
    /// Creates a new Jinja builder
    pub fn new(language: &'a JinjaLanguage) -> Self {
        Self { language }
    }

    /// Builds the root AST node from a green tree and source text
    fn build_root<'b>(&self, green_tree: &'b GreenNode<'b, JinjaLanguage>, source: &SourceText) -> Result<JinjaRoot, OakError> {
        let span = (0..green_tree.byte_length as usize).into();
        let mut elements = Vec::new();
        let mut offset = 0usize;

        for child in green_tree.children {
            match child {
                GreenTree::Node(node) => {
                    if let Some(element) = self.build_element(node, offset, source) {
                        elements.push(element);
                    }
                    offset += node.byte_length as usize;
                }
                GreenTree::Leaf(leaf) => {
                    offset += leaf.length as usize;
                }
            }
        }

        Ok(JinjaRoot::new(elements, span))
    }

    /// Builds a typed element from a green node
    fn build_element<'b>(&self, node: &'b GreenNode<'b, JinjaLanguage>, offset: usize, source: &SourceText) -> Option<JinjaElement> {
        let span: oak_core::Range<usize> = (offset..offset + node.byte_length as usize).into();

        match node.kind {
            JinjaElementType::Text => {
                let content = source.get_text_in(span.clone()).into_owned();
                Some(JinjaElement::Text { content, span })
            }
            JinjaElementType::Variable => {
                let expression = source.get_text_in(span.clone()).into_owned();
                Some(JinjaElement::Variable { expression, span })
            }
            JinjaElementType::Comment => {
                let content = source.get_text_in(span.clone()).into_owned();
                Some(JinjaElement::Comment { content, span })
            }
            JinjaElementType::IfStatement => {
                let (condition, body, else_body) = self.extract_if_parts(node, offset, source);
                Some(JinjaElement::IfStatement { condition, body, else_body, span })
            }
            JinjaElementType::ForStatement => {
                let (variable, iterable, body, else_body) = self.extract_for_parts(node, offset, source);
                Some(JinjaElement::ForStatement { variable, iterable, body, else_body, span })
            }
            JinjaElementType::Block => {
                let (name, body) = self.extract_block_parts(node, offset, source);
                Some(JinjaElement::Block { name, body, span })
            }
            JinjaElementType::MacroDefinition => {
                let (name, params, body) = self.extract_macro_parts(node, offset, source);
                Some(JinjaElement::MacroDefinition { name, params, body, span })
            }
            JinjaElementType::Extends => {
                let template = source.get_text_in(span.clone()).into_owned();
                Some(JinjaElement::Extends { template, span })
            }
            JinjaElementType::Include => {
                let template = source.get_text_in(span.clone()).into_owned();
                Some(JinjaElement::Include { template, span })
            }
            JinjaElementType::Set => {
                let (name, value) = self.extract_set_parts(node, offset, source);
                Some(JinjaElement::Set { name, value, span })
            }
            JinjaElementType::Import => {
                let expression = source.get_text_in(span.clone()).into_owned();
                Some(JinjaElement::Import { expression, span })
            }
            JinjaElementType::FromImport => {
                let expression = source.get_text_in(span.clone()).into_owned();
                Some(JinjaElement::FromImport { expression, span })
            }
            JinjaElementType::Tag => {
                let (name, content) = self.extract_tag_parts(node, offset, source);
                Some(JinjaElement::Tag { name, content, span })
            }
            _ => None,
        }
    }

    /// Extracts the parts of an if statement
    fn extract_if_parts<'b>(&self, node: &'b GreenNode<'b, JinjaLanguage>, offset: usize, source: &SourceText) -> (String, Vec<JinjaElement>, Option<Vec<JinjaElement>>) {
        let mut condition = String::new();
        let mut body = Vec::new();
        let mut else_body = None;
        let mut current_offset = offset;
        let mut found_tag_end = false;
        let in_else = false;
        let mut else_elements = Vec::new();

        for child in node.children {
            match child {
                GreenTree::Node(n) => {
                    if !found_tag_end {
                        if n.kind == JinjaElementType::Expression || n.kind == JinjaElementType::Identifier || n.kind == JinjaElementType::Literal {
                            let span: oak_core::Range<usize> = (current_offset..current_offset + n.byte_length as usize).into();
                            let text = source.get_text_in(span);
                            if !condition.is_empty() {
                                condition.push(' ');
                            }
                            condition.push_str(&text);
                        }
                        if n.kind == JinjaElementType::Tag {
                            found_tag_end = true;
                        }
                    }
                    else if in_else {
                        if let Some(element) = self.build_element(n, current_offset, source) {
                            else_elements.push(element);
                        }
                    }
                    else {
                        if let Some(element) = self.build_element(n, current_offset, source) {
                            body.push(element);
                        }
                    }
                    current_offset += n.byte_length as usize;
                }
                GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        if !else_elements.is_empty() {
            else_body = Some(else_elements);
        }

        (condition, body, else_body)
    }

    /// Extracts the parts of a for statement
    fn extract_for_parts<'b>(&self, node: &'b GreenNode<'b, JinjaLanguage>, offset: usize, source: &SourceText) -> (String, String, Vec<JinjaElement>, Option<Vec<JinjaElement>>) {
        let mut variable = String::new();
        let mut iterable = String::new();
        let mut body = Vec::new();
        let mut else_body = None;
        let mut current_offset = offset;
        let mut found_tag_end = false;
        let mut found_in = false;
        let in_else = false;
        let mut else_elements = Vec::new();

        for child in node.children {
            match child {
                GreenTree::Node(n) => {
                    if !found_tag_end {
                        if n.kind == JinjaElementType::Identifier {
                            let span: oak_core::Range<usize> = (current_offset..current_offset + n.byte_length as usize).into();
                            let text = source.get_text_in(span).into_owned();
                            if !found_in {
                                variable = text;
                            }
                            else {
                                iterable = text;
                            }
                        }
                        if n.kind == JinjaElementType::Expression {
                            let span: oak_core::Range<usize> = (current_offset..current_offset + n.byte_length as usize).into();
                            let text = source.get_text_in(span).into_owned();
                            if found_in {
                                iterable = text;
                            }
                        }
                        if n.kind == JinjaElementType::Tag {
                            found_tag_end = true;
                        }
                    }
                    else if in_else {
                        if let Some(element) = self.build_element(n, current_offset, source) {
                            else_elements.push(element);
                        }
                    }
                    else {
                        if let Some(element) = self.build_element(n, current_offset, source) {
                            body.push(element);
                        }
                    }
                    current_offset += n.byte_length as usize;
                }
                GreenTree::Leaf(leaf) => {
                    let span: oak_core::Range<usize> = (current_offset..current_offset + leaf.length as usize).into();
                    let text = source.get_text_in(span);
                    if text.trim() == "in" {
                        found_in = true;
                    }
                    current_offset += leaf.length as usize;
                }
            }
        }

        if !else_elements.is_empty() {
            else_body = Some(else_elements);
        }

        (variable, iterable, body, else_body)
    }

    /// Extracts the parts of a block statement
    fn extract_block_parts<'b>(&self, node: &'b GreenNode<'b, JinjaLanguage>, offset: usize, source: &SourceText) -> (String, Vec<JinjaElement>) {
        let mut name = String::new();
        let mut body = Vec::new();
        let mut current_offset = offset;
        let mut found_tag_end = false;
        let mut name_found = false;

        for child in node.children {
            match child {
                GreenTree::Node(n) => {
                    if !found_tag_end {
                        if n.kind == JinjaElementType::Identifier && !name_found {
                            let span: oak_core::Range<usize> = (current_offset..current_offset + n.byte_length as usize).into();
                            name = source.get_text_in(span).into_owned();
                            name_found = true;
                        }
                        if n.kind == JinjaElementType::Tag {
                            found_tag_end = true;
                        }
                    }
                    else if let Some(element) = self.build_element(n, current_offset, source) {
                        body.push(element);
                    }
                    current_offset += n.byte_length as usize;
                }
                GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        (name, body)
    }

    /// Extracts the parts of a macro definition
    fn extract_macro_parts<'b>(&self, node: &'b GreenNode<'b, JinjaLanguage>, offset: usize, source: &SourceText) -> (String, Vec<String>, Vec<JinjaElement>) {
        let mut name = String::new();
        let mut params = Vec::new();
        let mut body = Vec::new();
        let mut current_offset = offset;
        let mut found_tag_end = false;
        let mut name_found = false;

        for child in node.children {
            match child {
                GreenTree::Node(n) => {
                    if !found_tag_end {
                        if n.kind == JinjaElementType::Identifier && !name_found {
                            let span: oak_core::Range<usize> = (current_offset..current_offset + n.byte_length as usize).into();
                            name = source.get_text_in(span).into_owned();
                            name_found = true;
                        }
                        else if n.kind == JinjaElementType::Identifier && name_found {
                            let span: oak_core::Range<usize> = (current_offset..current_offset + n.byte_length as usize).into();
                            params.push(source.get_text_in(span).into_owned());
                        }
                        if n.kind == JinjaElementType::Tag {
                            found_tag_end = true;
                        }
                    }
                    else if let Some(element) = self.build_element(n, current_offset, source) {
                        body.push(element);
                    }
                    current_offset += n.byte_length as usize;
                }
                GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        (name, params, body)
    }

    /// Extracts the parts of a set statement
    fn extract_set_parts<'b>(&self, node: &'b GreenNode<'b, JinjaLanguage>, offset: usize, source: &SourceText) -> (String, String) {
        let mut name = String::new();
        let mut value = String::new();
        let mut current_offset = offset;
        let mut name_found = false;

        for child in node.children {
            match child {
                GreenTree::Node(n) => {
                    if n.kind == JinjaElementType::Identifier && !name_found {
                        let span: oak_core::Range<usize> = (current_offset..current_offset + n.byte_length as usize).into();
                        name = source.get_text_in(span).into_owned();
                        name_found = true;
                    }
                    else if name_found {
                        let span: oak_core::Range<usize> = (current_offset..current_offset + n.byte_length as usize).into();
                        let text = source.get_text_in(span);
                        if !value.is_empty() {
                            value.push(' ');
                        }
                        value.push_str(&text);
                    }
                    current_offset += n.byte_length as usize;
                }
                GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        (name, value)
    }

    /// Extracts the parts of a generic tag
    fn extract_tag_parts<'b>(&self, node: &'b GreenNode<'b, JinjaLanguage>, offset: usize, source: &SourceText) -> (String, String) {
        let mut name = String::new();
        let mut content = String::new();
        let mut current_offset = offset;
        let mut name_found = false;

        for child in node.children {
            match child {
                GreenTree::Node(n) => {
                    if n.kind == JinjaElementType::Identifier && !name_found {
                        let span: oak_core::Range<usize> = (current_offset..current_offset + n.byte_length as usize).into();
                        name = source.get_text_in(span).into_owned();
                        name_found = true;
                    }
                    else if name_found {
                        let span: oak_core::Range<usize> = (current_offset..current_offset + n.byte_length as usize).into();
                        let text = source.get_text_in(span);
                        if !content.is_empty() {
                            content.push(' ');
                        }
                        content.push_str(&text);
                    }
                    current_offset += n.byte_length as usize;
                }
                GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        (name, content)
    }
}

impl<'a> BuilderTrait<JinjaLanguage> for JinjaBuilder<'a> {
    fn build<'b, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], _cache: &'b mut impl BuilderCache<JinjaLanguage>) -> BuildOutput<JinjaLanguage> {
        let parser = JinjaParser::new(self.language);
        let lexer = JinjaLexer::new(self.language);

        let mut parse_cache = oak_core::parser::ParseSession::<JinjaLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, text, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(text.get_text_in((0..text.length()).into()).into_owned());
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
