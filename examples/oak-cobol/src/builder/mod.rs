use crate::{
    ast::*,
    language::CobolLanguage,
    parser::{CobolElementType, CobolParser},
};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// AST builder for the COBOL language.
#[derive(Clone, Copy)]
pub struct CobolBuilder<'config> {
    /// Language configuration.
    config: &'config CobolLanguage,
}

impl<'config> CobolBuilder<'config> {
    /// Creates a new `CobolBuilder` with the given language configuration.
    pub fn new(config: &'config CobolLanguage) -> Self {
        Self { config }
    }

    pub fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, CobolLanguage>, source: &SourceText) -> Result<CobolRoot, OakError> {
        let red_node = RedNode::new(green_tree, 0);
        let mut program = CobolProgram { identification_division: None, environment_division: None, data_division: None, procedure_division: None };

        for child in red_node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    CobolElementType::IdentificationDivision => {
                        program.identification_division = Some(self.build_identification_division(n, source)?);
                    }
                    CobolElementType::DataDivision => {
                        program.data_division = Some(self.build_data_division(n, source)?);
                    }
                    CobolElementType::ProcedureDivision => {
                        program.procedure_division = Some(self.build_procedure_division(n, source)?);
                    }
                    _ => {}
                }
            }
        }

        Ok(CobolRoot::new(program))
    }

    fn build_identification_division<'a>(&self, node: RedNode<'a, CobolLanguage>, source: &SourceText) -> Result<IdentificationDivision, OakError> {
        let mut program_id = String::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == CobolElementType::ProgramIdParagraph {
                    for subchild in n.children() {
                        if let RedTree::Leaf(t) = subchild {
                            if t.kind == crate::lexer::CobolTokenType::Identifier {
                                program_id = source.get_text_in(t.span.clone()).to_string();
                            }
                        }
                    }
                }
            }
        }
        Ok(IdentificationDivision { program_id })
    }

    fn build_data_division<'a>(&self, node: RedNode<'a, CobolLanguage>, source: &SourceText) -> Result<DataDivision, OakError> {
        let mut working_storage_section = None;
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == CobolElementType::WorkingStorageSection {
                    working_storage_section = Some(self.build_working_storage_section(n, source)?);
                }
            }
        }
        Ok(DataDivision { file_section: None, working_storage_section, linkage_section: None })
    }

    fn build_working_storage_section<'a>(&self, node: RedNode<'a, CobolLanguage>, source: &SourceText) -> Result<WorkingStorageSection, OakError> {
        let mut data_items = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == CobolElementType::DataItem {
                    data_items.push(self.build_data_item(n, source)?);
                }
            }
        }
        Ok(WorkingStorageSection { data_items })
    }

    fn build_data_item<'a>(&self, node: RedNode<'a, CobolLanguage>, source: &SourceText) -> Result<DataItem, OakError> {
        let mut level = 0;
        let mut name = String::new();
        let mut picture = None;
        let mut value = None;

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                match t.kind {
                    crate::lexer::CobolTokenType::NumberLiteral => {
                        if level == 0 {
                            level = source.get_text_in(t.span.clone()).parse().unwrap_or(0);
                        }
                        else if value.is_none() {
                            value = Some(source.get_text_in(t.span.clone()).to_string());
                        }
                    }
                    crate::lexer::CobolTokenType::Identifier => {
                        let text = source.get_text_in(t.span.clone()).to_string();
                        if name.is_empty() {
                            name = text;
                        }
                        else if picture.is_none() && (text.to_uppercase() != "VALUE") {
                            // Simplified: assume identifier after PIC is the picture string
                            picture = Some(text);
                        }
                        else if value.is_none() {
                            // This might be tricky, usually value comes after VALUE keyword
                        }
                    }
                    crate::lexer::CobolTokenType::StringLiteral => {
                        let text = source.get_text_in(t.span.clone());
                        value = Some(text.trim_matches('"').trim_matches('\'').to_string());
                    }
                    _ => {}
                }
            }
        }

        Ok(DataItem { level, name, picture, value, occurs: None, redefines: None, usage: None })
    }

    fn build_procedure_division<'a>(&self, node: RedNode<'a, CobolLanguage>, source: &SourceText) -> Result<ProcedureDivision, OakError> {
        let mut statements = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    CobolElementType::DisplayStatement => {
                        statements.push(Statement::Display(self.build_display_statement(n, source)?));
                    }
                    CobolElementType::StopStatement => {
                        statements.push(Statement::Stop(self.build_stop_statement(n, source)?));
                    }
                    CobolElementType::MoveStatement => {
                        statements.push(Statement::Move(self.build_move_statement(n, source)?));
                    }
                    _ => {}
                }
            }
        }
        Ok(ProcedureDivision { sections: Vec::new(), paragraphs: Vec::new(), statements })
    }

    fn build_display_statement<'a>(&self, node: RedNode<'a, CobolLanguage>, source: &SourceText) -> Result<DisplayStatement, OakError> {
        let mut items = Vec::new();
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                match t.kind {
                    crate::lexer::CobolTokenType::StringLiteral => {
                        let text = source.get_text_in(t.span.clone());
                        let text = text.trim_matches('"').trim_matches('\'').to_string();
                        items.push(text);
                    }
                    crate::lexer::CobolTokenType::NumberLiteral | crate::lexer::CobolTokenType::Identifier => {
                        let text = source.get_text_in(t.span.clone()).to_string();
                        items.push(text);
                    }
                    _ => {}
                }
            }
        }
        Ok(DisplayStatement { items })
    }

    fn build_move_statement<'a>(&self, node: RedNode<'a, CobolLanguage>, source: &SourceText) -> Result<MoveStatement, OakError> {
        let mut source_val = String::new();
        let mut targets = Vec::new();
        let mut to_found = false;

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                match t.kind {
                    crate::lexer::CobolTokenType::StringLiteral | crate::lexer::CobolTokenType::NumberLiteral | crate::lexer::CobolTokenType::Identifier => {
                        let text = source.get_text_in(t.span.clone()).to_string();
                        if source_val.is_empty() {
                            source_val = text;
                        }
                        else if to_found {
                            targets.push(text);
                        }
                        else if text.to_uppercase() == "TO" {
                            to_found = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(MoveStatement { source: source_val, targets })
    }

    fn build_stop_statement<'a>(&self, _node: RedNode<'a, CobolLanguage>, _source: &SourceText) -> Result<StopStatement, OakError> {
        // Simplified: assume it's always STOP RUN for now
        Ok(StopStatement { run: true })
    }
}

impl<'config> Builder<CobolLanguage> for CobolBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<CobolLanguage>) -> BuildOutput<CobolLanguage> {
        let parser = CobolParser::new(self.config);
        let mut session = oak_core::parser::ParseSession::<CobolLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
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
