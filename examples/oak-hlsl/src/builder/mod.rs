use crate::{
    ast::*,
    language::HlslLanguage,
    lexer::token_type::HlslTokenType,
    parser::{HlslParser, element_type::HlslElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

/// AST builder for the HLSL language.
pub struct HlslBuilder<'config> {
    /// Language configuration.
    config: &'config HlslLanguage,
}

impl<'config> HlslBuilder<'config> {
    /// Creates a new HLSL builder with the given language configuration.
    pub fn new(config: &'config HlslLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<HlslLanguage> for HlslBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<HlslLanguage>) -> oak_core::builder::BuildOutput<HlslLanguage> {
        let parser = HlslParser::new(self.config);
        let lexer = crate::lexer::HlslLexer::new(&self.config);

        // For now, we don't have a proper incremental builder implementation here
        // We just run the lexer and parser
        let mut lexer_cache = oak_core::parser::session::ParseSession::<HlslLanguage>::default();
        let _lex_output = lexer.lex(text, edits, &mut lexer_cache);

        let mut parser_cache = oak_core::parser::session::ParseSession::<HlslLanguage>::default();
        let parse_result = parser.parse(text, edits, &mut parser_cache);

        let OakDiagnostics { result, diagnostics } = parse_result;

        match result {
            Ok(green_tree) => {
                let source_text = SourceText::new(text.get_text_in((0..text.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
                    Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = diagnostics;
                        diagnostics.push(build_error.clone());
                        OakDiagnostics { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics },
        }
    }
}

impl<'config> HlslBuilder<'config> {
    fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, HlslLanguage>, source: &SourceText) -> Result<HlslRoot, OakError> {
        let red_root = RedNode::<HlslLanguage>::new(green_tree, 0);
        let mut declarations = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(n) = child {
                match n.element_type() {
                    k if k == HlslElementType::FunctionDeclaration => {
                        declarations.push(Declaration::Function(self.build_function(n, source)?));
                    }
                    k if k == HlslElementType::VariableDeclaration => {
                        declarations.push(Declaration::Variable(self.build_variable(n, source)?));
                    }
                    k if k == HlslElementType::StructDeclaration => {
                        declarations.push(Declaration::Struct(self.build_struct(n, source)?));
                    }
                    _ => {}
                }
            }
        }

        Ok(HlslRoot { declarations })
    }

    fn build_function(&self, node: RedNode<HlslLanguage>, source: &SourceText) -> Result<Function, OakError> {
        let mut name = Identifier { name: String::new(), span: (0..0).into() };
        let mut return_type = String::new();
        let mut parameters = Vec::new();
        let span = node.span();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == HlslTokenType::Identifier {
                        name = Identifier { name: source.get_text_in(t.span.clone().into()).to_string(), span: t.span.clone() };
                    }
                    else if self.is_basic_type(t.kind) {
                        return_type = source.get_text_in(t.span.clone().into()).to_string();
                    }
                }
                RedTree::Node(n) => {
                    if n.element_type() == HlslElementType::ParameterList {
                        parameters = self.build_parameter_list(n, source)?;
                    }
                }
            }
        }

        Ok(Function { name, return_type, parameters, span })
    }

    fn build_parameter_list(&self, node: RedNode<HlslLanguage>, source: &SourceText) -> Result<Vec<Parameter>, OakError> {
        let mut parameters = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.element_type() == HlslElementType::Parameter {
                    parameters.push(self.build_parameter(n, source)?);
                }
            }
        }
        Ok(parameters)
    }

    fn build_parameter(&self, node: RedNode<HlslLanguage>, source: &SourceText) -> Result<Parameter, OakError> {
        let mut name = Identifier { name: String::new(), span: (0..0).into() };
        let mut type_name = String::new();
        let span = node.span();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == HlslTokenType::Identifier {
                    name = Identifier { name: source.get_text_in(t.span.clone().into()).to_string(), span: t.span.clone() };
                }
                else if self.is_basic_type(t.kind) {
                    type_name = source.get_text_in(t.span.clone().into()).to_string();
                }
            }
        }

        Ok(Parameter { name, type_name, span })
    }

    fn build_variable(&self, node: RedNode<HlslLanguage>, source: &SourceText) -> Result<Variable, OakError> {
        let mut name = Identifier { name: String::new(), span: (0..0).into() };
        let mut type_name = String::new();
        let span = node.span();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == HlslTokenType::Identifier {
                    name = Identifier { name: source.get_text_in(t.span.clone().into()).to_string(), span: t.span.clone() };
                }
                else if self.is_basic_type(t.kind) {
                    type_name = source.get_text_in(t.span.clone().into()).to_string();
                }
            }
        }

        Ok(Variable { name, type_name, span })
    }

    fn build_struct(&self, node: RedNode<HlslLanguage>, source: &SourceText) -> Result<Struct, OakError> {
        let mut name = Identifier { name: String::new(), span: (0..0).into() };
        let mut members = Vec::new();
        let span = node.span();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == HlslTokenType::Identifier {
                        name = Identifier { name: source.get_text_in(t.span.clone().into()).to_string(), span: t.span.clone() };
                    }
                }
                RedTree::Node(n) => {
                    if n.element_type() == HlslElementType::VariableDeclaration {
                        members.push(self.build_variable(n, source)?);
                    }
                }
            }
        }

        Ok(Struct { name, members, span })
    }

    fn is_basic_type(&self, kind: HlslTokenType) -> bool {
        let val = kind as u8;
        (val >= 42 && val <= 92) || kind == HlslTokenType::Identifier
    }
}
