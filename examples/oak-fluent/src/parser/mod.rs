/// Fluent parser module.
pub mod element_type;

pub use element_type::FluentElementType;

use oak_core::{
    ParseCache, ParseOutput, Parser,
    source::{Source, TextEdit},
};

use crate::{ast::FluentPattern, language::FluentLanguage};

/// Fluent parser.
#[derive(Debug, Clone, Default)]
pub struct FluentParser;

impl Parser<FluentLanguage> for FluentParser {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<FluentLanguage>) -> ParseOutput<'a, FluentLanguage> {
        // Implementation will be added here
        // For now, return an empty result
        let diagnostics = oak_core::errors::OakDiagnostics { result: Err(oak_core::errors::OakError::custom_error("Not implemented")), diagnostics: vec![] };
        diagnostics
    }
}

/// Parses a Fluent string into a Fluent AST.
pub fn parse(input: &str) -> Result<crate::ast::FluentRoot, oak_core::errors::OakError> {
    use crate::ast::*;

    let mut messages = Vec::with_capacity(10); // Pre-allocate capacity for common case
    let mut current_id = String::new();
    let mut current_value = None;
    let mut in_message = false;
    let mut in_value = false;
    let mut value_content = String::new();

    for line in input.lines() {
        let original_line = line;
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Check if this is a new message definition (starts with a non-whitespace character and contains '=')
        let is_new_message = !original_line.starts_with(' ') && !original_line.starts_with('\t') && line.contains('=');

        if is_new_message && in_message {
            // Save the current message
            if !current_id.is_empty() {
                if !value_content.is_empty() {
                    let pattern = parse_pattern(&value_content);
                    current_value = Some(pattern);
                }

                messages.push(FluentMessage { id: current_id.clone(), value: current_value.clone(), attributes: Vec::new(), variants: Vec::new() });
            }

            // Start a new message
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() >= 2 {
                current_id = parts[0].trim().to_string();
                let joined_value = parts[1..].join("=");
                let value_part = joined_value.trim();
                if !value_part.is_empty() {
                    value_content = value_part.to_string();
                    in_value = true;
                }
                else {
                    value_content.clear();
                    in_value = false;
                }
                current_value = None;
            }
        }
        else if !in_message && is_new_message {
            // Start the first message
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() >= 2 {
                current_id = parts[0].trim().to_string();
                let joined_value = parts[1..].join("=");
                let value_part = joined_value.trim();
                if !value_part.is_empty() {
                    value_content = value_part.to_string();
                    in_value = true;
                }
                in_message = true;
            }
        }
        else if in_message && in_value {
            // Continue reading value if it spans multiple lines (indented)
            value_content.push_str("\n");
            value_content.push_str(line);
        }
    }

    // Save the last message
    if !current_id.is_empty() {
        if !value_content.is_empty() {
            let pattern = parse_pattern(&value_content);
            current_value = Some(pattern);
        }

        messages.push(FluentMessage { id: current_id.clone(), value: current_value.clone(), attributes: Vec::new(), variants: Vec::new() });
    }

    Ok(FluentRoot { messages })
}

/// Parses a Fluent pattern string into a FluentPattern.
fn parse_pattern(input: &str) -> FluentPattern {
    use crate::ast::*;

    let mut elements = Vec::with_capacity(5); // Pre-allocate capacity for common case
    let mut current_text = String::with_capacity(100); // Pre-allocate capacity for common case
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' {
            // Start of a reference or expression
            if !current_text.is_empty() {
                elements.push(FluentPatternElement::Text(current_text));
                current_text = String::with_capacity(100); // Reuse capacity
            }

            // Parse the content inside the braces
            let mut ref_content = String::with_capacity(50); // Pre-allocate capacity
            let mut depth = 1;

            while let Some(c) = chars.next() {
                if c == '{' {
                    depth += 1;
                    ref_content.push(c);
                }
                else if c == '}' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    else {
                        ref_content.push(c);
                    }
                }
                else {
                    ref_content.push(c);
                }
            }

            // Process the reference content
            let trimmed = ref_content.trim();
            if !trimmed.is_empty() {
                if trimmed.contains("->") {
                    // Select expression (plural or gender)
                    let parts: Vec<&str> = trimmed.split("->").collect();
                    if parts.len() == 2 {
                        let selector = parts[0].trim();
                        let variants_str = parts[1].trim();

                        // Parse selector
                        let expression = if selector.starts_with('$') {
                            // Variable reference selector
                            let var_name = selector[1..].trim().to_string();
                            FluentExpression::VariableReference(var_name)
                        }
                        else if selector.parse::<i64>().is_ok() {
                            // Number literal selector
                            let num = selector.parse::<i64>().unwrap();
                            FluentExpression::NumberLiteral(num)
                        }
                        else if selector.contains('.') {
                            // Message reference with attribute selector
                            let parts: Vec<&str> = selector.split('.').collect();
                            if parts.len() == 2 {
                                let msg_id = parts[0].trim().to_string();
                                let attr = parts[1].trim().to_string();
                                FluentExpression::MessageReference(msg_id, Some(attr))
                            }
                            else {
                                // Invalid selector, treat as text
                                elements.push(FluentPatternElement::Text(format!("{{{}}}", ref_content)));
                                continue;
                            }
                        }
                        else {
                            // Message reference selector
                            let msg_id = selector.to_string();
                            FluentExpression::MessageReference(msg_id, None)
                        };

                        // Parse variants
                        let mut variants = Vec::with_capacity(3); // Pre-allocate capacity for common case
                        let variant_lines = variants_str.lines();

                        for line in variant_lines {
                            let line = line.trim();
                            if line.is_empty() {
                                continue;
                            }

                            // Parse variant key and value
                            if line.starts_with("[") || line.starts_with("*[") {
                                let mut key_str = String::with_capacity(20); // Pre-allocate capacity
                                let mut key_depth = 1;
                                let mut key_chars = if line.starts_with("*[") { line.chars().skip(2) } else { line.chars().skip(1) };

                                while let Some(c) = key_chars.next() {
                                    if c == '[' {
                                        key_depth += 1;
                                        key_str.push(c);
                                    }
                                    else if c == ']' {
                                        key_depth -= 1;
                                        if key_depth == 0 {
                                            break;
                                        }
                                        else {
                                            key_str.push(c);
                                        }
                                    }
                                    else {
                                        key_str.push(c);
                                    }
                                }

                                let key = if key_str.starts_with('*') {
                                    // Default variant
                                    let actual_key = key_str[1..].trim();
                                    if actual_key.parse::<i64>().is_ok() { FluentVariantKey::Number(actual_key.parse::<i64>().unwrap()) } else { FluentVariantKey::String(actual_key.to_string()) }
                                }
                                else {
                                    // Regular variant
                                    if key_str.parse::<i64>().is_ok() { FluentVariantKey::Number(key_str.parse::<i64>().unwrap()) } else { FluentVariantKey::String(key_str.to_string()) }
                                };

                                // Parse variant value
                                let parts: Vec<&str> = line.split(']').skip(1).collect();
                                let joined = parts.join("]");
                                let value_str = joined.trim();
                                let value_pattern = parse_pattern(value_str);

                                variants.push(FluentVariant { key, value: value_pattern });
                            }
                        }

                        // If no variants were parsed, add a default one
                        if variants.is_empty() {
                            let default_pattern = parse_pattern("");
                            variants.push(FluentVariant { key: FluentVariantKey::String("other".to_string()), value: default_pattern });
                        }

                        elements.push(FluentPatternElement::SelectExpression(Box::new(expression), variants));
                    }
                    else {
                        // Invalid select expression, treat as text
                        elements.push(FluentPatternElement::Text(format!("{{{}}}", ref_content)));
                    }
                }
                else if trimmed.starts_with('$') {
                    // Variable reference: { $name }
                    let var_name = trimmed[1..].trim().to_string();
                    elements.push(FluentPatternElement::VariableReference(var_name));
                }
                else if trimmed.contains('.') {
                    // Message reference with attribute: { message.attr }
                    let parts: Vec<&str> = trimmed.split('.').collect();
                    if parts.len() == 2 {
                        let msg_id = parts[0].trim().to_string();
                        let attr = parts[1].trim().to_string();
                        elements.push(FluentPatternElement::MessageReference(msg_id, Some(attr)));
                    }
                    else {
                        // Invalid reference, treat as text
                        elements.push(FluentPatternElement::Text(format!("{{{}}}", ref_content)));
                    }
                }
                else {
                    // Message reference: { message }
                    let msg_id = trimmed.to_string();
                    elements.push(FluentPatternElement::MessageReference(msg_id, None));
                }
            }
            else {
                // Empty reference, treat as text
                elements.push(FluentPatternElement::Text("{}".to_string()));
            }
        }
        else {
            // Regular text
            current_text.push(c);
        }
    }

    // Add any remaining text
    if !current_text.is_empty() {
        elements.push(FluentPatternElement::Text(current_text));
    }

    FluentPattern { elements }
}

/// Parses a Fluent string into a Fluent AST with configuration.
pub fn parse_with_config(input: &str, _config: ()) -> Result<crate::ast::FluentRoot, oak_core::errors::OakError> {
    // Delegate to parse function for now
    parse(input)
}
