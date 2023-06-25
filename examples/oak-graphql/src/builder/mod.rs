use crate::{
    ast,
    ast::*,
    language::GraphQLLanguage,
    lexer::token_type::GraphQLTokenType,
    parser::{GraphQLParser, element_type::GraphQLElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, Range, RedNode, RedTree, SourceText, TextEdit, TokenType, builder::BuildOutput, source::Source};

/// AST builder for GraphQL.
#[derive(Clone)]
pub struct GraphQLBuilder<'config> {
    config: &'config GraphQLLanguage,
}

impl<'config> GraphQLBuilder<'config> {
    /// Creates a new GraphQL builder.
    pub fn new(config: &'config GraphQLLanguage) -> Self {
        Self { config }
    }

    fn get_text(&self, span: Range<usize>, source: &SourceText) -> String {
        source.text()[span].to_string()
    }
}

impl<'config> Builder<GraphQLLanguage> for GraphQLBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<GraphQLLanguage>) -> BuildOutput<GraphQLLanguage> {
        let parser = GraphQLParser::new(self.config);
        let mut parse_cache = oak_core::parser::session::ParseSession::<GraphQLLanguage>::default();
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

impl<'config> GraphQLBuilder<'config> {
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, GraphQLLanguage>, source: &SourceText) -> Result<GraphQLRoot, OakError> {
        let root_node = RedNode::new(green_tree, 0);
        let mut definitions = Vec::new();

        for child in root_node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    GraphQLElementType::QueryKeyword | GraphQLElementType::MutationKeyword | GraphQLElementType::SubscriptionKeyword => {
                        definitions.push(Definition::Operation(self.build_operation_definition(n, source)?));
                    }
                    GraphQLElementType::FragmentKeyword => {
                        definitions.push(Definition::Fragment(self.build_fragment_definition(n, source)?));
                    }
                    GraphQLElementType::SchemaKeyword => {
                        definitions.push(Definition::Schema(self.build_schema_definition(n, source)?));
                    }
                    GraphQLElementType::TypeKeyword => {
                        definitions.push(Definition::Type(self.build_type_definition(n, source)?));
                    }
                    _ => {
                        // Handle other definition types as needed
                    }
                }
            }
        }

        Ok(GraphQLRoot { document: Document { definitions, span: root_node.span() } })
    }

    fn build_operation_definition<'a>(&self, node: RedNode<'a, GraphQLLanguage>, source: &SourceText) -> Result<OperationDefinition, OakError> {
        let mut operation_type = OperationType::Query;
        let mut name = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    GraphQLTokenType::QueryKeyword => operation_type = OperationType::Query,
                    GraphQLTokenType::MutationKeyword => operation_type = OperationType::Mutation,
                    GraphQLTokenType::SubscriptionKeyword => operation_type = OperationType::Subscription,
                    GraphQLTokenType::Name => name = Some(self.get_text(t.span.clone(), source)),
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(OperationDefinition { operation_type, name, span: node.span() })
    }

    fn build_fragment_definition<'a>(&self, node: RedNode<'a, GraphQLLanguage>, source: &SourceText) -> Result<FragmentDefinition, OakError> {
        let mut name = String::new();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == GraphQLTokenType::Name {
                    name = self.get_text(t.span.clone(), source);
                }
            }
        }

        Ok(FragmentDefinition { name, span: node.span() })
    }

    fn build_schema_definition<'a>(&self, node: RedNode<'a, GraphQLLanguage>, _source: &SourceText) -> Result<SchemaDefinition, OakError> {
        Ok(SchemaDefinition { span: node.span() })
    }

    fn build_type_definition<'a>(&self, node: RedNode<'a, GraphQLLanguage>, source: &SourceText) -> Result<TypeDefinition, OakError> {
        let mut name = String::new();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == GraphQLTokenType::Name {
                    name = self.get_text(t.span.clone(), source);
                }
            }
        }

        Ok(TypeDefinition { name, span: node.span() })
    }
}
