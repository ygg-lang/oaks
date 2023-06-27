#[cfg(feature = "oak-pretty-print")]
use crate::ast::*;
#[cfg(feature = "oak-pretty-print")]
use oak_pretty_print::{AsDocument, Document, LINE as line, NIL as nil, SOFT_LINE_SPACE as soft_space, doc, indent};

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqRoot {
    type Params = ();

    fn as_document(&self, _params: &Self::Params) -> Document<'_> {
        Document::join(self.items.iter().map(|it| it.as_document(&())), line)
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqItem {
    type Params = ();

    fn as_document(&self, params: &Self::Params) -> Document<'_> {
        match self {
            RbqItem::Struct(it) => it.as_document(params),
            RbqItem::Union(it) => it.as_document(params),
            RbqItem::Enum(it) => it.as_document(params),
            RbqItem::Trait(it) => it.as_document(params),
            RbqItem::Namespace(it) => it.as_document(params),
            RbqItem::TypeAlias(it) => it.as_document(params),
            RbqItem::Micro(it) => it.as_document(params),
            RbqItem::Import(it) => it.as_document(params),
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqStruct {
    type Params = ();

    fn as_document(&self, params: &Self::Params) -> Document<'_> {
        let header = doc!(Document::join(self.annotations.iter().map(|it| it.as_document(params)), line), if !self.annotations.is_empty() { line } else { nil }, "model", soft_space, self.name.as_str(), soft_space, "{");

        let body = doc!(Document::join(self.fields.iter().map(|it| it.as_document(params)), line),);

        Document::group(doc!(header, indent(doc!(line, body)), line, "}"))
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqNamespace {
    type Params = ();

    fn as_document(&self, _params: &Self::Params) -> Document<'_> {
        Document::group(doc!("namespace", soft_space, self.path.as_str(), soft_space, "{", indent(doc!(line,)), line, "}"))
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqUnion {
    type Params = ();

    fn as_document(&self, params: &Self::Params) -> Document<'_> {
        let header = doc!(Document::join(self.annotations.iter().map(|it| it.as_document(params)), line), if !self.annotations.is_empty() { line } else { nil }, "union", soft_space, self.name.as_str(), soft_space, "{");

        Document::group(doc!(header, indent(doc!(line, Document::join(self.members.iter().map(|it| it.as_document(params)), line),)), line, "}"))
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqEnum {
    type Params = ();

    fn as_document(&self, params: &Self::Params) -> Document<'_> {
        let header = doc!(Document::join(self.annotations.iter().map(|it| it.as_document(params)), line), if !self.annotations.is_empty() { line } else { nil }, "enum", soft_space, self.name.as_str(), soft_space, "{");

        Document::group(doc!(header, indent(doc!(line, Document::join(self.variants.iter().map(|it| Document::text(it.as_str())), line),)), line, "}"))
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqUnionMember {
    type Params = ();

    fn as_document(&self, params: &Self::Params) -> Document<'_> {
        doc!(
            Document::join(self.annotations.iter().map(|it| it.as_document(params)), line),
            if !self.annotations.is_empty() { line } else { nil },
            self.name.as_str(),
            self.payload.as_ref().map(|it| it.as_document(params)).unwrap_or(nil),
            self.value.as_ref().map(|it| doc!(soft_space, "=", soft_space, it.as_str())).unwrap_or(nil),
            ";"
        )
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqUnionPayload {
    type Params = ();

    fn as_document(&self, params: &Self::Params) -> Document<'_> {
        match self {
            RbqUnionPayload::Tuple(it) => doc!("(", Document::join(it.iter().map(|t| t.as_document(params)), doc!(",", soft_space)), ")"),
            RbqUnionPayload::Struct(it) => Document::group(doc!(soft_space, "{", indent(doc!(line, Document::join(it.iter().map(|f| f.as_document(params)), line),)), line, "}")),
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqTrait {
    type Params = ();

    fn as_document(&self, params: &Self::Params) -> Document<'_> {
        Document::group(doc!(
            Document::join(self.annotations.iter().map(|it| it.as_document(params)), line),
            if !self.annotations.is_empty() { line } else { nil },
            "trait",
            soft_space,
            self.name.as_str(),
            soft_space,
            "{",
            indent(doc!(line, Document::join(self.items.iter().map(|it| it.as_document(params)), line),)),
            line,
            "}"
        ))
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqTraitItem {
    type Params = ();

    fn as_document(&self, params: &Self::Params) -> Document<'_> {
        match self {
            RbqTraitItem::Field(f) => f.as_document(params),
            RbqTraitItem::Method(m) => m.as_document(params),
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqField {
    type Params = ();

    fn as_document(&self, params: &Self::Params) -> Document<'_> {
        doc!(
            Document::join(self.annotations.iter().map(|it| it.as_document(params)), line),
            if !self.annotations.is_empty() { line } else { nil },
            self.name.as_str(),
            ":",
            soft_space,
            self.type_ref.as_document(params),
            self.default_value.as_ref().map(|it| doc!(soft_space, "=", soft_space, it.as_document(params))).unwrap_or(nil),
            ";"
        )
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqTypeAlias {
    type Params = ();

    fn as_document(&self, params: &Self::Params) -> Document<'_> {
        doc!(Document::join(self.annotations.iter().map(|it| it.as_document(params)), line), if !self.annotations.is_empty() { line } else { nil }, "type", soft_space, self.name.as_str(), soft_space, "=", soft_space, self.type_ref.as_document(params), ";")
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqAnnotation {
    type Params = ();

    fn as_document(&self, _params: &Self::Params) -> Document<'_> {
        doc!("@", self.name.as_str(), if self.args.is_empty() { nil } else { doc!("(", Document::join(self.args.iter().map(|it| Document::text(it.as_str())), doc!(",", soft_space)), ")") })
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqMicro {
    type Params = ();

    fn as_document(&self, params: &Self::Params) -> Document<'_> {
        doc!("micro", soft_space, self.name.as_str(), "(", Document::join(self.args.iter().map(|a| doc!(a.name.as_str(), ":", soft_space, a.type_ref.as_document(params))), doc!(",", soft_space)), ")")
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqImport {
    type Params = ();

    fn as_document(&self, _params: &Self::Params) -> Document<'_> {
        doc!("import", soft_space, self.path.as_str())
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqType {
    type Params = ();

    fn as_document(&self, params: &Self::Params) -> Document<'_> {
        match self {
            RbqType::Named { path, generic_args, .. } => {
                if generic_args.is_empty() {
                    path.as_str().into()
                }
                else {
                    doc!(path.as_str(), "<", Document::join(generic_args.iter().map(|t| t.as_document(params)), doc!(",", soft_space)), ">")
                }
            }
            RbqType::InlineStruct(fields, _) => {
                doc!("{", Document::join(fields.iter().map(|f| f.as_document(params)), line), "}")
            }
            RbqType::PhysicalRef(inner, _) => {
                doc!("&", inner.as_document(params))
            }
            RbqType::Optional(inner, _) => {
                doc!(inner.as_document(params), "?")
            }
            RbqType::Literal(value, _) => value.as_str().into(),
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqLiteral {
    type Params = ();

    fn as_document(&self, _params: &Self::Params) -> Document<'_> {
        match self {
            RbqLiteral::String(s) => format!("\"{}\"", s).into(),
            RbqLiteral::Number(n) => n.as_str().into(),
            RbqLiteral::Boolean(b) => b.to_string().into(),
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqExpr {
    type Params = ();

    fn as_document(&self, params: &Self::Params) -> Document<'_> {
        match &self.kind {
            RbqExprKind::Literal(it) => it.as_document(params),
            RbqExprKind::Identifier(it) => it.as_str().into(),
            RbqExprKind::MagicVar(it) => it.as_str().into(),
            RbqExprKind::Binary { left, op, right } => {
                doc!(left.as_document(params), soft_space, op.as_str(), soft_space, right.as_document(params))
            }
            RbqExprKind::Unary { op, expr } => {
                doc!(op.as_str(), expr.as_document(params))
            }
            RbqExprKind::Call { callee, args } => {
                doc!(callee.as_document(params), "(", Document::join(args.iter().map(|a| a.as_document(params)), doc!(",", soft_space)), ")")
            }
            RbqExprKind::Member { object, property } => {
                doc!(object.as_document(params), ".", property.as_str())
            }
            RbqExprKind::Pipeline { base, steps } => {
                doc!(base.as_document(params), Document::join(steps.iter().map(|s| doc!(soft_space, "|", soft_space, s.as_document(params))), nil))
            }
            RbqExprKind::Closure { args, body } => {
                doc!("{", soft_space, Document::join(args.iter().map(|a| Document::text(a.as_str())), doc!(",", soft_space)), soft_space, "->", soft_space, Document::join(body.iter().map(|e| e.as_document(params)), line), soft_space, "}")
            }
            RbqExprKind::Block(exprs) => {
                doc!("{", Document::join(exprs.iter().map(|e| e.as_document(params)), line), "}")
            }
        }
    }
}

#[cfg(feature = "oak-pretty-print")]
impl AsDocument for RbqPipelineStep {
    type Params = ();

    fn as_document(&self, params: &Self::Params) -> Document<'_> {
        if self.args.is_empty() { self.name.as_str().into() } else { doc!(self.name.as_str(), soft_space, Document::join(self.args.iter().map(|a| a.as_document(params)), doc!(",", soft_space))) }
    }
}
