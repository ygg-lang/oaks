use crate::{ast::*, language::RustLanguage, lexer::RustTokenType, parser::RustElementType};
use core::range::Range;
use oak_core::{BuilderCache, RedNode, RedTree, SourceText};

impl<'config> super::RustBuilder<'config> {
    /// Builds generic parameters and where clauses from a node's children.
    pub fn build_generics_and_where(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Option<Generics>, oak_core::OakError> {
        let mut params = Vec::new();
        let mut where_clause = None;
        let mut generics_span: Option<Range<usize>> = None;

        let children: Vec<_> = node.children().collect();
        let mut i = 0;
        while i < children.len() {
            match &children[i] {
                RedTree::Leaf(t) if t.kind == RustTokenType::Lt => {
                    let start = t.span.start;
                    let mut j = i + 1;
                    while j < children.len() {
                        if let RedTree::Leaf(t_end) = &children[j] {
                            if t_end.kind == RustTokenType::Gt {
                                generics_span = Some(Range { start, end: t_end.span.end });
                                // Parse parameters inside < >
                                params = self.parse_generic_params(&children[i + 1..j], source)?;
                                i = j;
                                break;
                            }
                        }
                        j += 1;
                    }
                }
                RedTree::Leaf(t) if t.kind == RustTokenType::Where => {
                    let start = t.span.start;
                    // Find where the where clause ends (usually before a { or ;)
                    let mut j = i + 1;
                    while j < children.len() {
                        match &children[j] {
                            RedTree::Leaf(t_end) if t_end.kind == RustTokenType::LeftBrace || t_end.kind == RustTokenType::Semicolon => {
                                let end = t_end.span.start;
                                where_clause = Some(self.parse_where_clause(&children[i + 1..j], Range { start, end }, source)?);
                                i = j - 1;
                                break;
                            }
                            _ => j += 1,
                        }
                    }
                }
                _ => {}
            }
            i += 1;
        }

        if params.is_empty() && where_clause.is_none() {
            Ok(None)
        }
        else {
            let span = match (generics_span, &where_clause) {
                (Some(g), Some(w)) => Range { start: g.start, end: w.span.end },
                (Some(g), None) => g,
                (None, Some(w)) => w.span.clone(),
                (None, None) => Range { start: 0, end: 0 },
            };
            Ok(Some(Generics { params, where_clause, span }))
        }
    }

    fn parse_generic_params(&self, children: &[RedTree<RustLanguage>], source: &SourceText) -> Result<Vec<GenericParam>, oak_core::OakError> {
        let mut params = Vec::new();
        let mut current_start = 0;

        while current_start < children.len() {
            // Find next comma or end
            let mut current_end = current_start;
            while current_end < children.len() {
                if let RedTree::Leaf(t) = &children[current_end] {
                    if t.kind == RustTokenType::Comma {
                        break;
                    }
                }
                current_end += 1;
            }

            if current_start < current_end {
                params.push(self.parse_single_generic_param(&children[current_start..current_end], source)?);
            }

            current_start = current_end + 1;
        }

        Ok(params)
    }

    fn parse_single_generic_param(&self, children: &[RedTree<RustLanguage>], source: &SourceText) -> Result<GenericParam, oak_core::OakError> {
        if children.is_empty() {
            return Err(oak_core::OakError::syntax_error("Empty generic parameter".to_string(), 0, None));
        }

        // Handle attributes (skipped for now but structure is here)
        let mut i = 0;
        while i < children.len() {
            if let RedTree::Leaf(t) = &children[i] {
                if t.kind == RustTokenType::Hash {
                    // Skip attribute for now
                    let mut j = i + 1;
                    let mut bracket_count = 0;
                    while j < children.len() {
                        if let RedTree::Leaf(t_inner) = &children[j] {
                            if t_inner.kind == RustTokenType::LeftBracket {
                                bracket_count += 1;
                            }
                            else if t_inner.kind == RustTokenType::RightBracket {
                                bracket_count -= 1;
                                if bracket_count == 0 {
                                    i = j;
                                    break;
                                }
                            }
                        }
                        j += 1;
                    }
                }
                else {
                    break;
                }
            }
            else {
                break;
            }
            i += 1;
        }

        let remaining = &children[i..];
        if remaining.is_empty() {
            return Err(oak_core::OakError::syntax_error("Generic parameter name missing after attributes".to_string(), children[0].span().start, None));
        }

        // Lifetime parameter: 'a: 'b + 'c
        if let RedTree::Leaf(t) = &remaining[0] {
            if t.kind == RustTokenType::Lifetime {
                let lifetime = Lifetime { name: crate::builder::text(source, t.span.clone().into()), span: t.span.clone().into() };

                let mut bounds = Vec::new();
                if remaining.len() > 1 {
                    if let RedTree::Leaf(t_colon) = &remaining[1] {
                        if t_colon.kind == RustTokenType::Colon {
                            for child in &remaining[2..] {
                                if let RedTree::Leaf(t_bound) = child {
                                    if t_bound.kind == RustTokenType::Lifetime {
                                        bounds.push(Lifetime { name: crate::builder::text(source, t_bound.span.clone().into()), span: t_bound.span.clone().into() });
                                    }
                                }
                            }
                        }
                    }
                }
                // Update GenericParam::Lifetime to include bounds if AST supports it,
                // but currently AST GenericParam::Lifetime(Lifetime) only takes one Lifetime.
                // We'll stick to the current AST but noted the limitation.
                return Ok(GenericParam::Lifetime(lifetime));
            }
        }

        // Const parameter: const N: usize = 10
        if let RedTree::Leaf(t) = &remaining[0] {
            if t.kind == RustTokenType::Const {
                let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
                let mut ty = Type::Path("usize".to_string()); // Default fallback
                let mut default = None;

                let mut j = 1;
                while j < remaining.len() {
                    match &remaining[j] {
                        RedTree::Leaf(t_id) if t_id.kind == RustTokenType::Identifier => {
                            name.name = crate::builder::text(source, t_id.span.clone().into());
                            name.span = t_id.span.clone().into();
                        }
                        RedTree::Leaf(t_colon) if t_colon.kind == RustTokenType::Colon => {
                            if j + 1 < remaining.len() {
                                if let RedTree::Node(n_ty) = &remaining[j + 1] {
                                    ty = self.build_type(n_ty.clone(), source)?;
                                    j += 1;
                                }
                            }
                        }
                        RedTree::Leaf(t_eq) if t_eq.kind == RustTokenType::Eq => {
                            if j + 1 < remaining.len() {
                                if let RedTree::Node(n_expr) = &remaining[j + 1] {
                                    default = Some(self.build_expr(n_expr.clone(), source)?);
                                    j += 1;
                                }
                            }
                        }
                        _ => {}
                    }
                    j += 1;
                }
                return Ok(GenericParam::Const { name, ty, default });
            }
        }

        // Type parameter: T: Bound1 + Bound2 = DefaultType
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
        let mut bounds = Vec::new();
        let mut default = None;

        let mut j = 0;
        while j < remaining.len() {
            match &remaining[j] {
                RedTree::Leaf(t_id) if t_id.kind == RustTokenType::Identifier => {
                    name.name = crate::builder::text(source, t_id.span.clone().into());
                    name.span = t_id.span.clone().into();
                }
                RedTree::Leaf(t_colon) if t_colon.kind == RustTokenType::Colon => {
                    let mut k = j + 1;
                    while k < remaining.len() {
                        if let RedTree::Leaf(t_eq) = &remaining[k] {
                            if t_eq.kind == RustTokenType::Eq {
                                break;
                            }
                        }
                        k += 1;
                    }
                    bounds = self.parse_type_param_bounds(&remaining[j + 1..k], source)?;
                    j = k - 1;
                }
                RedTree::Leaf(t_eq) if t_eq.kind == RustTokenType::Eq => {
                    if j + 1 < remaining.len() {
                        if let RedTree::Node(n_ty) = &remaining[j + 1] {
                            default = Some(self.build_type(n_ty.clone(), source)?);
                            j += 1;
                        }
                    }
                }
                _ => {}
            }
            j += 1;
        }

        Ok(GenericParam::Type { name, bounds, default })
    }

    fn parse_type_param_bounds(&self, children: &[RedTree<RustLanguage>], source: &SourceText) -> Result<Vec<TypeParamBound>, oak_core::OakError> {
        let mut bounds = Vec::new();
        for child in children {
            match child {
                RedTree::Leaf(t) if t.kind == RustTokenType::Lifetime => {
                    bounds.push(TypeParamBound::Lifetime(Lifetime { name: crate::builder::text(source, t.span.clone().into()), span: t.span.clone().into() }));
                }
                RedTree::Node(n) => {
                    bounds.push(TypeParamBound::Trait(self.build_type(n.clone(), source)?));
                }
                _ => {}
            }
        }
        Ok(bounds)
    }

    fn parse_where_clause(&self, children: &[RedTree<RustLanguage>], span: Range<usize>, source: &SourceText) -> Result<WhereClause, oak_core::OakError> {
        let mut predicates = Vec::new();
        let mut current_start = 0;

        while current_start < children.len() {
            let mut current_end = current_start;
            while current_end < children.len() {
                if let RedTree::Leaf(t) = &children[current_end] {
                    if t.kind == RustTokenType::Comma {
                        break;
                    }
                }
                current_end += 1;
            }

            if current_start < current_end {
                predicates.push(self.parse_where_predicate(&children[current_start..current_end], source)?);
            }

            current_start = current_end + 1;
        }

        Ok(WhereClause { predicates, span })
    }

    fn parse_where_predicate(&self, children: &[RedTree<RustLanguage>], source: &SourceText) -> Result<WherePredicate, oak_core::OakError> {
        let mut colon_idx = None;
        for (i, child) in children.iter().enumerate() {
            if let RedTree::Leaf(t) = child {
                if t.kind == RustTokenType::Colon {
                    colon_idx = Some(i);
                    break;
                }
            }
        }

        if let Some(idx) = colon_idx {
            let left = &children[..idx];
            let right = &children[idx + 1..];

            if left.len() == 1 {
                if let RedTree::Leaf(t) = &left[0] {
                    if t.kind == RustTokenType::Lifetime {
                        let lifetime = Lifetime { name: crate::builder::text(source, t.span.clone().into()), span: t.span.clone().into() };
                        let mut bounds = Vec::new();
                        for child in right {
                            if let RedTree::Leaf(t_bound) = child {
                                if t_bound.kind == RustTokenType::Lifetime {
                                    bounds.push(Lifetime { name: crate::builder::text(source, t_bound.span.clone().into()), span: t_bound.span.clone().into() });
                                }
                            }
                        }
                        return Ok(WherePredicate::Lifetime { lifetime, bounds });
                    }
                }
            }

            let bounded_ty = if left.len() == 1 {
                if let RedTree::Node(n) = &left[0] { self.build_type(n.clone(), source)? } else { Type::Path(crate::builder::text(source, left[0].span().into())) }
            }
            else {
                Type::Path(crate::builder::text(source, Range { start: left[0].span().start, end: left.last().unwrap().span().end }))
            };

            let bounds = self.parse_type_param_bounds(right, source)?;
            Ok(WherePredicate::Type { bounded_ty, bounds })
        }
        else {
            Err(oak_core::OakError::syntax_error("Missing colon in where predicate".to_string(), children[0].span().start, None))
        }
    }
}
