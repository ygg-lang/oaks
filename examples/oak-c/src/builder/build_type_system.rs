use crate::ast::*;
use oak_core::SourceText;

impl<'config> super::CBuilder<'config> {
    /// Builds a canonical `Type` from declaration specifiers and a declarator.
    pub fn build_type(&self, specifiers: &[DeclarationSpecifier], declarator: &Declarator, source: &SourceText) -> Type {
        let base_type = self.extract_base_type(specifiers);
        self.apply_declarator(base_type, declarator, source)
    }

    /// Extracts the base type from declaration specifiers.
    fn extract_base_type(&self, specifiers: &[DeclarationSpecifier]) -> Type {
        // Handle combined specifiers like "unsigned long long int"
        let mut _is_unsigned = false;
        let mut _is_signed = false;
        let mut long_count = 0;
        let mut is_short = false;
        let mut base: Option<TypeSpecifier> = None;

        for spec in specifiers {
            if let DeclarationSpecifier::TypeSpecifier(ts) = spec {
                match ts {
                    TypeSpecifier::Unsigned { .. } => _is_unsigned = true,
                    TypeSpecifier::Signed { .. } => _is_signed = true,
                    TypeSpecifier::Long { .. } => long_count += 1,
                    TypeSpecifier::Short { .. } => is_short = true,
                    TypeSpecifier::Int { .. }
                    | TypeSpecifier::Char { .. }
                    | TypeSpecifier::Float { .. }
                    | TypeSpecifier::Double { .. }
                    | TypeSpecifier::Void { .. }
                    | TypeSpecifier::Bool { .. }
                    | TypeSpecifier::StructOrUnion(_)
                    | TypeSpecifier::Enum(_)
                    | TypeSpecifier::TypedefName(_, _) => {
                        base = Some(ts.clone());
                    }
                    _ => {}
                }
            }
        }

        // Simplified logic for combining specifiers
        if let Some(b) = base {
            Type::Base(b)
        }
        else if long_count > 0 {
            Type::Base(TypeSpecifier::Long { span: (0..0).into() })
        }
        else if is_short {
            Type::Base(TypeSpecifier::Short { span: (0..0).into() })
        }
        else {
            Type::Base(TypeSpecifier::Int { span: (0..0).into() })
        }
    }

    /// Applies a declarator to a base type to build the full type.
    fn apply_declarator(&self, mut current_type: Type, declarator: &Declarator, source: &SourceText) -> Type {
        // 1. Apply pointer if present
        if let Some(pointer) = &declarator.pointer {
            current_type = self.apply_pointer(current_type, pointer);
        }

        // 2. Apply direct declarator recursively
        self.apply_direct_declarator(current_type, &declarator.direct_declarator, source)
    }

    fn apply_pointer(&self, mut current_type: Type, pointer: &Pointer) -> Type {
        current_type = Type::Pointer(Box::new(current_type));
        if let Some(inner) = &pointer.pointer { self.apply_pointer(current_type, inner) } else { current_type }
    }

    /// Applies a direct declarator to a base type.
    fn apply_direct_declarator(&self, current_type: Type, direct: &DirectDeclarator, source: &SourceText) -> Type {
        match direct {
            DirectDeclarator::Identifier(_, _) => current_type,
            DirectDeclarator::Declarator(inner, _) => self.apply_declarator(current_type, inner, source),
            DirectDeclarator::Array { direct_declarator, assignment_expression, .. } => {
                // In C, array and function modifiers bind more tightly than pointers.
                // int *a[10] is an array of 10 pointers to int.
                // current_type here is already the "base" (including pointers if any applied so far).
                let element_type = self.apply_direct_declarator(current_type, direct_declarator, source);
                Type::Array { element_type: Box::new(element_type), size: assignment_expression.clone() }
            }
            DirectDeclarator::Function { direct_declarator, parameter_list, .. } => {
                let return_type = self.apply_direct_declarator(current_type, direct_declarator, source);
                let mut parameters = Vec::new();
                for param in &parameter_list.parameter_declarations {
                    if let Some(param_type) = self.build_parameter_type(param, source) {
                        parameters.push(param_type);
                    }
                }
                Type::Function { return_type: Box::new(return_type), parameters, variadic: parameter_list.variadic }
            }
        }
    }

    /// Builds a type for a parameter declaration.
    fn build_parameter_type(&self, param: &ParameterDeclaration, source: &SourceText) -> Option<Type> {
        let base_type = self.extract_base_type(&param.declaration_specifiers);

        if let Some(decl) = &param.declarator {
            Some(self.apply_declarator(base_type, decl, source))
        }
        else if let Some(abs_decl) = &param.abstract_declarator {
            Some(self.apply_abstract_declarator(base_type, abs_decl, source))
        }
        else {
            Some(base_type)
        }
    }

    /// Applies an abstract declarator to a base type.
    fn apply_abstract_declarator(&self, mut current_type: Type, abs_decl: &AbstractDeclarator, source: &SourceText) -> Type {
        if let Some(pointer) = &abs_decl.pointer {
            current_type = self.apply_pointer(current_type, pointer);
        }

        if let Some(direct) = &abs_decl.direct_abstract_declarator { self.apply_direct_abstract_declarator(current_type, direct, source) } else { current_type }
    }

    /// Applies a direct abstract declarator to a base type.
    fn apply_direct_abstract_declarator(&self, current_type: Type, direct: &DirectAbstractDeclarator, source: &SourceText) -> Type {
        match direct {
            DirectAbstractDeclarator::AbstractDeclarator(inner) => self.apply_abstract_declarator(current_type, inner, source),
            DirectAbstractDeclarator::Array { declarator, assignment_expression, .. } => {
                let element_type = if let Some(inner) = declarator { self.apply_direct_abstract_declarator(current_type, inner, source) } else { current_type };
                Type::Array { element_type: Box::new(element_type), size: assignment_expression.clone() }
            }
            DirectAbstractDeclarator::Function { declarator, parameter_list, .. } => {
                let return_type = if let Some(inner) = declarator { self.apply_direct_abstract_declarator(current_type, inner, source) } else { current_type };
                let mut parameters = Vec::new();
                if let Some(list) = parameter_list {
                    for param in &list.parameter_declarations {
                        if let Some(param_type) = self.build_parameter_type(param, source) {
                            parameters.push(param_type);
                        }
                    }
                }
                Type::Function { return_type: Box::new(return_type), parameters, variadic: parameter_list.as_ref().map(|l| l.variadic).unwrap_or(false) }
            }
        }
    }
}
