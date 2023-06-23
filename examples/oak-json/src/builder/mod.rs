use crate::{
    JsonLanguage,
    ast::{JsonRoot, JsonValue},
    lexer::JsonLexer,
    parser::JsonParser,
};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, SourceText, TextEdit, parser::session::ParseSession, source::Source};

/// JSON AST 构建器
#[derive(Clone)]
pub struct JsonBuilder<'config> {
    config: &'config JsonLanguage,
}

impl<'config> JsonBuilder<'config> {
    pub fn new(config: &'config JsonLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<JsonLanguage> for JsonBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<JsonLanguage>) -> OakDiagnostics<JsonRoot> {
        let parser = JsonParser::new(self.config);
        let lexer = JsonLexer::new(self.config);

        let mut cache = ParseSession::<JsonLanguage>::default();
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

impl<'config> JsonBuilder<'config> {
    fn build_root<'a>(&self, green_tree: &GreenNode<'a, JsonLanguage>, source: &SourceText) -> Result<JsonRoot, OakError> {
        let root_node = match green_tree.children.first() {
            Some(oak_core::GreenTree::Node(n)) => n,
            _ => return Err(OakError::unexpected_eof(0, None)),
        };

        let value = self.build_value(root_node, 0, source)?;
        Ok(JsonRoot { value })
    }

    fn build_value<'a>(&self, node: &GreenNode<'a, JsonLanguage>, offset: usize, source: &SourceText) -> Result<JsonValue, OakError> {
        use crate::parser::element_type::JsonElementType;
        let span: oak_core::Range<usize> = (offset..offset + node.text_len as usize).into();

        match node.kind {
            JsonElementType::Object => {
                let mut fields = Vec::new();
                let mut current_offset = offset;
                for child in node.children {
                    match child {
                        oak_core::GreenTree::Node(n) => {
                            if n.kind == JsonElementType::ObjectEntry {
                                fields.push(self.build_field(n, current_offset, source)?);
                            }
                            current_offset += n.text_len as usize;
                        }
                        oak_core::GreenTree::Leaf(l) => {
                            current_offset += l.length as usize;
                        }
                    }
                }
                Ok(JsonValue::Object(crate::ast::JsonObject { fields, span }))
            }
            JsonElementType::Array => {
                let mut elements = Vec::new();
                let mut current_offset = offset;
                for child in node.children {
                    match child {
                        oak_core::GreenTree::Node(n) => {
                            match n.kind {
                                JsonElementType::ArrayElement | JsonElementType::Value | JsonElementType::Object | JsonElementType::Array | JsonElementType::String | JsonElementType::Number | JsonElementType::Boolean | JsonElementType::Null => {
                                    elements.push(self.build_value(n, current_offset, source)?);
                                }
                                _ => {}
                            }
                            current_offset += n.text_len as usize;
                        }
                        oak_core::GreenTree::Leaf(l) => {
                            current_offset += l.length as usize;
                        }
                    }
                }
                Ok(JsonValue::Array(crate::ast::JsonArray { elements, span }))
            }
            JsonElementType::String => {
                let text = source.get_text_in(span.clone());
                let value = text.trim_matches('"').to_string();
                Ok(JsonValue::String(crate::ast::JsonString { value, span }))
            }
            JsonElementType::Number => {
                let text = source.get_text_in(span.clone());
                let value = text.parse::<f64>().map_err(|_| OakError::syntax_error(format!("Invalid number: {}", text), span.start, None))?;
                Ok(JsonValue::Number(crate::ast::JsonNumber { value, span }))
            }
            JsonElementType::Boolean => {
                let text = source.get_text_in(span.clone());
                let value = text == "true";
                Ok(JsonValue::Boolean(crate::ast::JsonBoolean { value, span }))
            }
            JsonElementType::Null => Ok(JsonValue::Null(crate::ast::JsonNull { span })),
            JsonElementType::Value | JsonElementType::ArrayElement | JsonElementType::Root => {
                let mut current_offset = offset;
                for child in node.children {
                    if let oak_core::GreenTree::Node(n) = child {
                        return self.build_value(n, current_offset, source);
                    }
                    else if let oak_core::GreenTree::Leaf(l) = child {
                        current_offset += l.length as usize;
                    }
                }
                Err(OakError::unexpected_eof(span.start, None))
            }
            _ => Err(OakError::syntax_error(format!("Unexpected node kind: {:?}", node.kind), span.start, None)),
        }
    }

    fn build_field<'a>(&self, node: &GreenNode<'a, JsonLanguage>, offset: usize, source: &SourceText) -> Result<crate::ast::JsonField, OakError> {
        use crate::{lexer::token_type::JsonTokenType, parser::element_type::JsonElementType};
        let span: oak_core::Range<usize> = (offset..offset + node.text_len as usize).into();

        let mut name = None;
        let mut value = None;
        let mut seen_colon = false;
        let mut current_offset = offset;

        for child in node.children {
            match child {
                oak_core::GreenTree::Node(n) => {
                    if !seen_colon {
                        if n.kind == JsonElementType::String {
                            let s_span: oak_core::Range<usize> = (current_offset..current_offset + n.text_len as usize).into();
                            let text = source.get_text_in(s_span.clone());
                            let val = text.trim_matches('"').to_string();
                            name = Some(crate::ast::JsonString { value: val, span: s_span });
                        }
                    }
                    else if value.is_none() {
                        match n.kind {
                            JsonElementType::Value | JsonElementType::Object | JsonElementType::Array | JsonElementType::String | JsonElementType::Number | JsonElementType::Boolean | JsonElementType::Null => {
                                value = Some(self.build_value(n, current_offset, source)?);
                            }
                            _ => {}
                        }
                    }
                    current_offset += n.text_len as usize;
                }
                oak_core::GreenTree::Leaf(l) => {
                    match l.kind {
                        JsonTokenType::Colon => {
                            seen_colon = true;
                        }
                        JsonTokenType::StringLiteral | JsonTokenType::BareKey if !seen_colon => {
                            let b_span: oak_core::Range<usize> = (current_offset..current_offset + l.length as usize).into();
                            let text = source.get_text_in(b_span.clone());
                            let val = if l.kind == JsonTokenType::StringLiteral { text.trim_matches('"').to_string() } else { text.to_string() };
                            name = Some(crate::ast::JsonString { value: val, span: b_span });
                        }
                        _ => {}
                    }
                    current_offset += l.length as usize;
                }
            }
        }

        let name = name.ok_or_else(|| OakError::expected_token("String", span.start, None))?;
        let value = value.ok_or_else(|| OakError::expected_token("Value", span.start, None))?;

        Ok(crate::ast::JsonField { name, value, span })
    }
}
