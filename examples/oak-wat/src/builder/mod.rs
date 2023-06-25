use crate::{WatParser, ast::*, language::WatLanguage, lexer::token_type::WatTokenType, parser::element_type::WatElementType};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// AST builder for the WebAssembly Text (WAT) format.
#[derive(Clone, Copy)]
pub struct WatBuilder<'config> {
    /// Language configuration
    config: &'config WatLanguage,
}

impl<'config> WatBuilder<'config> {
    /// Creates a new `WatBuilder` with the given configuration.
    pub const fn new(config: &'config WatLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<WatLanguage> for WatBuilder<'config> {
    fn build<'a, S: oak_core::source::Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<WatLanguage>) -> BuildOutput<WatLanguage> {
        let parser = WatParser::new(self.config);
        let mut session = oak_core::parser::session::ParseSession::<WatLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut session);

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

impl<'config> WatBuilder<'config> {
    fn build_root(&self, green_tree: GreenNode<WatLanguage>, source: &SourceText) -> Result<WatRoot, OakError> {
        let red_root = RedNode::new(&green_tree, 0);
        let mut items = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    WatElementType::Module => {
                        items.push(WatItem::Module(self.build_module(n, source)?));
                    }
                    _ => {}
                }
            }
        }

        Ok(WatRoot { items })
    }

    fn build_module(&self, node: RedNode<WatLanguage>, source: &SourceText) -> Result<WatModule, OakError> {
        let mut name = None;
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == WatTokenType::Identifier {
                        name = Some(source.get_text_in(t.span.clone()).to_string());
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    WatElementType::Func => {
                        items.push(WatModuleField::Func(self.build_func(n, source)?));
                    }
                    WatElementType::Export => {
                        items.push(WatModuleField::Export(self.build_export(n, source)?));
                    }
                    WatElementType::Import => {
                        items.push(WatModuleField::Import(self.build_import(n, source)?));
                    }
                    WatElementType::Type => {
                        items.push(WatModuleField::Type(self.build_type(n, source)?));
                    }
                    WatElementType::Table => {
                        items.push(WatModuleField::Table(self.build_table(n, source)?));
                    }
                    WatElementType::Memory => {
                        items.push(WatModuleField::Memory(self.build_memory(n, source)?));
                    }
                    WatElementType::Global => {
                        items.push(WatModuleField::Global(self.build_global(n, source)?));
                    }
                    _ => {}
                },
            }
        }

        Ok(WatModule { name, items })
    }

    fn build_func(&self, node: RedNode<WatLanguage>, source: &SourceText) -> Result<WatFunc, OakError> {
        let mut name = None;
        let mut params = Vec::new();
        let mut results = Vec::new();
        let mut locals = Vec::new();
        let mut body = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == WatTokenType::Identifier {
                        name = Some(source.get_text_in(t.span.clone().into()).to_string());
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    WatElementType::Param => {
                        params.push(self.build_param(n, source)?);
                    }
                    WatElementType::Result => {
                        results.push(self.build_result(n, source)?);
                    }
                    WatElementType::Local => {
                        locals.push(self.build_local(n, source)?);
                    }
                    WatElementType::Instruction => {
                        body.push(self.build_instruction(n, source)?);
                    }
                    _ => {}
                },
            }
        }

        Ok(WatFunc { name, params, results, locals, body })
    }

    fn build_param(&self, node: RedNode<WatLanguage>, source: &SourceText) -> Result<WatParam, OakError> {
        let mut name = None;
        let mut ty = WatTypeKind::I32; // Default

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == WatTokenType::Identifier {
                        name = Some(source.get_text_in(t.span.clone()).to_string());
                    }
                    else if let Some(t_kind) = self.parse_type_kind(&source.get_text_in(t.span.clone())) {
                        ty = t_kind;
                    }
                }
                _ => {}
            }
        }

        Ok(WatParam { name, ty })
    }

    fn build_result(&self, node: RedNode<WatLanguage>, source: &SourceText) -> Result<WatResult, OakError> {
        let mut ty = WatTypeKind::I32; // Default

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if let Some(t_kind) = self.parse_type_kind(&source.get_text_in(t.span.clone())) {
                    ty = t_kind;
                }
            }
        }

        Ok(WatResult { ty })
    }

    fn build_local(&self, node: RedNode<WatLanguage>, source: &SourceText) -> Result<WatLocal, OakError> {
        let mut name = None;
        let mut ty = WatTypeKind::I32; // Default

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == WatTokenType::Identifier {
                        name = Some(source.get_text_in(t.span.clone()).to_string());
                    }
                    else if let Some(t_kind) = self.parse_type_kind(&source.get_text_in(t.span.clone())) {
                        ty = t_kind;
                    }
                }
                _ => {}
            }
        }

        Ok(WatLocal { name, ty })
    }

    fn build_instruction(&self, node: RedNode<WatLanguage>, source: &SourceText) -> Result<WatInstruction, OakError> {
        let mut opcode = String::new();
        let mut args = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    let text = source.get_text_in(t.span.clone()).to_string();
                    if text != "(" && text != ")" {
                        if opcode.is_empty() {
                            opcode = text;
                        }
                        else {
                            args.push(text);
                        }
                    }
                }
                RedTree::Node(n) => {
                    if n.green.kind == WatElementType::Instruction {
                        // For nested instructions, we could handle them if WatInstruction was recursive.
                        // For now, we'll just treat the inner instruction's opcode as an argument or similar.
                        // Let's just flatten the text for now as a simple fallback.
                        args.push(source.get_text_in(n.span().into()).to_string());
                    }
                }
            }
        }

        match opcode.as_str() {
            "unreachable" => Ok(WatInstruction::Unreachable),
            "nop" => Ok(WatInstruction::Nop),
            "drop" => Ok(WatInstruction::Drop),
            "select" => Ok(WatInstruction::Select),
            "return" => Ok(WatInstruction::Return),
            "local.get" if !args.is_empty() => Ok(WatInstruction::LocalGet(args[0].clone())),
            "local.set" if !args.is_empty() => Ok(WatInstruction::LocalSet(args[0].clone())),
            "local.tee" if !args.is_empty() => Ok(WatInstruction::LocalTee(args[0].clone())),
            "global.get" if !args.is_empty() => Ok(WatInstruction::GlobalGet(args[0].clone())),
            "global.set" if !args.is_empty() => Ok(WatInstruction::GlobalSet(args[0].clone())),
            "i32.const" if !args.is_empty() => Ok(WatInstruction::I32Const(args[0].parse().unwrap_or(0))),
            "i64.const" if !args.is_empty() => Ok(WatInstruction::I64Const(args[0].parse().unwrap_or(0))),
            "f32.const" if !args.is_empty() => Ok(WatInstruction::F32Const(args[0].parse().unwrap_or(0.0))),
            "f64.const" if !args.is_empty() => Ok(WatInstruction::F64Const(args[0].parse().unwrap_or(0.0))),
            "i32.add" => Ok(WatInstruction::I32Add),
            "i32.sub" => Ok(WatInstruction::I32Sub),
            "i32.mul" => Ok(WatInstruction::I32Mul),
            "i64.add" => Ok(WatInstruction::I64Add),
            "i64.sub" => Ok(WatInstruction::I64Sub),
            "i64.mul" => Ok(WatInstruction::I64Mul),
            _ => Ok(WatInstruction::Other(opcode, args)),
        }
    }

    fn build_export(&self, node: RedNode<WatLanguage>, source: &SourceText) -> Result<WatExport, OakError> {
        let mut name = String::new();
        let mut kind = String::new();
        let mut id = String::new();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                let text = source.get_text_in(t.span.clone().into()).to_string();
                if t.kind == WatTokenType::StringLiteral {
                    name = text.trim_matches('"').to_string();
                }
                else if text == "(" || text == ")" || text == "export" {
                    continue;
                }
                else if kind.is_empty() {
                    kind = text;
                }
                else {
                    id = text;
                }
            }
        }

        Ok(WatExport { name, kind, id })
    }

    fn build_import(&self, node: RedNode<WatLanguage>, source: &SourceText) -> Result<WatImport, OakError> {
        let mut module = String::new();
        let mut name = String::new();
        let mut kind = String::new();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                let text = source.get_text_in(t.span.clone().into()).to_string();
                if t.kind == WatTokenType::StringLiteral {
                    if module.is_empty() {
                        module = text.trim_matches('"').to_string();
                    }
                    else {
                        name = text.trim_matches('"').to_string();
                    }
                }
                else if text == "(" || text == ")" || text == "import" {
                    continue;
                }
                else if kind.is_empty() {
                    kind = text;
                }
            }
        }

        Ok(WatImport { module, name, kind })
    }

    fn build_type(&self, node: RedNode<WatLanguage>, source: &SourceText) -> Result<WatType, OakError> {
        let mut id = None;
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == WatTokenType::Identifier {
                    id = Some(source.get_text_in(t.span.clone()).to_string());
                }
            }
        }
        Ok(WatType { id })
    }

    fn build_table(&self, node: RedNode<WatLanguage>, source: &SourceText) -> Result<WatTable, OakError> {
        let mut id = None;
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == WatTokenType::Identifier {
                    id = Some(source.get_text_in(t.span.clone()).to_string());
                }
            }
        }
        Ok(WatTable { id, span: node.span() })
    }

    fn build_memory(&self, node: RedNode<WatLanguage>, source: &SourceText) -> Result<WatMemory, OakError> {
        let mut id = None;
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == WatTokenType::Identifier {
                    id = Some(source.get_text_in(t.span.clone()).to_string());
                }
            }
        }
        Ok(WatMemory { id, span: node.span() })
    }

    fn build_global(&self, node: RedNode<WatLanguage>, source: &SourceText) -> Result<WatGlobal, OakError> {
        let mut id = None;
        let mut ty = WatTypeKind::I32;
        let mut mutable = false;

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                let text = source.get_text_in(t.span.clone()).to_string();
                if t.kind == WatTokenType::Identifier {
                    id = Some(text);
                }
                else if text == "mut" {
                    mutable = true;
                }
                else if let Some(k) = self.parse_type_kind(&text) {
                    ty = k;
                }
            }
        }

        Ok(WatGlobal { id, ty, mutable })
    }

    fn parse_type_kind(&self, text: &str) -> Option<WatTypeKind> {
        match text {
            "i32" => Some(WatTypeKind::I32),
            "i64" => Some(WatTypeKind::I64),
            "f32" => Some(WatTypeKind::F32),
            "f64" => Some(WatTypeKind::F64),
            _ => None,
        }
    }
}
