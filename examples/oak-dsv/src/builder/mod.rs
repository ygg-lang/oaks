use crate::{
    DsvLanguage,
    ast::{DsvField, DsvRecord, DsvRoot},
    language::Dsv,
    lexer::DsvLexer,
    parser::DsvParser,
};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, SourceText, TextEdit, parser::session::ParseSession, source::Source};

/// AST builder for DSV.
#[derive(Clone, Default)]
pub struct DsvBuilder<const LANG: DsvLanguage>;

impl<const LANG: DsvLanguage> DsvBuilder<LANG> {
    /// Creates a new DSV builder.
    pub fn new() -> Self {
        Self
    }
}

impl<const LANG: DsvLanguage> Builder<Dsv<LANG>> for DsvBuilder<LANG> {
    /// Builds the DSV AST.
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<Dsv<LANG>>) -> OakDiagnostics<DsvRoot<LANG>> {
        let parser = DsvParser::<LANG>::new();
        let lexer = DsvLexer::<LANG>::new();

        let mut cache = ParseSession::<Dsv<LANG>>::default();
        lexer.lex(source, edits, &mut cache);
        let parse_result = parser.parse(source, edits, &mut cache);

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

impl<const LANG: DsvLanguage> DsvBuilder<LANG> {
    /// Builds the root of the DSV AST.
    fn build_root<'a>(&self, green_tree: &GreenNode<'a, Dsv<LANG>>, source: &SourceText) -> Result<DsvRoot<LANG>, OakError> {
        let mut records = Vec::new();
        let mut current_offset = 0;

        for child in green_tree.children {
            match child {
                oak_core::GreenTree::Node(n) => {
                    if n.kind == crate::parser::element_type::DsvElementType::Record {
                        records.push(self.build_record(n, current_offset, source)?);
                    }
                    current_offset += n.byte_length as usize;
                }
                oak_core::GreenTree::Leaf(l) => {
                    current_offset += l.length as usize;
                }
            }
        }

        Ok(DsvRoot::new(records))
    }

    /// Builds a DSV record.
    fn build_record<'a>(&self, node: &GreenNode<'a, Dsv<LANG>>, offset: usize, source: &SourceText) -> Result<DsvRecord<LANG>, OakError> {
        let span = (offset..offset + node.byte_length as usize).into();
        let mut fields = Vec::new();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::GreenTree::Node(n) => {
                    if n.kind == crate::parser::element_type::DsvElementType::Field {
                        fields.push(self.build_field(n, current_offset, source)?);
                    }
                    current_offset += n.byte_length as usize;
                }
                oak_core::GreenTree::Leaf(l) => {
                    current_offset += l.length as usize;
                }
            }
        }

        Ok(DsvRecord { fields, span })
    }

    /// Builds a DSV field.
    fn build_field<'a>(&self, node: &GreenNode<'a, Dsv<LANG>>, offset: usize, source: &SourceText) -> Result<DsvField<LANG>, OakError> {
        let span = (offset..offset + node.byte_length as usize).into();
        let raw_value = source.get_text_in(span).into_owned();

        let (value, is_quoted) = if raw_value.starts_with(LANG.quote_char) && raw_value.ends_with(LANG.quote_char) && raw_value.len() >= 2 {
            let unquoted = &raw_value[1..raw_value.len() - 1];
            (unquoted.replace(&format!("{}{}", LANG.quote_char, LANG.quote_char), &LANG.quote_char.to_string()), true)
        }
        else {
            (raw_value, false)
        };

        Ok(DsvField { value, is_quoted, span })
    }
}
