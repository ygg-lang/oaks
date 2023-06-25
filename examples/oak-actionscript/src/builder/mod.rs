use crate::{
    ast::*,
    language::ActionScriptLanguage,
    parser::{ActionScriptElementType, ActionScriptParser},
};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

/// AST builder for the ActionScript language.
#[derive(Clone)]
pub struct ActionScriptBuilder<'config> {
    /// Language configuration
    config: &'config ActionScriptLanguage,
}

impl<'config> ActionScriptBuilder<'config> {
    /// Creates a new ActionScript builder.
    pub fn new(config: &'config ActionScriptLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<ActionScriptLanguage> for ActionScriptBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<ActionScriptLanguage>) -> oak_core::builder::BuildOutput<ActionScriptLanguage> {
        let parser = ActionScriptParser::new(self.config);
        let lexer = crate::lexer::ActionScriptLexer::new(&self.config);

        let mut cache = oak_core::parser::session::ParseSession::<ActionScriptLanguage>::default();
        lexer.lex(source, edits, &mut cache);
        let parse_result = parser.parse(source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree.clone(), &source_text) {
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

impl<'config> ActionScriptBuilder<'config> {
    /// Builds the root node.
    pub(crate) fn build_root(&self, green_tree: GreenNode<ActionScriptLanguage>, source: &SourceText) -> Result<ActionScriptRoot, OakError> {
        let red_root = RedNode::new(&green_tree, 0);
        let mut items = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(n) = child {
                if let Some(item) = self.build_item(&n, source) {
                    items.push(item);
                }
            }
        }
        Ok(ActionScriptRoot { items })
    }

    fn build_item(&self, node: &RedNode<ActionScriptLanguage>, source: &SourceText) -> Option<ActionScriptItem> {
        match node.green.kind {
            ActionScriptElementType::Package => self.build_package(node, source).map(ActionScriptItem::Package),
            ActionScriptElementType::Class => self.build_class(node, source).map(ActionScriptItem::Class),
            ActionScriptElementType::Interface => self.build_interface(node, source).map(ActionScriptItem::Interface),
            ActionScriptElementType::Function => self.build_function(node, source).map(ActionScriptItem::Function),
            ActionScriptElementType::Variable => self.build_variable(node, source).map(ActionScriptItem::Variable),
            ActionScriptElementType::Import => self.build_import(node, source).map(ActionScriptItem::Import),
            _ => None,
        }
    }

    fn build_package(&self, node: &RedNode<ActionScriptLanguage>, source: &SourceText) -> Option<PackageDeclaration> {
        let mut name = None;
        let mut items = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    ActionScriptElementType::Identifier => {
                        name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                    }
                    ActionScriptElementType::Block => {
                        for block_child in n.children() {
                            if let RedTree::Node(bn) = block_child {
                                if let Some(item) = self.build_item(&bn, source) {
                                    items.push(item);
                                }
                            }
                        }
                    }
                    _ => {
                        if let Some(item) = self.build_item(&n, source) {
                            items.push(item);
                        }
                    }
                }
            }
        }

        Some(PackageDeclaration { name, items, span: node.span() })
    }

    fn build_class(&self, node: &RedNode<ActionScriptLanguage>, source: &SourceText) -> Option<ClassDeclaration> {
        let mut name = None;
        let mut modifiers = Vec::new();
        let extends = None;
        let implements = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    ActionScriptElementType::Public
                    | ActionScriptElementType::Private
                    | ActionScriptElementType::Internal
                    | ActionScriptElementType::Protected
                    | ActionScriptElementType::Static
                    | ActionScriptElementType::Final
                    | ActionScriptElementType::Dynamic => {
                        modifiers.push(source.get_text_in(n.span()).to_string());
                    }
                    ActionScriptElementType::Identifier => {
                        if name.is_none() {
                            name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                    }
                    ActionScriptElementType::Extends => {
                        // The next identifier is the base class
                    }
                    ActionScriptElementType::Implements => {
                        // Subsequent identifiers are interfaces
                    }
                    ActionScriptElementType::Block => {
                        for block_child in n.children() {
                            if let RedTree::Node(bn) = block_child {
                                if let Some(item) = self.build_item(&bn, source) {
                                    items.push(item);
                                }
                            }
                        }
                    }
                    _ => {
                        if let Some(item) = self.build_item(&n, source) {
                            items.push(item);
                        }
                    }
                }
            }
        }

        name.map(|name| ClassDeclaration { name, modifiers, extends, implements, items, span: node.span() })
    }

    fn build_interface(&self, node: &RedNode<ActionScriptLanguage>, source: &SourceText) -> Option<InterfaceDeclaration> {
        let mut name = None;
        let mut extends = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    ActionScriptElementType::Identifier => {
                        if name.is_none() {
                            name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                        else {
                            extends.push(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                    }
                    ActionScriptElementType::Block => {
                        for block_child in n.children() {
                            if let RedTree::Node(bn) = block_child {
                                if let Some(item) = self.build_item(&bn, source) {
                                    items.push(item);
                                }
                            }
                        }
                    }
                    _ => {
                        if let Some(item) = self.build_item(&n, source) {
                            items.push(item);
                        }
                    }
                }
            }
        }

        name.map(|name| InterfaceDeclaration { name, extends, items, span: node.span() })
    }

    fn build_function(&self, node: &RedNode<ActionScriptLanguage>, source: &SourceText) -> Option<FunctionDeclaration> {
        let mut name = None;
        let mut parameters = Vec::new();
        let mut return_type = None;
        let mut found_colon = false;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    ActionScriptElementType::Identifier => {
                        if name.is_none() {
                            name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                        else if found_colon {
                            return_type = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                    }
                    ActionScriptElementType::ParameterList => {
                        parameters = self.build_parameters(&n, source);
                    }
                    ActionScriptElementType::Colon => {
                        found_colon = true;
                    }
                    _ => {}
                }
            }
        }

        name.map(|name| FunctionDeclaration { name, parameters, return_type, span: node.span() })
    }

    fn build_parameters(&self, node: &RedNode<ActionScriptLanguage>, source: &SourceText) -> Vec<Parameter> {
        let mut params = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == ActionScriptElementType::Identifier {
                    params.push(Parameter {
                        name: Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() },
                        type_annotation: None, // Simplified
                    });
                }
            }
        }
        params
    }

    fn build_variable(&self, node: &RedNode<ActionScriptLanguage>, source: &SourceText) -> Option<VariableDeclaration> {
        let mut name = None;
        let mut type_annotation = None;
        let mut is_const = false;
        let mut found_colon = false;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    ActionScriptElementType::Const => {
                        is_const = true;
                    }
                    ActionScriptElementType::Identifier => {
                        if name.is_none() {
                            name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                        else if found_colon {
                            type_annotation = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                    }
                    ActionScriptElementType::Colon => {
                        found_colon = true;
                    }
                    _ => {}
                }
            }
        }

        name.map(|name| VariableDeclaration { name, type_annotation, is_const, span: node.span() })
    }

    fn build_import(&self, node: &RedNode<ActionScriptLanguage>, source: &SourceText) -> Option<ImportDeclaration> {
        let mut path = String::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == ActionScriptElementType::Identifier || n.green.kind == ActionScriptElementType::Dot || n.green.kind == ActionScriptElementType::Star {
                    path.push_str(&source.get_text_in(n.span()));
                }
            }
        }

        if path.is_empty() { None } else { Some(ImportDeclaration { path, span: node.span() }) }
    }
}
