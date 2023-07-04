use crate::{
    VocLanguage,
    ast::{Attribute, ScriptAst, StyleAst, StyleRule, TemplateNode, VxDocument},
    lexer::{VocLexer, VocTokenType},
    parser::{VocParser, element_type::VocElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, Range, SourceText, TextEdit, parser::session::ParseSession, source::Source};
use oak_von::ast::VonString;

/// A builder for VOC AST and diagnostic results.
#[derive(Clone)]
pub struct VocBuilder<'config> {
    /// Language configuration reference.
    config: &'config VocLanguage,
}

impl<'config> VocBuilder<'config> {
    /// Creates a new `VocBuilder` with the given configuration.
    pub fn new(config: &'config VocLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<VocLanguage> for VocBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<VocLanguage>) -> OakDiagnostics<VxDocument> {
        let parser = VocParser::new(self.config);
        let lexer = VocLexer::new(self.config);

        let mut parse_session = ParseSession::<VocLanguage>::default();
        lexer.lex(source, edits, &mut parse_session);
        let parse_result = parser.parse(source, edits, &mut parse_session);

        match parse_result.result {
            Ok(green_tree) => {
                let text = source.get_text_in((0..source.length()).into());
                let source_text = SourceText::new(text.into_owned());
                match self.build_root(&green_tree, &source_text) {
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

impl<'config> VocBuilder<'config> {
    /// Builds the root VxDocument from a green tree.
    fn build_root<'a>(&self, green_tree: &GreenNode<'a, VocLanguage>, source: &SourceText) -> Result<VxDocument, OakError> {
        let root_node = match green_tree.children.first() {
            Some(oak_core::GreenTree::Node(n)) => n,
            _ => return Err(OakError::unexpected_eof(0, None)),
        };
        self.build_vx_document(root_node, 0, source)
    }

    /// Builds a VxDocument from a green node.
    fn build_vx_document<'a>(&self, node: &GreenNode<'a, VocLanguage>, offset: usize, source: &SourceText) -> Result<VxDocument, OakError> {
        let mut template = None;
        let mut script = None;
        let mut style = None;
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::GreenTree::Node(n) => {
                    match n.kind {
                        VocElementType::TemplateSection => {
                            template = Some(self.build_template(n, current_offset, source)?);
                        }
                        VocElementType::ScriptSection => {
                            script = Some(self.build_script(n, current_offset, source)?);
                        }
                        VocElementType::StyleSection => {
                            style = Some(self.build_style(n, current_offset, source)?);
                        }
                        _ => {}
                    }
                    current_offset += n.byte_length as usize;
                }
                oak_core::GreenTree::Leaf(l) => {
                    current_offset += l.length as usize;
                }
            }
        }

        Ok(VxDocument { template, script, style })
    }

    /// Builds a TemplateNode from a template section green node.
    fn build_template<'a>(&self, section: &GreenNode<'a, VocLanguage>, offset: usize, source: &SourceText) -> Result<TemplateNode, OakError> {
        let mut children = Vec::new();
        let mut current_offset = offset;

        for child in section.children {
            match child {
                oak_core::GreenTree::Node(n) => {
                    if n.kind == VocElementType::TemplateElement {
                        children.push(self.build_template_element(n, current_offset, source)?);
                    }
                    current_offset += n.byte_length as usize;
                }
                oak_core::GreenTree::Leaf(l) => {
                    if l.kind == VocTokenType::Text {
                        let l_span: Range<usize> = Range::from(current_offset..current_offset + l.length as usize);
                        let text = source.get_text_in(l_span.clone()).to_string();
                        if !text.trim().is_empty() {
                            children.push(TemplateNode::Text(VonString { value: text, span: l_span }));
                        }
                    }
                    current_offset += l.length as usize;
                }
            }
        }

        if children.len() == 1 { Ok(children.pop().unwrap()) } else { Ok(TemplateNode::element("fragment", Vec::new(), children)) }
    }

    /// Builds a TemplateNode from a template element green node.
    fn build_template_element<'a>(&self, element: &GreenNode<'a, VocLanguage>, offset: usize, source: &SourceText) -> Result<TemplateNode, OakError> {
        let mut tag = String::new();
        let mut tag_span: Range<usize> = Range::from(0..0);
        let mut attributes = Vec::new();
        let mut children = Vec::new();
        let mut found_tag = false;
        let mut current_offset = offset;

        for child in element.children {
            match child {
                oak_core::GreenTree::Leaf(l) => {
                    let l_span: Range<usize> = Range::from(current_offset..current_offset + l.length as usize);
                    match l.kind {
                        VocTokenType::TagOpen => {
                            if !found_tag {
                                let text = source.get_text_in(l_span.clone()).to_string();
                                let (t, a) = parse_tag_open(&text);
                                tag = t;
                                tag_span = l_span;
                                attributes = a;
                                found_tag = true;
                            }
                        }
                        VocTokenType::SelfCloseTag => {
                            let text = source.get_text_in(l_span.clone()).to_string();
                            let (t, a) = parse_tag_open(&text);
                            return Ok(TemplateNode::Element { tag: VonString { value: t, span: l_span }, attributes: a, children: Vec::new() });
                        }
                        VocTokenType::Text => {
                            let text = source.get_text_in(l_span.clone()).to_string();
                            if !text.trim().is_empty() {
                                children.push(TemplateNode::Text(VonString { value: text, span: l_span }));
                            }
                        }
                        _ => {}
                    }
                    current_offset += l.length as usize;
                }
                oak_core::GreenTree::Node(n) => {
                    if n.kind == VocElementType::TemplateElement {
                        children.push(self.build_template_element(n, current_offset, source)?);
                    }
                    current_offset += n.byte_length as usize;
                }
            }
        }

        Ok(TemplateNode::Element { tag: VonString { value: tag, span: tag_span }, attributes, children })
    }

    /// Builds a ScriptAst from a script section green node.
    fn build_script<'a>(&self, section: &GreenNode<'a, VocLanguage>, offset: usize, source: &SourceText) -> Result<ScriptAst, OakError> {
        let mut raw_source = String::new();
        let mut past_open = false;
        let mut current_offset = offset;

        for child in section.children {
            match child {
                oak_core::GreenTree::Leaf(l) => {
                    let l_span: Range<usize> = Range::from(current_offset..current_offset + l.length as usize);
                    match l.kind {
                        VocTokenType::SectionOpen => {
                            past_open = true;
                        }
                        VocTokenType::SectionClose => {
                            break;
                        }
                        VocTokenType::Whitespace => {
                            if past_open {
                                raw_source.push(' ');
                            }
                        }
                        _ => {
                            if past_open {
                                raw_source.push_str(&source.get_text_in(l_span));
                            }
                        }
                    }
                    current_offset += l.length as usize;
                }
                oak_core::GreenTree::Node(n) => {
                    current_offset += n.byte_length as usize;
                }
            }
        }

        Ok(ScriptAst { raw_source: raw_source.trim().to_string() })
    }

    /// Builds a StyleAst from a style section green node.
    fn build_style<'a>(&self, section: &GreenNode<'a, VocLanguage>, offset: usize, source: &SourceText) -> Result<StyleAst, OakError> {
        let mut rules = Vec::new();
        let mut current_offset = offset;

        for child in section.children {
            match child {
                oak_core::GreenTree::Node(n) => {
                    if n.kind == VocElementType::StyleRule {
                        if let Some(rule) = self.build_style_rule(n, current_offset, source)? {
                            rules.push(rule);
                        }
                    }
                    current_offset += n.byte_length as usize;
                }
                oak_core::GreenTree::Leaf(l) => {
                    current_offset += l.length as usize;
                }
            }
        }

        Ok(StyleAst { rules })
    }

    /// Builds a StyleRule from a style rule green node.
    fn build_style_rule<'a>(&self, rule_node: &GreenNode<'a, VocLanguage>, offset: usize, source: &SourceText) -> Result<Option<StyleRule>, OakError> {
        let mut selector = String::new();
        let mut properties = Vec::new();
        let mut current_offset = offset;

        for child in rule_node.children {
            match child {
                oak_core::GreenTree::Leaf(l) => {
                    let l_span: Range<usize> = Range::from(current_offset..current_offset + l.length as usize);
                    match l.kind {
                        VocTokenType::Selector => {
                            selector = source.get_text_in(l_span).trim().to_string();
                        }
                        VocTokenType::Property | VocTokenType::Variable => {
                            let text = source.get_text_in(l_span).to_string();
                            if let Some((key, value)) = text.trim().split_once(':') {
                                properties.push((key.trim().to_string(), value.trim().trim_end_matches(';').to_string()));
                            }
                        }
                        _ => {}
                    }
                    current_offset += l.length as usize;
                }
                oak_core::GreenTree::Node(n) => {
                    if n.kind == VocElementType::StyleProperty {
                        let mut prop_offset = current_offset;
                        for prop_child in n.children {
                            match prop_child {
                                oak_core::GreenTree::Leaf(l) => {
                                    if l.kind == VocTokenType::Property {
                                        let l_span: Range<usize> = Range::from(prop_offset..prop_offset + l.length as usize);
                                        let text = source.get_text_in(l_span).to_string();
                                        if let Some((key, value)) = text.trim().split_once(':') {
                                            properties.push((key.trim().to_string(), value.trim().trim_end_matches(';').to_string()));
                                        }
                                    }
                                    prop_offset += l.length as usize;
                                }
                                oak_core::GreenTree::Node(pn) => {
                                    prop_offset += pn.byte_length as usize;
                                }
                            }
                        }
                    }
                    current_offset += n.byte_length as usize;
                }
            }
        }

        if selector.is_empty() { Ok(None) } else { Ok(Some(StyleRule { selector, properties })) }
    }
}

/// Parses a tag open token text into a tag name and attributes.
fn parse_tag_open(text: &str) -> (String, Vec<Attribute>) {
    let inner = text.trim_start_matches('<').trim_end_matches('>');
    let inner = inner.trim_end_matches('/');
    let inner = inner.trim();
    let mut parts = inner.splitn(2, |c: char| c.is_whitespace());
    let tag = parts.next().unwrap_or("").to_string();
    let attrs_text = parts.next().unwrap_or("");
    let mut attributes = Vec::new();
    for part in attrs_text.split_whitespace() {
        if let Some((key, value)) = part.split_once('=') {
            let value = value.trim_matches('"');
            attributes.push(Attribute { name: VonString { value: key.to_string(), span: Range::from(0..0) }, value: VonString { value: value.to_string(), span: Range::from(0..0) } });
        }
    }
    (tag, attributes)
}
