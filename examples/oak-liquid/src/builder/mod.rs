/// Liquid Builder module
///
/// This module defines the builder for Liquid templates, used to construct
/// the typed AST from the parsed green tree.
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, builder::BuildOutput, parser::session::ParseSession, source::Source};

use crate::{
    ast::*,
    language::LiquidLanguage,
    parser::{LiquidParser, element_type::LiquidElementType},
};

/// Builder for Liquid templates.
///
/// Converts a parsed [`GreenNode`] tree into a strongly-typed [`LiquidRoot`] AST
/// by traversing the tree using [`RedNode`] and extracting source text spans.
#[derive(Debug, Clone)]
pub struct LiquidBuilder<'a> {
    /// The language configuration instance.
    language: &'a LiquidLanguage,
}

impl<'a> LiquidBuilder<'a> {
    /// Creates a new Liquid builder with the given language configuration.
    pub fn new(language: &'a LiquidLanguage) -> Self {
        Self { language }
    }

    /// Builds the root AST node from a parsed green tree.
    ///
    /// Traverses the children of the root green node and converts each
    /// child into a typed [`LiquidNode`].
    fn build_root<'b>(&self, green_tree: &'b GreenNode<'b, LiquidLanguage>, source: &SourceText) -> Result<LiquidRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut children = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(n) = child {
                if let Some(node) = self.build_node(n, source) {
                    children.push(node);
                }
            }
        }

        Ok(LiquidRoot { span: red_root.span(), children })
    }

    /// Builds a typed [`LiquidNode`] from a red node.
    ///
    /// Dispatches based on the element type of the green node and
    /// constructs the appropriate typed AST node.
    fn build_node(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> Option<LiquidNode> {
        match node.green.kind {
            LiquidElementType::Text => Some(self.build_text(node, source)),
            LiquidElementType::Variable => Some(self.build_variable(node, source)),
            LiquidElementType::Tag => Some(self.build_tag(node, source)),
            LiquidElementType::Comment => Some(self.build_comment(node, source)),
            LiquidElementType::IfStatement => Some(self.build_if(node, source)),
            LiquidElementType::ForStatement => Some(self.build_for(node, source)),
            LiquidElementType::Block => Some(self.build_block(node, source)),
            LiquidElementType::Assign => Some(self.build_assign(node, source)),
            LiquidElementType::Capture => Some(self.build_capture(node, source)),
            LiquidElementType::Case => Some(self.build_case(node, source)),
            LiquidElementType::Include => Some(self.build_include(node, source)),
            LiquidElementType::Render => Some(self.build_render(node, source)),
            LiquidElementType::Unless => Some(self.build_unless(node, source)),
            LiquidElementType::Raw => Some(self.build_raw(node, source)),
            LiquidElementType::Break => Some(self.build_break(node)),
            LiquidElementType::Continue => Some(self.build_continue(node)),
            LiquidElementType::Tablerow => Some(self.build_tablerow(node, source)),
            LiquidElementType::Cycle => Some(self.build_cycle(node, source)),
            LiquidElementType::MacroDefinition => Some(self.build_macro(node, source)),
            LiquidElementType::Error => Some(self.build_error(node)),
            _ => None,
        }
    }

    /// Collects child [`LiquidNode`] values from a red node's children.
    fn build_children(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> Vec<LiquidNode> {
        let mut children = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if let Some(child_node) = self.build_node(n, source) {
                    children.push(child_node);
                }
            }
        }
        children
    }

    /// Builds a text content node.
    fn build_text(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let text = source.get_text_in(span.clone()).to_string();
        LiquidNode::Text(LiquidText { text, span })
    }

    /// Builds a variable output expression node.
    fn build_variable(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let children = self.build_children(node, source);
        LiquidNode::Variable(LiquidVariable { children, span })
    }

    /// Builds a generic tag statement node.
    fn build_tag(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let children = self.build_children(node, source);
        LiquidNode::Tag(LiquidTag { children, span })
    }

    /// Builds a comment block node.
    fn build_comment(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let text = source.get_text_in(span.clone()).to_string();
        LiquidNode::Comment(LiquidComment { text, span })
    }

    /// Builds an if/elsif/else conditional block node.
    fn build_if(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let children = self.build_children(node, source);
        LiquidNode::If(LiquidIf { children, span })
    }

    /// Builds a for loop iteration block node.
    fn build_for(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let children = self.build_children(node, source);
        LiquidNode::For(LiquidFor { children, span })
    }

    /// Builds a block placeholder statement node.
    fn build_block(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let children = self.build_children(node, source);
        LiquidNode::Block(LiquidBlock { children, span })
    }

    /// Builds an assign statement node.
    fn build_assign(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let children = self.build_children(node, source);
        LiquidNode::Assign(LiquidAssign { children, span })
    }

    /// Builds a capture block statement node.
    fn build_capture(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let children = self.build_children(node, source);
        LiquidNode::Capture(LiquidCapture { children, span })
    }

    /// Builds a case/when conditional block node.
    fn build_case(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let children = self.build_children(node, source);
        LiquidNode::Case(LiquidCase { children, span })
    }

    /// Builds an include statement node.
    fn build_include(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let children = self.build_children(node, source);
        LiquidNode::Include(LiquidInclude { children, span })
    }

    /// Builds a render statement node.
    fn build_render(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let children = self.build_children(node, source);
        LiquidNode::Render(LiquidRender { children, span })
    }

    /// Builds an unless negated conditional block node.
    fn build_unless(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let children = self.build_children(node, source);
        LiquidNode::Unless(LiquidUnless { children, span })
    }

    /// Builds a raw block node for unprocessed content.
    fn build_raw(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let text = source.get_text_in(span.clone()).to_string();
        LiquidNode::Raw(LiquidRaw { text, span })
    }

    /// Builds a break statement node.
    fn build_break(&self, node: RedNode<LiquidLanguage>) -> LiquidNode {
        LiquidNode::Break(LiquidBreak { span: node.span() })
    }

    /// Builds a continue statement node.
    fn build_continue(&self, node: RedNode<LiquidLanguage>) -> LiquidNode {
        LiquidNode::Continue(LiquidContinue { span: node.span() })
    }

    /// Builds a tablerow iteration statement node.
    fn build_tablerow(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let children = self.build_children(node, source);
        LiquidNode::Tablerow(LiquidTablerow { children, span })
    }

    /// Builds a cycle statement node.
    fn build_cycle(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let children = self.build_children(node, source);
        LiquidNode::Cycle(LiquidCycle { children, span })
    }

    /// Builds a macro definition block node.
    fn build_macro(&self, node: RedNode<LiquidLanguage>, source: &SourceText) -> LiquidNode {
        let span = node.span();
        let children = self.build_children(node, source);
        LiquidNode::Macro(LiquidMacro { children, span })
    }

    /// Builds an error node for malformed constructs.
    fn build_error(&self, node: RedNode<LiquidLanguage>) -> LiquidNode {
        LiquidNode::Error(LiquidError { span: node.span() })
    }
}

impl<'a> Builder<LiquidLanguage> for LiquidBuilder<'a> {
    fn build<'b, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'b mut impl BuilderCache<LiquidLanguage>) -> BuildOutput<LiquidLanguage> {
        let parser = LiquidParser::new(self.language);
        let mut parse_cache = ParseSession::<LiquidLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
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
