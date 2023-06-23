use crate::{ast::*, builder::TypeScriptBuilder, language::TypeScriptLanguage, parser::element_type::TypeScriptElementType};
use oak_core::{OakError, RedNode, RedTree, Source, SourceText};

impl<'config> TypeScriptBuilder<'config> {
    pub(crate) fn build_jsx_element(&self, node: &RedNode<TypeScriptLanguage>, source: &SourceText) -> Result<Option<JsxElement>, OakError> {
        let span = node.span();
        let mut opening_element = None;
        let mut children = Vec::new();
        let mut closing_element = None;

        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                match child_node.green.kind {
                    TypeScriptElementType::JsxOpeningElement => opening_element = self.build_jsx_opening_element(&child_node, source)?,
                    TypeScriptElementType::JsxClosingElement => closing_element = self.build_jsx_closing_element(&child_node, source)?,
                    _ => {
                        if let Some(c) = self.build_jsx_child(&child_node, source)? {
                            children.push(c)
                        }
                    }
                }
            }
        }

        if let (Some(opening), Some(closing)) = (opening_element, closing_element) { Ok(Some(JsxElement { opening_element: opening, children, closing_element: closing, span: span.into() })) } else { Ok(None) }
    }

    pub(crate) fn build_jsx_opening_element(&self, node: &RedNode<TypeScriptLanguage>, source: &SourceText) -> Result<Option<JsxOpeningElement>, OakError> {
        let span = node.span();
        let mut name = None;
        let mut attributes = Vec::new();

        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                match child_node.green.kind {
                    TypeScriptElementType::IdentifierName => name = Some(JsxTagName::Identifier(source.get_text_in(child_node.span().into()).to_string())),
                    TypeScriptElementType::JsxAttributes => {
                        for attr_child in child_node.children() {
                            if let RedTree::Node(attr_node) = attr_child {
                                if let Some(attr) = self.build_jsx_attribute_or_spread(&attr_node, source)? {
                                    attributes.push(attr)
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Some(n) = name { Ok(Some(JsxOpeningElement { name: n, attributes, span: span.into() })) } else { Ok(None) }
    }

    pub(crate) fn build_jsx_closing_element(&self, node: &RedNode<TypeScriptLanguage>, source: &SourceText) -> Result<Option<JsxClosingElement>, OakError> {
        let span = node.span();
        let mut name = None;

        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                if child_node.green.kind == TypeScriptElementType::IdentifierName {
                    name = Some(JsxTagName::Identifier(source.get_text_in(child_node.span().into()).to_string()))
                }
            }
        }

        if let Some(n) = name { Ok(Some(JsxClosingElement { name: n, span: span.into() })) } else { Ok(None) }
    }

    pub(crate) fn build_jsx_self_closing_element(&self, node: &RedNode<TypeScriptLanguage>, source: &SourceText) -> Result<Option<JsxSelfClosingElement>, OakError> {
        let span = node.span();
        let mut name = None;
        let mut attributes = Vec::new();

        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                match child_node.green.kind {
                    TypeScriptElementType::IdentifierName => name = Some(JsxTagName::Identifier(source.get_text_in(child_node.span().into()).to_string())),
                    TypeScriptElementType::JsxAttributes => {
                        for attr_child in child_node.children() {
                            if let RedTree::Node(attr_node) = attr_child {
                                if let Some(attr) = self.build_jsx_attribute_or_spread(&attr_node, source)? {
                                    attributes.push(attr)
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Some(n) = name { Ok(Some(JsxSelfClosingElement { name: n, attributes, span: span.into() })) } else { Ok(None) }
    }

    pub(crate) fn build_jsx_fragment(&self, node: &RedNode<TypeScriptLanguage>, source: &SourceText) -> Result<Option<JsxFragment>, OakError> {
        let span = node.span();
        let mut opening_fragment = None;
        let mut children = Vec::new();
        let mut closing_fragment = None;

        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                match child_node.green.kind {
                    TypeScriptElementType::JsxOpeningFragment => opening_fragment = Some(JsxOpeningFragment { span: child_node.span().into() }),
                    TypeScriptElementType::JsxClosingFragment => closing_fragment = Some(JsxClosingFragment { span: child_node.span().into() }),
                    _ => {
                        if let Some(c) = self.build_jsx_child(&child_node, source)? {
                            children.push(c)
                        }
                    }
                }
            }
        }

        if let (Some(opening), Some(closing)) = (opening_fragment, closing_fragment) { Ok(Some(JsxFragment { opening_fragment: opening, children, closing_fragment: closing, span: span.into() })) } else { Ok(None) }
    }

    pub(crate) fn build_jsx_child(&self, node: &RedNode<TypeScriptLanguage>, source: &SourceText) -> Result<Option<JsxChild>, OakError> {
        match node.green.kind {
            TypeScriptElementType::JsxElement => {
                if let Some(e) = self.build_jsx_element(node, source)? {
                    Ok(Some(JsxChild::JsxElement(Box::new(e))))
                }
                else {
                    Ok(None)
                }
            }
            TypeScriptElementType::JsxSelfClosingElement => {
                if let Some(e) = self.build_jsx_self_closing_element(node, source)? {
                    Ok(Some(JsxChild::JsxSelfClosingElement(Box::new(e))))
                }
                else {
                    Ok(None)
                }
            }
            TypeScriptElementType::JsxFragment => {
                if let Some(f) = self.build_jsx_fragment(node, source)? {
                    Ok(Some(JsxChild::JsxFragment(Box::new(f))))
                }
                else {
                    Ok(None)
                }
            }
            TypeScriptElementType::JsxText => Ok(Some(JsxChild::JsxText(source.get_text_in(node.span().into()).to_string()))),
            TypeScriptElementType::JsxExpressionContainer => {
                for child in node.children() {
                    if let RedTree::Node(child_node) = child {
                        if let Some(expr) = self.build_expression(&child_node, source)? {
                            return Ok(Some(JsxChild::JsxExpressionContainer(Some(expr))));
                        }
                    }
                }
                Ok(Some(JsxChild::JsxExpressionContainer(None)))
            }
            _ => Ok(None),
        }
    }

    pub(crate) fn build_jsx_attribute_or_spread(&self, node: &RedNode<TypeScriptLanguage>, source: &SourceText) -> Result<Option<JsxAttributeOrSpread>, OakError> {
        match node.green.kind {
            TypeScriptElementType::JsxAttribute => {
                let span = node.span();
                let mut name = String::new();
                let mut value = None;

                for child in node.children() {
                    if let RedTree::Node(child_node) = child {
                        match child_node.green.kind {
                            TypeScriptElementType::IdentifierName => name = source.get_text_in(child_node.span().into()).to_string(),
                            TypeScriptElementType::StringLiteral => value = Some(JsxAttributeValue::StringLiteral(source.get_text_in(child_node.span().into()).to_string())),
                            TypeScriptElementType::JsxExpressionContainer => {
                                for sub_child in child_node.children() {
                                    if let RedTree::Node(sub_node) = sub_child {
                                        if let Some(expr) = self.build_expression(&sub_node, source)? {
                                            value = Some(JsxAttributeValue::ExpressionContainer(Some(expr)));
                                            break;
                                        }
                                    }
                                }
                                if value.is_none() {
                                    value = Some(JsxAttributeValue::ExpressionContainer(None))
                                }
                            }
                            TypeScriptElementType::JsxElement => {
                                if let Some(e) = self.build_jsx_element(&child_node, source)? {
                                    value = Some(JsxAttributeValue::Element(Box::new(e)))
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Ok(Some(JsxAttributeOrSpread::Attribute(JsxAttribute { name, value, span: span.into() })))
            }
            TypeScriptElementType::JsxSpreadAttribute => {
                for child in node.children() {
                    if let RedTree::Node(child_node) = child {
                        if let Some(expr) = self.build_expression(&child_node, source)? {
                            return Ok(Some(JsxAttributeOrSpread::Spread(expr)));
                        }
                    }
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}
