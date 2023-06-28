/// Fluent translator module.
use crate::ast::*;
use std::collections::{HashMap, HashSet};

/// Fluent translator.
#[derive(Debug, Clone)]
pub struct Translator {
    /// Fluent root AST.
    root: FluentRoot,
    /// Message map for fast lookup (id -> index).
    message_map: HashMap<String, usize>,
    /// Translation cache.
    translation_cache: std::sync::Arc<std::sync::Mutex<HashMap<(String, String, Option<Vec<(String, String)>>), String>>>,
}

impl Translator {
    /// Creates a new Fluent translator.
    pub fn new(root: FluentRoot) -> Self {
        // Build message map for fast lookup
        let mut message_map = HashMap::new();
        for (index, message) in root.messages.iter().enumerate() {
            message_map.insert(message.id.clone(), index);
        }

        Self { root, message_map, translation_cache: std::sync::Arc::new(std::sync::Mutex::new(HashMap::new())) }
    }

    /// Generates a cache key for translation.
    fn generate_cache_key(&self, id: &str, args: &std::collections::HashMap<String, String>) -> (String, String, Option<Vec<(String, String)>>) {
        if args.is_empty() {
            (id.to_string(), "".to_string(), None)
        }
        else {
            let mut args_vec: Vec<(String, String)> = args.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            args_vec.sort_by(|(a, _), (b, _)| a.cmp(b));
            (id.to_string(), "".to_string(), Some(args_vec))
        }
    }

    /// Translates a message by id.
    pub fn translate(&self, id: &str, args: &std::collections::HashMap<String, String>) -> Option<String> {
        // Generate cache key
        let cache_key = self.generate_cache_key(id, args);

        // Check cache
        if let Ok(mut cache) = self.translation_cache.lock() {
            if let Some(cached) = cache.get(&cache_key) {
                return Some(cached.clone());
            }
        }

        // Fast message lookup using message_map
        if let Some(&index) = self.message_map.get(id) {
            if let Some(message) = self.root.messages.get(index) {
                if let Some(result) = self.translate_message(message, args) {
                    // Cache the result
                    if let Ok(mut cache) = self.translation_cache.lock() {
                        cache.insert(cache_key, result.clone());
                    }
                    return Some(result);
                }
            }
        }

        None
    }

    /// Translates a message with attribute.
    pub fn translate_with_attribute(&self, id: &str, attribute: &str, args: &std::collections::HashMap<String, String>) -> Option<String> {
        // Fast message lookup using message_map
        if let Some(&index) = self.message_map.get(id) {
            if let Some(message) = self.root.messages.get(index) {
                if let Some(attr) = message.attributes.iter().find(|a| a.id == attribute) {
                    return self.translate_pattern(&attr.value, args);
                }
            }
        }

        None
    }

    /// Translates a message.
    fn translate_message(&self, message: &FluentMessage, args: &std::collections::HashMap<String, String>) -> Option<String> {
        message.value.as_ref().and_then(|pattern| self.translate_pattern(pattern, args))
    }

    /// Translates a pattern.
    fn translate_pattern(&self, pattern: &FluentPattern, args: &std::collections::HashMap<String, String>) -> Option<String> {
        let mut result = String::new();

        for element in &pattern.elements {
            if let Some(text) = self.translate_pattern_element(element, args) {
                result.push_str(&text);
            }
        }

        if result.is_empty() { None } else { Some(result) }
    }

    /// Translates a pattern element.
    fn translate_pattern_element(&self, element: &FluentPatternElement, args: &std::collections::HashMap<String, String>) -> Option<String> {
        match element {
            FluentPatternElement::Text(text) => Some(text.clone()),
            FluentPatternElement::VariableReference(name) => args.get(name).cloned(),
            FluentPatternElement::MessageReference(id, attribute) => {
                if let Some(attribute) = attribute {
                    self.translate_with_attribute(id, attribute, args)
                }
                else {
                    self.translate(id, args)
                }
            }
            FluentPatternElement::SelectExpression(expr, variants) => {
                let value = self.evaluate_expression(expr, args)?;
                self.select_variant(variants, &value, args)
            }
        }
    }

    /// Evaluates an expression.
    fn evaluate_expression(&self, expr: &FluentExpression, args: &std::collections::HashMap<String, String>) -> Option<String> {
        match expr {
            FluentExpression::VariableReference(name) => args.get(name).cloned(),
            FluentExpression::MessageReference(id, attribute) => {
                if let Some(attribute) = attribute {
                    self.translate_with_attribute(id, attribute, args)
                }
                else {
                    self.translate(id, args)
                }
            }
            FluentExpression::NumberLiteral(number) => Some(number.to_string()),
        }
    }

    /// Selects a variant based on the value.
    fn select_variant(&self, variants: &[FluentVariant], value: &str, args: &std::collections::HashMap<String, String>) -> Option<String> {
        // Try to find an exact match
        for variant in variants {
            if let Some(match_value) = self.variant_key_to_string(&variant.key) {
                if match_value == value {
                    return self.translate_pattern(&variant.value, args);
                }
            }
        }

        // Try to find a default variant (usually the first one)
        variants.first().and_then(|variant| self.translate_pattern(&variant.value, args))
    }

    /// Converts a variant key to string.
    fn variant_key_to_string(&self, key: &FluentVariantKey) -> Option<String> {
        match key {
            FluentVariantKey::Number(n) => Some(n.to_string()),
            FluentVariantKey::String(s) => Some(s.clone()),
        }
    }
}
