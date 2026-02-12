use crate::{
    ast::{Expression, Literal},
    language::PythonLanguage,
    parser::{PythonElementType, PythonParser},
};
use oak_core::{GreenTree, OakError, Parser, SourceText};

impl<'config> super::PythonBuilder<'config> {
    /// Parses an f-string's text content into a list of expressions.
    pub(crate) fn parse_fstring_text(&self, text: &str) -> Result<Vec<Expression>, OakError> {
        let mut values = Vec::new();
        let mut i = 0;
        let bytes = text.as_bytes();

        // Skip prefix (f, rf, fr)
        while i < bytes.len() && (bytes[i] == b'f' || bytes[i] == b'F' || bytes[i] == b'r' || bytes[i] == b'R') {
            i += 1;
        }

        // Skip opening quotes
        if i >= bytes.len() {
            return Ok(values);
        }
        let quote_char = bytes[i];
        let mut quote_count = 0;
        while i < bytes.len() && bytes[i] == quote_char && quote_count < 3 {
            i += 1;
            quote_count += 1;
        }

        let end_index = if bytes.len() >= quote_count { bytes.len() - quote_count } else { bytes.len() };
        let mut start = i;

        while i < end_index {
            if bytes[i] == b'{' {
                // Potential expression
                if i + 1 < end_index && bytes[i + 1] == b'{' {
                    // Escaped {{
                    i += 2;
                    continue;
                }

                // Add literal part before {
                if i > start {
                    let s = &text[start..i];
                    values.push(Expression::Literal(Literal::String(s.to_string())));
                }

                // Find matching }
                i += 1; // skip {
                let expr_start = i;
                let mut brace_level = 1;
                while i < end_index && brace_level > 0 {
                    if bytes[i] == b'{' {
                        brace_level += 1
                    }
                    else if bytes[i] == b'}' {
                        brace_level -= 1
                    }
                    i += 1
                }

                if brace_level == 0 {
                    let expr_text = &text[expr_start..i - 1];
                    let expr = self.parse_single_expression(expr_text)?;
                    values.push(Expression::FormattedValue { value: Box::new(expr), conversion: 0, format_spec: None });
                    start = i
                }
                else {
                    // Unmatched {
                    start = i
                }
            }
            else if bytes[i] == b'}' {
                if i + 1 < end_index && bytes[i + 1] == b'}' {
                    // Escaped }}
                    i += 2;
                    continue;
                }
                i += 1
            }
            else {
                i += 1
            }
        }

        if start < end_index {
            let s = &text[start..end_index];
            values.push(Expression::Literal(Literal::String(s.to_string())))
        }

        Ok(values)
    }

    /// Parses a single expression from text.
    pub(crate) fn parse_single_expression(&self, text: &str) -> Result<Expression, OakError> {
        let parser = PythonParser::new(self.config);
        let mut cache = oak_core::parser::session::ParseSession::<PythonLanguage>::default();
        let source = SourceText::new(text.to_string());
        let output = parser.parse(&source, &[], &mut cache);

        match output.result {
            Ok(green) => {
                for child in green.children() {
                    if let GreenTree::Node(node) = child {
                        if node.kind == PythonElementType::Expr {
                            for subchild in node.children() {
                                if let GreenTree::Node(expr_node) = subchild {
                                    if !expr_node.kind.is_trivia() {
                                        return self.build_expression(expr_node, 0, &source);
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(Expression::Name(text.to_string()))
            }
            Err(e) => Err(e),
        }
    }
}
