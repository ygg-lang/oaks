use crate::ast::*;
use oak_pretty_print::{AsDocument, Document, LINE as line, NIL as nil, SOFT_LINE_SPACE as soft_space, doc};

impl AsDocument for RbqRoot {
    fn as_document(&self) -> Document<'_> {
        Document::join(self.items.iter().map(|it| it.as_document()), line)
    }
}

impl AsDocument for RbqItem {
    fn as_document(&self) -> Document<'_> {
        match self {
            RbqItem::Struct(it) => it.as_document(),
            RbqItem::Union(it) => it.as_document(),
            RbqItem::Enum(it) => it.as_document(),
            RbqItem::Trait(it) => it.as_document(),
            RbqItem::Namespace(it) => it.as_document(),
            RbqItem::TypeAlias(it) => it.as_document(),
            RbqItem::Micro(it) => it.as_document(),
            RbqItem::Query(it) => it.as_document(),
        }
    }
}

impl AsDocument for RbqStruct {
    fn as_document(&self) -> Document<'_> {
        let header = doc!(Document::join(self.annotations.iter().map(|it| it.as_document()), line), if !self.annotations.is_empty() { line } else { nil }, "struct", soft_space, self.name.as_str(), soft_space, "{");

        let body =
            doc!(Document::join(self.using.iter().map(|u| doc!("using", soft_space, u.as_str(), ";")), line), if !self.using.is_empty() && !self.fields.is_empty() { line } else { nil }, Document::join(self.fields.iter().map(|it| it.as_document()), line),);

        Document::group(doc!(header, indent(doc!(line, body)), line, "}"))
    }
}

impl AsDocument for RbqNamespace {
    fn as_document(&self) -> Document<'_> {
        Document::group(doc!("namespace", soft_space, self.name.as_str(), soft_space, "{", indent(doc!(line, Document::join(self.items.iter().map(|it| it.as_document()), line),)), line, "}"))
    }
}

impl AsDocument for RbqUnion {
    fn as_document(&self) -> Document<'_> {
        let header = doc!(Document::join(self.annotations.iter().map(|it| it.as_document()), line), if !self.annotations.is_empty() { line } else { nil }, "union", soft_space, self.name.as_str(), soft_space, "{");

        Document::group(doc!(header, indent(doc!(line, Document::join(self.members.iter().map(|it| it.as_document()), line),)), line, "}"))
    }
}

impl AsDocument for RbqEnum {
    fn as_document(&self) -> Document<'_> {
        let header = doc!(Document::join(self.annotations.iter().map(|it| it.as_document()), line), if !self.annotations.is_empty() { line } else { nil }, "enum", soft_space, self.name.as_str(), soft_space, "{");

        Document::group(doc!(header, indent(doc!(line, Document::join(self.variants.iter().map(|it| it.as_document()), line),)), line, "}"))
    }
}

impl AsDocument for RbqEnumMember {
    fn as_document(&self) -> Document<'_> {
        doc!(
            Document::join(self.annotations.iter().map(|it| it.as_document()), line),
            if !self.annotations.is_empty() { line } else { nil },
            self.name.as_str(),
            self.value.as_ref().map(|it| doc!(soft_space, "=", soft_space, it.as_str())).unwrap_or(nil),
            ";"
        )
    }
}

impl AsDocument for RbqUnionMember {
    fn as_document(&self) -> Document<'_> {
        doc!(
            Document::join(self.annotations.iter().map(|it| it.as_document()), line),
            if !self.annotations.is_empty() { line } else { nil },
            self.name.as_str(),
            self.payload.as_ref().map(|it| it.as_document()).unwrap_or(nil),
            self.value.as_ref().map(|it| doc!(soft_space, "=", soft_space, it.as_str())).unwrap_or(nil),
            ";"
        )
    }
}

impl AsDocument for RbqUnionPayload {
    fn as_document(&self) -> Document<'_> {
        match self {
            RbqUnionPayload::Tuple(it) => doc!("(", Document::join(it.iter().map(|t| Document::text(t.as_str())), doc!(",", soft_space)), ")"),
            RbqUnionPayload::Struct(it) => Document::group(doc!(soft_space, "{", indent(doc!(line, Document::join(it.iter().map(|f| f.as_document()), line),)), line, "}")),
        }
    }
}

impl AsDocument for RbqTrait {
    fn as_document(&self) -> Document<'_> {
        Document::group(doc!(
            Document::join(self.annotations.iter().map(|it| it.as_document()), line),
            if !self.annotations.is_empty() { line } else { nil },
            "trait",
            soft_space,
            self.name.as_str(),
            soft_space,
            "{",
            indent(doc!(line, Document::join(self.items.iter().map(|it| it.as_document()), line),)),
            line,
            "}"
        ))
    }
}

impl AsDocument for RbqField {
    fn as_document(&self) -> Document<'_> {
        doc!(
            Document::join(self.annotations.iter().map(|it| it.as_document()), line),
            if !self.annotations.is_empty() { line } else { nil },
            self.name.as_str(),
            ":",
            soft_space,
            self.type_ref.as_str(),
            self.default_value.as_ref().map(|it| doc!(soft_space, "=", soft_space, it.as_str())).unwrap_or(nil),
            ";"
        )
    }
}

impl AsDocument for RbqTypeAlias {
    fn as_document(&self) -> Document<'_> {
        doc!(Document::join(self.annotations.iter().map(|it| it.as_document()), line), if !self.annotations.is_empty() { line } else { nil }, "type", soft_space, self.name.as_str(), soft_space, "=", soft_space, self.type_ref.as_str(), ";")
    }
}

impl AsDocument for RbqAnnotation {
    fn as_document(&self) -> Document<'_> {
        doc!("@", self.name.as_str(), if self.args.is_empty() { nil } else { doc!("(", Document::join(self.args.iter().map(|it| Document::text(it.as_str())), doc!(",", soft_space)), ")") })
    }
}

impl AsDocument for RbqMicro {
    fn as_document(&self) -> Document<'_> {
        doc!("micro", soft_space, self.name.as_str(), "(", Document::join(self.args.iter().map(|(n, t)| doc!(n.as_str(), ":", soft_space, t.as_str())), doc!(",", soft_space)), ")")
    }
}

impl AsDocument for RbqExpr {
    fn as_document(&self) -> Document<'_> {
        match &self.kind {
            RbqExprKind::Literal(it) => it.as_str().into(),
            RbqExprKind::Identifier(it) => it.as_str().into(),
            RbqExprKind::MagicVar(it) => it.as_str().into(),
            RbqExprKind::Binary { left, op, right } => {
                doc!(left.as_document(), soft_space, op.as_str(), soft_space, right.as_document())
            }
            RbqExprKind::Unary { op, expr } => {
                doc!(op.as_str(), expr.as_document())
            }
            RbqExprKind::Call { callee, args } => {
                doc!(callee.as_document(), "(", Document::join(args.iter().map(|a| a.as_document()), doc!(",", soft_space)), ")")
            }
            RbqExprKind::Member { object, property } => {
                doc!(object.as_document(), ".", property.as_str())
            }
            RbqExprKind::Pipeline { base, steps } => {
                doc!(base.as_document(), Document::join(steps.iter().map(|s| doc!(soft_space, "|", soft_space, s.as_document())), nil))
            }
            RbqExprKind::Closure { args, body } => {
                doc!("{", soft_space, Document::join(args.iter().map(|a| Document::text(a.as_str())), doc!(",", soft_space)), soft_space, "->", soft_space, body.as_document(), soft_space, "}")
            }
        }
    }
}

impl AsDocument for RbqPipelineStep {
    fn as_document(&self) -> Document<'_> {
        if self.args.is_empty() { self.name.as_str().into() } else { doc!(self.name.as_str(), soft_space, Document::join(self.args.iter().map(|a| a.as_document()), doc!(",", soft_space))) }
    }
}
