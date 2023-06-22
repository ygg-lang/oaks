use crate::{ast::*, kind::TypstSyntaxKind, language::TypstLanguage, parser::TypstParser};
use oak_core::{
    Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree,
    source::{Source, TextEdit},
};

/// Typst 语言的 AST 构建器
#[derive(Clone)]
pub struct TypstBuilder<'config> {
    config: &'config TypstLanguage,
}

impl<'config> TypstBuilder<'config> {
    pub fn new(config: &'config TypstLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<TypstLanguage> for TypstBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<TypstLanguage>) -> OakDiagnostics<TypstRoot> {
        let parser = TypstParser::new(self.config);

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

impl<'config> TypstBuilder<'config> {
    pub(crate) fn build_root<S: Source + ?Sized>(&self, green_tree: &GreenNode<TypstLanguage>, source: &S) -> Result<TypstRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut root = TypstRoot::new(red_root.span());

        for child in red_root.children() {
            if let Some(item) = self.build_tree(child, source)? {
                root.items.push(item);
            }
        }

        Ok(root)
    }

    fn build_tree<S: Source + ?Sized>(&self, tree: RedTree<TypstLanguage>, source: &S) -> Result<Option<TypstItem>, OakError> {
        match tree {
            RedTree::Node(node) => self.build_item(node, source),
            RedTree::Leaf(leaf) => match leaf.kind {
                TypstSyntaxKind::Whitespace | TypstSyntaxKind::Newline => Ok(Some(TypstItem::Space)),
                _ => Ok(Some(TypstItem::Text(source.get_text_in(leaf.span).to_string()))),
            },
        }
    }

    fn build_item<S: Source + ?Sized>(&self, node: RedNode<TypstLanguage>, source: &S) -> Result<Option<TypstItem>, OakError> {
        match node.kind::<TypstSyntaxKind>() {
            TypstSyntaxKind::Text => Ok(Some(TypstItem::Text(source.get_text_in(node.span()).to_string()))),
            TypstSyntaxKind::Paragraph => {
                let mut root = TypstRoot::new(node.span());
                for child in node.children() {
                    if let Some(item) = self.build_tree(child, source)? {
                        root.items.push(item);
                    }
                }
                Ok(Some(TypstItem::Block(root)))
            }
            TypstSyntaxKind::Math | TypstSyntaxKind::InlineMath | TypstSyntaxKind::DisplayMath => {
                let mut root = TypstRoot::new(node.span());
                for child in node.children() {
                    if let Some(item) = self.build_tree(child, source)? {
                        root.items.push(item);
                    }
                }
                Ok(Some(TypstItem::Math(root)))
            }
            TypstSyntaxKind::Heading => {
                let text = source.get_text_in(node.span());
                let mut level = 0;
                for ch in text.chars() {
                    if ch == '=' {
                        level += 1;
                    }
                    else {
                        break;
                    }
                }
                let content_text = text.trim_start_matches('=').trim_start().to_string();
                let mut content = TypstRoot::new(node.span());
                content.items.push(TypstItem::Text(content_text));

                Ok(Some(TypstItem::Heading(TypstHeading { level, content })))
            }
            TypstSyntaxKind::Strong => {
                let mut root = TypstRoot::new(node.span());
                for child in node.children() {
                    if let Some(item) = self.build_tree(child, source)? {
                        root.items.push(item);
                    }
                }
                Ok(Some(TypstItem::Strong(root)))
            }
            TypstSyntaxKind::Emphasis => {
                let mut root = TypstRoot::new(node.span());
                for child in node.children() {
                    if let Some(item) = self.build_tree(child, source)? {
                        root.items.push(item);
                    }
                }
                Ok(Some(TypstItem::Emphasis(root)))
            }
            TypstSyntaxKind::ListItem => {
                let mut root = TypstRoot::new(node.span());
                for child in node.children() {
                    if let Some(item) = self.build_tree(child, source)? {
                        root.items.push(item);
                    }
                }
                Ok(Some(TypstItem::ListItem(root)))
            }
            TypstSyntaxKind::EnumItem => {
                let mut root = TypstRoot::new(node.span());
                for child in node.children() {
                    if let Some(item) = self.build_tree(child, source)? {
                        root.items.push(item);
                    }
                }
                Ok(Some(TypstItem::EnumItem(root)))
            }
            TypstSyntaxKind::Link => Ok(Some(TypstItem::Link(TypstLink { url: source.get_text_in(node.span()).to_string(), content: None }))),
            TypstSyntaxKind::Raw => Ok(Some(TypstItem::Raw(source.get_text_in(node.span()).to_string()))),
            TypstSyntaxKind::Quote => {
                let mut root = TypstRoot::new(node.span());
                // In our simple parser, Quote just contains the whole line
                // But for the converter, we want the content inside #quote[...]
                let text = source.get_text_in(node.span()).to_string();
                let content = if text.starts_with("#quote[") && text.ends_with("]") {
                    &text[7..text.len() - 1]
                }
                else if text.starts_with("#quote") {
                    &text[6..]
                }
                else {
                    &text
                };
                root.items.push(TypstItem::Text(content.trim().to_string()));
                Ok(Some(TypstItem::Quote(root)))
            }
            _ => {
                let mut has_children = false;
                for _ in node.children() {
                    has_children = true;
                    break;
                }
                if has_children {
                    // Could recursively build, but for now just return None for unknown blocks
                }
                Ok(None)
            }
        }
    }
}
