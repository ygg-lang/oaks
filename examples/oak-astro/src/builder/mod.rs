use crate::{
    SvelteElementType,
    ast::{Attribute, Directive, SvelteAttribute, SvelteAttributeValue, SvelteBlock, SvelteBranch, SvelteElement, SvelteExpression, SvelteNode, SvelteRoot, SvelteText},
    language::SvelteLanguage,
    lexer::token_type::SvelteTokenType,
};
use core::range::Range;
use oak_core::{
    Builder, BuilderCache, GreenNode, GreenTree, OakDiagnostics, OakError, Parser, Source,
    builder::BuildOutput,
    source::{SourceText, TextEdit},
};

/// Svelte AST builder.
pub struct SvelteBuilder;

impl SvelteBuilder {
    /// Creates a new `SvelteBuilder`.
    pub fn new() -> Self {
        Self
    }

    fn build_root<'a>(&self, green_tree: &GreenNode<'a, SvelteLanguage>, source: &SourceText) -> Result<SvelteRoot, OakError> {
        let mut nodes = Vec::new();
        let mut current_offset = 0;

        for child in green_tree.children {
            match child {
                GreenTree::Node(node) => {
                    if let Some(node) = self.build_node(node, current_offset, source)? {
                        nodes.push(node);
                    }
                    current_offset += node.byte_length as usize;
                }
                GreenTree::Leaf(leaf) => {
                    current_offset += leaf.length as usize;
                }
            }
        }

        Ok(SvelteRoot { nodes })
    }

    fn build_node<'a>(&self, node: &GreenNode<'a, SvelteLanguage>, offset: usize, source: &SourceText) -> Result<Option<SvelteNode>, OakError> {
        match node.kind {
            SvelteElementType::Element => Ok(Some(SvelteNode::Element(self.build_element(node, offset, source)?))),
            SvelteElementType::Expression => Ok(Some(SvelteNode::Expression(self.build_expression(node, offset, source)?))),
            SvelteElementType::Block => Ok(Some(SvelteNode::Block(self.build_block(node, offset, source)?))),
            SvelteElementType::TextNode => {
                let content = source.get_text_in(Range { start: offset, end: offset + node.byte_length as usize }).to_string();
                Ok(Some(SvelteNode::Text(SvelteText { content, span: Range { start: offset, end: offset + node.byte_length as usize } })))
            }
            SvelteElementType::CommentNode => {
                let content = source.get_text_in(Range { start: offset, end: offset + node.byte_length as usize }).to_string();
                Ok(Some(SvelteNode::Comment(content)))
            }
            _ => Ok(None),
        }
    }

    fn build_element<'a>(&self, node: &GreenNode<'a, SvelteLanguage>, offset: usize, source: &SourceText) -> Result<SvelteElement, OakError> {
        let mut tag_name = String::new();
        let mut attributes = Vec::new();
        let mut children = Vec::new();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                GreenTree::Node(n) => {
                    match n.kind {
                        SvelteElementType::Tag => {
                            let mut sub_offset = current_offset;
                            for sub_child in n.children {
                                match sub_child {
                                    GreenTree::Leaf(t) if t.kind == SvelteTokenType::Identifier => {
                                        if tag_name.is_empty() {
                                            tag_name = source.get_text_in(Range { start: sub_offset, end: sub_offset + t.length as usize }).to_string();
                                        }
                                    }
                                    GreenTree::Node(attr_node) if attr_node.kind == SvelteElementType::Attribute => {
                                        attributes.push(self.build_attribute(attr_node, sub_offset, source)?);
                                    }
                                    _ => {}
                                }
                                sub_offset += sub_child.len() as usize;
                            }
                        }
                        _ => {
                            if let Some(child_node) = self.build_node(n, current_offset, source)? {
                                children.push(child_node);
                            }
                        }
                    }
                    current_offset += n.byte_length as usize;
                }
                GreenTree::Leaf(t) => {
                    if t.kind == SvelteTokenType::Text {
                        let content = source.get_text_in(Range { start: current_offset, end: current_offset + t.length as usize }).to_string();
                        if !content.trim().is_empty() {
                            children.push(SvelteNode::Text(SvelteText { content, span: Range { start: current_offset, end: current_offset + t.length as usize } }));
                        }
                    }
                    current_offset += t.length as usize;
                }
            }
        }

        Ok(SvelteElement { tag_name, attributes, children, span: Range { start: offset, end: offset + node.byte_length as usize } })
    }

    fn build_attribute<'a>(&self, node: &GreenNode<'a, SvelteLanguage>, offset: usize, source: &SourceText) -> Result<SvelteAttribute, OakError> {
        let mut name = String::new();
        let mut value = None;
        let mut current_offset = offset;

        // Check if it's a directive (contains ':') or shorthand
        let mut is_directive = false;
        let mut directive_kind = String::new();

        for child in node.children {
            match child {
                GreenTree::Leaf(t) => {
                    match t.kind {
                        SvelteTokenType::Identifier => {
                            let text = source.get_text_in(Range { start: current_offset, end: current_offset + t.length as usize }).to_string();
                            if name.is_empty() {
                                name = text;
                            }
                            else {
                                // Probably directive name after ':'
                                directive_kind = name;
                                name = text;
                                is_directive = true;
                            }
                        }
                        SvelteTokenType::Colon => {
                            is_directive = true;
                        }
                        SvelteTokenType::StringLiteral => {
                            let raw = source.get_text_in(Range { start: current_offset, end: current_offset + t.length as usize });
                            if raw.len() >= 2 {
                                value = Some(SvelteAttributeValue::Literal(raw[1..raw.len() - 1].to_string()));
                            }
                            else {
                                value = Some(SvelteAttributeValue::Literal(raw.to_string()));
                            }
                        }
                        _ => {}
                    }
                    current_offset += t.length as usize;
                }
                GreenTree::Node(n) => {
                    if n.kind == SvelteElementType::Expression {
                        let expr = self.build_expression(n, current_offset, source)?;
                        let expr_text = expr.expression.clone();
                        value = Some(SvelteAttributeValue::Expression(expr.expression));
                        if name.is_empty() {
                            // Shorthand: {name}
                            name = expr_text;
                        }
                    }
                    current_offset += n.byte_length as usize;
                }
            }
        }

        if is_directive {
            Ok(SvelteAttribute::Directive(Directive {
                kind: directive_kind,
                name,
                expression: value.and_then(|v| match v {
                    SvelteAttributeValue::Expression(e) => Some(e),
                    SvelteAttributeValue::Literal(l) => Some(l),
                }),
                span: Range { start: offset, end: offset + node.byte_length as usize },
            }))
        }
        else {
            Ok(SvelteAttribute::Attribute(Attribute { name, value, span: Range { start: offset, end: offset + node.byte_length as usize } }))
        }
    }

    fn build_expression<'a>(&self, node: &GreenNode<'a, SvelteLanguage>, offset: usize, source: &SourceText) -> Result<SvelteExpression, OakError> {
        let mut expression = String::new();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                GreenTree::Leaf(t) => {
                    if t.kind != SvelteTokenType::LeftBrace && t.kind != SvelteTokenType::RightBrace {
                        expression.push_str(&source.get_text_in(Range { start: current_offset, end: current_offset + t.length as usize }));
                    }
                    current_offset += t.length as usize;
                }
                GreenTree::Node(n) => {
                    expression.push_str(&source.get_text_in(Range { start: current_offset, end: current_offset + n.byte_length as usize }));
                    current_offset += n.byte_length as usize;
                }
            }
        }

        Ok(SvelteExpression { expression: expression.trim().to_string(), span: Range { start: offset, end: offset + node.byte_length as usize } })
    }

    fn build_block<'a>(&self, node: &GreenNode<'a, SvelteLanguage>, offset: usize, source: &SourceText) -> Result<SvelteBlock, OakError> {
        let mut kind = String::new();
        let mut expression = String::new();
        let mut children = Vec::new();
        let mut branches = Vec::new();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                GreenTree::Node(n) => {
                    match n.kind {
                        SvelteElementType::BlockHeader => {
                            let mut sub_offset = current_offset;
                            for sub_child in n.children {
                                match sub_child {
                                    GreenTree::Leaf(t) => {
                                        match t.kind {
                                            SvelteTokenType::Identifier if kind.is_empty() => {
                                                kind = source.get_text_in(Range { start: sub_offset, end: sub_offset + t.length as usize }).to_string();
                                            }
                                            SvelteTokenType::HashBrace | SvelteTokenType::RightBrace => {}
                                            _ => {
                                                expression.push_str(&source.get_text_in(Range { start: sub_offset, end: sub_offset + t.length as usize }));
                                            }
                                        }
                                        sub_offset += t.length as usize;
                                    }
                                    GreenTree::Node(sub_n) => {
                                        expression.push_str(&source.get_text_in(Range { start: sub_offset, end: sub_offset + sub_n.byte_length as usize }));
                                        sub_offset += sub_n.byte_length as usize;
                                    }
                                }
                            }
                        }
                        SvelteElementType::BlockContent => {
                            let mut sub_offset = current_offset;
                            for sub_child in n.children {
                                if let GreenTree::Node(sub_n) = sub_child {
                                    if let Some(child_node) = self.build_node(sub_n, sub_offset, source)? {
                                        children.push(child_node);
                                    }
                                    sub_offset += sub_n.byte_length as usize;
                                }
                                else if let GreenTree::Leaf(t) = sub_child {
                                    if t.kind == SvelteTokenType::Text {
                                        let content = source.get_text_in(Range { start: sub_offset, end: sub_offset + t.length as usize }).to_string();
                                        if !content.trim().is_empty() {
                                            children.push(SvelteNode::Text(SvelteText { content, span: Range { start: sub_offset, end: sub_offset + t.length as usize } }));
                                        }
                                    }
                                    sub_offset += t.length as usize;
                                }
                            }
                        }
                        SvelteElementType::BlockBranch => {
                            branches.push(self.build_branch(n, current_offset, source)?);
                        }
                        _ => {}
                    }
                    current_offset += n.byte_length as usize;
                }
                GreenTree::Leaf(t) => {
                    current_offset += t.length as usize;
                }
            }
        }

        Ok(SvelteBlock { kind, expression: expression.trim().to_string(), children, branches, span: Range { start: offset, end: offset + node.byte_length as usize } })
    }

    fn build_branch<'a>(&self, node: &GreenNode<'a, SvelteLanguage>, offset: usize, source: &SourceText) -> Result<SvelteBranch, OakError> {
        let mut name = String::new();
        let mut expression = None;
        let mut children = Vec::new();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                GreenTree::Node(n) => {
                    match n.kind {
                        SvelteElementType::BlockHeader => {
                            let mut sub_offset = current_offset;
                            for sub_child in n.children {
                                match sub_child {
                                    GreenTree::Leaf(t) => {
                                        match t.kind {
                                            SvelteTokenType::Identifier if name.is_empty() => {
                                                name = source.get_text_in(Range { start: sub_offset, end: sub_offset + t.length as usize }).to_string();
                                            }
                                            SvelteTokenType::ColonBrace | SvelteTokenType::RightBrace => {}
                                            _ => {
                                                let expr = expression.get_or_insert_with(String::new);
                                                expr.push_str(&source.get_text_in(Range { start: sub_offset, end: sub_offset + t.length as usize }));
                                            }
                                        }
                                        sub_offset += t.length as usize;
                                    }
                                    GreenTree::Node(sub_n) => {
                                        let expr = expression.get_or_insert_with(String::new);
                                        expr.push_str(&source.get_text_in(Range { start: sub_offset, end: sub_offset + sub_n.byte_length as usize }));
                                        sub_offset += sub_n.byte_length as usize;
                                    }
                                }
                            }
                        }
                        SvelteElementType::BlockContent => {
                            let mut sub_offset = current_offset;
                            for sub_child in n.children {
                                if let GreenTree::Node(sub_n) = sub_child {
                                    if let Some(child_node) = self.build_node(sub_n, sub_offset, source)? {
                                        children.push(child_node);
                                    }
                                    sub_offset += sub_n.byte_length as usize;
                                }
                                else if let GreenTree::Leaf(t) = sub_child {
                                    if t.kind == SvelteTokenType::Text {
                                        let content = source.get_text_in(Range { start: sub_offset, end: sub_offset + t.length as usize }).to_string();
                                        if !content.trim().is_empty() {
                                            children.push(SvelteNode::Text(SvelteText { content, span: Range { start: sub_offset, end: sub_offset + t.length as usize } }));
                                        }
                                    }
                                    sub_offset += t.length as usize;
                                }
                            }
                        }
                        _ => {}
                    }
                    current_offset += n.byte_length as usize;
                }
                GreenTree::Leaf(t) => {
                    current_offset += t.length as usize;
                }
            }
        }

        Ok(SvelteBranch { name, expression: expression.map(|e| e.trim().to_string()), children, span: Range { start: offset, end: offset + node.byte_length as usize } })
    }
}

impl Builder<SvelteLanguage> for SvelteBuilder {
    fn build<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<SvelteLanguage>) -> BuildOutput<SvelteLanguage> {
        let source = SourceText::new(text.get_text_in(Range { start: 0, end: text.length() }).to_string());
        let config = SvelteLanguage::default();
        let parser = crate::parser::SvelteParser::new(&config);
        let parse_output = parser.parse(text, edits, cache);

        let mut diagnostics = Vec::new();
        for error in parse_output.diagnostics {
            diagnostics.push(error);
        }

        let result = parse_output.result.and_then(|green_tree| self.build_root(green_tree, &source));

        OakDiagnostics { result, diagnostics }
    }
}
