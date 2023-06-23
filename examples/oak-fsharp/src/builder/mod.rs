use crate::{ast::*, language::FSharpLanguage, lexer::token_type::FSharpTokenType, parser::element_type::FSharpElementType};
use core::range::Range;
use oak_core::{
    GreenNode, Parser, Source, TokenType,
    builder::{BuildOutput, Builder},
    source::TextEdit,
    tree::red_tree::{RedNode, RedTree},
};

pub struct FSharpBuilder<'config> {
    language: &'config FSharpLanguage,
}

impl<'config> FSharpBuilder<'config> {
    pub fn new(language: &'config FSharpLanguage) -> Self {
        Self { language }
    }

    fn build_root(&self, green: &GreenNode<FSharpLanguage>, source: &str) -> FSharpRoot {
        let red = RedNode::new(green, 0);
        let mut items = Vec::new();

        for child in red.children() {
            if let RedTree::Node(node) = child {
                if let Some(item) = self.build_item(node, source) {
                    items.push(item);
                }
            }
        }

        FSharpRoot { items }
    }

    fn build_item(&self, node: RedNode<FSharpLanguage>, source: &str) -> Option<Item> {
        match node.green.kind {
            FSharpElementType::Namespace => Some(Item::Namespace(self.build_namespace(node, source))),
            FSharpElementType::Module => Some(Item::Module(self.build_module(node, source))),
            FSharpElementType::Open => Some(Item::Open(self.build_open(node, source))),
            FSharpElementType::Let => Some(Item::Binding(self.build_binding(node, source))),
            _ => None,
        }
    }

    fn get_text<'a>(&self, span: Range<usize>, source: &'a str) -> &'a str {
        let start = span.start;
        let end = span.end;
        if start > source.len() || end > source.len() || start > end {
            return "";
        }
        &source[start..end]
    }

    fn build_namespace(&self, node: RedNode<FSharpLanguage>, source: &str) -> NamespaceDeclaration {
        let span = node.span();
        let mut name_parts = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) if leaf.kind == FSharpTokenType::Identifier => {
                    name_parts.push(self.get_text(leaf.span, source));
                }
                RedTree::Node(child_node) => {
                    if let Some(item) = self.build_item(child_node, source) {
                        items.push(item);
                    }
                }
                _ => {}
            }
        }

        NamespaceDeclaration { name: name_parts.join("."), items, span }
    }

    fn build_module(&self, node: RedNode<FSharpLanguage>, source: &str) -> ModuleDeclaration {
        let span = node.span();
        let mut name = String::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) if leaf.kind == FSharpTokenType::Identifier => {
                    name = self.get_text(leaf.span, source).to_string();
                }
                RedTree::Node(child_node) => {
                    if let Some(item) = self.build_item(child_node, source) {
                        items.push(item);
                    }
                }
                _ => {}
            }
        }

        ModuleDeclaration { name, is_top_level: true, items, span }
    }

    fn build_open(&self, node: RedNode<FSharpLanguage>, source: &str) -> OpenDirective {
        let span = node.span();
        let mut path_parts = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) if leaf.kind == FSharpTokenType::Identifier => {
                    path_parts.push(self.get_text(leaf.span, source));
                }
                _ => {}
            }
        }

        OpenDirective { path: path_parts.join("."), span }
    }

    fn build_binding(&self, node: RedNode<FSharpLanguage>, source: &str) -> Binding {
        let span = node.span();
        let mut name = String::new();
        let mut is_rec = false;
        let mut parameters = Vec::new();
        let mut expression = None;
        let mut found_equal = false;

        for child in node.children() {
            match child {
                RedTree::Node(child_node) => {
                    if found_equal && expression.is_none() {
                        expression = Some(self.build_expression(child_node, source));
                    }
                }
                RedTree::Leaf(leaf) => {
                    if found_equal && expression.is_none() && !leaf.kind.is_ignored() && !leaf.kind.is_whitespace() {
                        let text = self.get_text(leaf.span, source).trim();
                        if !text.is_empty() {
                            expression = Some(Expression::Simple(text.to_string()));
                        }
                    }
                    else if leaf.kind == FSharpTokenType::Rec {
                        is_rec = true;
                    }
                    else if leaf.kind == FSharpTokenType::Identifier {
                        if name.is_empty() {
                            name = self.get_text(leaf.span, source).to_string();
                        }
                        else if !found_equal {
                            parameters.push(self.get_text(leaf.span, source).to_string());
                        }
                    }
                    else if leaf.kind == FSharpTokenType::Equal {
                        found_equal = true;
                    }
                }
            }
        }

        Binding { name, is_rec, parameters, expression: expression.unwrap_or_else(|| Expression::Simple(String::new())), span }
    }

    fn build_expression(&self, node: RedNode<FSharpLanguage>, source: &str) -> Expression {
        match node.green.kind {
            FSharpElementType::If => {
                let mut parts = Vec::new();
                for child in node.children() {
                    if let RedTree::Node(child_node) = child {
                        parts.push(self.build_expression(child_node, source));
                    }
                }

                let condition = parts.get(0).cloned().map(Box::new).unwrap_or_else(|| Box::new(Expression::Simple(String::new())));
                let then_branch = parts.get(1).cloned().map(Box::new).unwrap_or_else(|| Box::new(Expression::Simple(String::new())));
                let else_branch = parts.get(2).cloned().map(Box::new);

                Expression::If { condition, then_branch, else_branch }
            }
            FSharpElementType::Expression => Expression::Simple(self.get_text(node.span(), source).trim().to_string()),
            _ => Expression::Simple(self.get_text(node.span(), source).trim().to_string()),
        }
    }
}

impl<'config> Builder<FSharpLanguage> for FSharpBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::parser::ParseCache<FSharpLanguage>) -> BuildOutput<FSharpLanguage> {
        let parser = crate::parser::FSharpParser::new(self.language);
        let output = parser.parse(text, edits, cache);

        let source_str = text.get_text_in(Range { start: 0, end: text.length() });
        let result = output.result.map(|green| self.build_root(green, &source_str));

        oak_core::errors::OakDiagnostics { result, diagnostics: output.diagnostics }
    }
}
