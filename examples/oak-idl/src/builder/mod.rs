use crate::{IdlLanguage, IdlParser, ast::*, lexer::token_type::IdlTokenType, parser::element_type::IdlElementType};
use oak_core::{Builder, BuilderCache, GreenNode, GreenTree, OakDiagnostics, OakError, Parser, SourceText, TextEdit, source::Source};

/// A builder for IDL language structures.
pub struct IdlBuilder<'config> {
    config: &'config IdlLanguage,
}

impl<'config> IdlBuilder<'config> {
    /// Creates a new instance of the IDL builder.
    pub fn new(config: &'config IdlLanguage) -> Self {
        Self { config }
    }

    /// Builds the IDL AST root from a green tree.
    pub fn build_root(&self, green_tree: GreenNode<IdlLanguage>, source: &SourceText) -> Result<IdlRoot, OakError> {
        let mut items = Vec::new();
        let mut current_offset = 0;

        // Recursively search for items in the tree
        fn find_items<'a>(node: &GreenNode<IdlLanguage>, offset: &mut usize, source: &SourceText, builder: &IdlBuilder, items: &mut Vec<IdlItem>) -> Result<(), OakError> {
            // 对于 SourceFile 节点，直接遍历其子节点
            if node.kind == crate::parser::element_type::IdlElementType::SourceFile {
                for child in node.children {
                    match child {
                        GreenTree::Node(n) => {
                            find_items(n, offset, source, builder, items)?;
                            *offset += n.byte_length as usize;
                        }
                        GreenTree::Leaf(l) => {
                            *offset += l.length as usize;
                        }
                    }
                }
                return Ok(());
            }

            // 对于其他节点，尝试构建 item
            if let Some(item) = builder.build_item(node, *offset, source)? {
                items.push(item);
            }

            // 递归处理子节点
            for child in node.children {
                match child {
                    GreenTree::Node(n) => {
                        find_items(n, offset, source, builder, items)?;
                        *offset += n.byte_length as usize;
                    }
                    GreenTree::Leaf(l) => {
                        *offset += l.length as usize;
                    }
                }
            }
            Ok(())
        }

        find_items(&green_tree, &mut current_offset, source, self, &mut items)?;

        Ok(IdlRoot { items })
    }

    fn build_item(&self, node: &GreenNode<IdlLanguage>, offset: usize, source: &SourceText) -> Result<Option<IdlItem>, OakError> {
        let span = core::range::Range { start: offset, end: offset + node.byte_length as usize };

        match node.kind {
            IdlElementType::Module => {
                // Parse module name and items
                let mut module_name = String::new();
                let mut module_items = Vec::new();
                let mut current_offset = offset;

                for child in node.children {
                    match child {
                        GreenTree::Node(n) => {
                            if module_name.is_empty() && n.kind == IdlElementType::Identifier {
                                module_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                            }
                            else if n.kind == IdlElementType::Module {
                                if let Some(item) = self.build_item(n, current_offset, source)? {
                                    module_items.push(item);
                                }
                            }
                            current_offset += n.byte_length as usize;
                        }
                        GreenTree::Leaf(l) => {
                            if module_name.is_empty() && l.kind == IdlTokenType::Identifier {
                                module_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                            }
                            current_offset += l.length as usize;
                        }
                    }
                }

                Ok(Some(IdlItem::Module(Module { name: module_name, items: module_items, span: span.into() })))
            }
            IdlElementType::Interface => {
                // Parse interface name and members
                let mut interface_name = String::new();
                let mut interface_members = Vec::new();
                let mut current_offset = offset;

                for child in node.children {
                    match child {
                        GreenTree::Node(n) => {
                            if interface_name.is_empty() && n.kind == IdlElementType::Identifier {
                                interface_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                            }
                            else if n.kind == IdlElementType::Attribute || n.kind == IdlElementType::Operation {
                                if let Some(member) = self.build_member(n, current_offset, source)? {
                                    interface_members.push(member);
                                }
                            }
                            current_offset += n.byte_length as usize;
                        }
                        GreenTree::Leaf(l) => {
                            if interface_name.is_empty() && l.kind == IdlTokenType::Identifier {
                                interface_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                            }
                            current_offset += l.length as usize;
                        }
                    }
                }

                Ok(Some(IdlItem::Interface(Interface { name: interface_name, members: interface_members, span: span.into() })))
            }
            IdlElementType::Struct => {
                // Parse struct name and fields
                let mut struct_name = String::new();
                let mut struct_fields = Vec::new();
                let mut current_offset = offset;

                for child in node.children {
                    match child {
                        GreenTree::Node(n) => {
                            if struct_name.is_empty() && n.kind == IdlElementType::Identifier {
                                struct_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                            }
                            else if n.kind == IdlElementType::Field {
                                if let Some(field) = self.build_field(n, current_offset, source)? {
                                    struct_fields.push(field);
                                }
                            }
                            current_offset += n.byte_length as usize;
                        }
                        GreenTree::Leaf(l) => {
                            if struct_name.is_empty() && l.kind == IdlTokenType::Identifier {
                                struct_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                            }
                            current_offset += l.length as usize;
                        }
                    }
                }

                Ok(Some(IdlItem::Struct(Struct { name: struct_name, fields: struct_fields, span: span.into() })))
            }
            IdlElementType::Enum => {
                // Parse enum name and variants
                let mut enum_name = String::new();
                let mut enum_variants = Vec::new();
                let mut current_offset = offset;

                for child in node.children {
                    match child {
                        GreenTree::Node(n) => {
                            if enum_name.is_empty() && n.kind == IdlElementType::Identifier {
                                enum_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                            }
                            else if n.kind == IdlElementType::StringLiteral {
                                let variant = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                                let variant = variant.trim_matches('"').to_string();
                                enum_variants.push(variant);
                            }
                            current_offset += n.byte_length as usize;
                        }
                        GreenTree::Leaf(l) => {
                            if enum_name.is_empty() && l.kind == IdlTokenType::Identifier {
                                enum_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                            }
                            else if l.kind == IdlTokenType::StringLiteral {
                                let variant = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                                let variant = variant.trim_matches('"').to_string();
                                enum_variants.push(variant);
                            }
                            current_offset += l.length as usize;
                        }
                    }
                }

                Ok(Some(IdlItem::Enum(Enum { name: enum_name, variants: enum_variants })))
            }
            IdlElementType::Typedef => {
                // Parse typedef
                let mut type_name = String::new();
                let mut new_name = String::new();
                let mut current_offset = offset;

                for child in node.children {
                    match child {
                        GreenTree::Node(n) => {
                            if type_name.is_empty() && (n.kind == IdlElementType::Identifier || n.kind.is_basic_type()) {
                                type_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                            }
                            else if !type_name.is_empty() && new_name.is_empty() && n.kind == IdlElementType::Identifier {
                                new_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                            }
                            current_offset += n.byte_length as usize;
                        }
                        GreenTree::Leaf(l) => {
                            if type_name.is_empty() && l.kind == IdlTokenType::Identifier {
                                type_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                            }
                            else if !type_name.is_empty() && new_name.is_empty() && l.kind == IdlTokenType::Identifier {
                                new_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                            }
                            current_offset += l.length as usize;
                        }
                    }
                }

                Ok(Some(IdlItem::Typedef(Typedef { name: new_name, type_name })))
            }
            IdlElementType::Const => {
                // Parse const
                let mut type_name = String::new();
                let mut const_name = String::new();
                let mut const_value = String::new();
                let mut current_offset = offset;

                for child in node.children {
                    match child {
                        GreenTree::Node(n) => {
                            if type_name.is_empty() && (n.kind == IdlElementType::Identifier || n.kind.is_basic_type()) {
                                type_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                            }
                            else if !type_name.is_empty() && const_name.is_empty() && n.kind == IdlElementType::Identifier {
                                const_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                            }
                            else if !const_name.is_empty() && const_value.is_empty() && (n.kind == IdlElementType::StringLiteral || n.kind == IdlElementType::NumberLiteral || n.kind == IdlElementType::BooleanLiteral) {
                                const_value = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                            }
                            current_offset += n.byte_length as usize;
                        }
                        GreenTree::Leaf(l) => {
                            if type_name.is_empty() && l.kind == IdlTokenType::Identifier {
                                type_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                            }
                            else if !type_name.is_empty() && const_name.is_empty() && l.kind == IdlTokenType::Identifier {
                                const_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                            }
                            else if !const_name.is_empty() && const_value.is_empty() && (l.kind == IdlTokenType::StringLiteral || l.kind == IdlTokenType::NumberLiteral || l.kind == IdlTokenType::BooleanLiteral) {
                                const_value = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                            }
                            current_offset += l.length as usize;
                        }
                    }
                }

                Ok(Some(IdlItem::Const(Const { name: const_name, type_name, value: const_value })))
            }
            _ => {
                // For other types, try to find a child node that is an item
                for child in node.children {
                    if let GreenTree::Node(n) = child {
                        if let Some(item) = self.build_item(n, offset, source)? {
                            return Ok(Some(item));
                        }
                    }
                }
                Ok(None)
            }
        }
    }

    /// Build a member from a green node
    fn build_member(&self, node: &GreenNode<IdlLanguage>, offset: usize, source: &SourceText) -> Result<Option<IdlMember>, OakError> {
        match node.kind {
            IdlElementType::Attribute => {
                let mut name = String::new();
                let mut type_name = String::new();
                let mut readonly = false;
                let mut current_offset = offset;

                for child in node.children {
                    match child {
                        GreenTree::Node(n) => {
                            if n.kind == IdlElementType::Readonly {
                                readonly = true;
                            }
                            else if type_name.is_empty() && (n.kind == IdlElementType::Identifier || n.kind.is_basic_type()) {
                                type_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                            }
                            else if !type_name.is_empty() && name.is_empty() && n.kind == IdlElementType::Identifier {
                                name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                            }
                            current_offset += n.byte_length as usize;
                        }
                        GreenTree::Leaf(l) => {
                            if l.kind == IdlTokenType::Readonly {
                                readonly = true;
                            }
                            else if type_name.is_empty() && l.kind == IdlTokenType::Identifier {
                                type_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                            }
                            else if !type_name.is_empty() && name.is_empty() && l.kind == IdlTokenType::Identifier {
                                name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                            }
                            current_offset += l.length as usize;
                        }
                    }
                }

                Ok(Some(IdlMember::Attribute(Attribute { name, type_name, readonly })))
            }
            IdlElementType::Operation => {
                let mut name = String::new();
                let mut return_type = String::new();
                let mut params = Vec::new();
                let mut current_offset = offset;

                for child in node.children {
                    match child {
                        GreenTree::Node(n) => {
                            if return_type.is_empty() && (n.kind == IdlElementType::Identifier || n.kind.is_basic_type() || n.kind == IdlElementType::Void) {
                                return_type = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                            }
                            else if !return_type.is_empty() && name.is_empty() && n.kind == IdlElementType::Identifier {
                                name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                            }
                            else if n.kind == IdlElementType::Param {
                                if let Some(param) = self.build_param(n, current_offset, source)? {
                                    params.push(param);
                                }
                            }
                            current_offset += n.byte_length as usize;
                        }
                        GreenTree::Leaf(l) => {
                            if return_type.is_empty() && l.kind == IdlTokenType::Identifier {
                                return_type = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                            }
                            else if !return_type.is_empty() && name.is_empty() && l.kind == IdlTokenType::Identifier {
                                name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                            }
                            current_offset += l.length as usize;
                        }
                    }
                }

                Ok(Some(IdlMember::Operation(Operation { name, return_type, params })))
            }
            _ => Ok(None),
        }
    }

    /// Build a field from a green node
    fn build_field(&self, node: &GreenNode<IdlLanguage>, offset: usize, source: &SourceText) -> Result<Option<Field>, OakError> {
        let mut name = String::new();
        let mut type_name = String::new();
        let mut current_offset = offset;

        for child in node.children {
            match child {
                GreenTree::Node(n) => {
                    if type_name.is_empty() && (n.kind == IdlElementType::Identifier || n.kind.is_basic_type()) {
                        type_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                    }
                    else if !type_name.is_empty() && name.is_empty() && n.kind == IdlElementType::Identifier {
                        name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                    }
                    current_offset += n.byte_length as usize;
                }
                GreenTree::Leaf(l) => {
                    if type_name.is_empty() && l.kind == IdlTokenType::Identifier {
                        type_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                    }
                    else if !type_name.is_empty() && name.is_empty() && l.kind == IdlTokenType::Identifier {
                        name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                    }
                    current_offset += l.length as usize;
                }
            }
        }

        Ok(Some(Field { name, type_name }))
    }

    /// Build a parameter from a green node
    fn build_param(&self, node: &GreenNode<IdlLanguage>, offset: usize, source: &SourceText) -> Result<Option<Param>, OakError> {
        let mut name = String::new();
        let mut type_name = String::new();
        let mut direction = ParamDirection::In;
        let mut current_offset = offset;

        for child in node.children {
            match child {
                GreenTree::Node(n) => {
                    if n.kind == IdlElementType::In {
                        direction = ParamDirection::In;
                    }
                    else if n.kind == IdlElementType::Out {
                        direction = ParamDirection::Out;
                    }
                    else if n.kind == IdlElementType::Inout {
                        direction = ParamDirection::Inout;
                    }
                    else if type_name.is_empty() && (n.kind == IdlElementType::Identifier || n.kind.is_basic_type()) {
                        type_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                    }
                    else if !type_name.is_empty() && name.is_empty() && n.kind == IdlElementType::Identifier {
                        name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + n.byte_length as usize }).to_string();
                    }
                    current_offset += n.byte_length as usize;
                }
                GreenTree::Leaf(l) => {
                    if l.kind == IdlTokenType::In {
                        direction = ParamDirection::In;
                    }
                    else if l.kind == IdlTokenType::Out {
                        direction = ParamDirection::Out;
                    }
                    else if l.kind == IdlTokenType::Inout {
                        direction = ParamDirection::Inout;
                    }
                    else if type_name.is_empty() && l.kind == IdlTokenType::Identifier {
                        type_name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                    }
                    else if !type_name.is_empty() && name.is_empty() && l.kind == IdlTokenType::Identifier {
                        name = source.get_text_in(core::range::Range { start: current_offset, end: current_offset + l.length as usize }).to_string();
                    }
                    current_offset += l.length as usize;
                }
            }
        }

        Ok(Some(Param { name, type_name, direction }))
    }
}

impl<'config> Builder<IdlLanguage> for IdlBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<IdlLanguage>) -> OakDiagnostics<IdlRoot> {
        let parser = IdlParser::new(self.config);
        let mut cache = oak_core::parser::session::ParseSession::<IdlLanguage>::default();
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
