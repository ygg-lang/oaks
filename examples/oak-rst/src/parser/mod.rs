/// Element types for the reStructuredText language.
pub mod element_type;

use crate::{language::RstLanguage, lexer::token_type::RstTokenType, parser::element_type::RstElementType as ET};
use oak_core::{GreenNode, OakError, Parser, ParserState, source::Source};

/// Parser for reStructuredText language.
pub struct RstParser<'config> {
    pub(crate) config: &'config RstLanguage,
}

impl<'config> RstParser<'config> {
    /// Creates a new RstParser with the given configuration.
    pub fn new(config: &'config RstLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<RstLanguage> for RstParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::ParseCache<RstLanguage>) -> oak_core::ParseOutput<'a, RstLanguage> {
        let lexer = crate::lexer::RstLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                let item_checkpoint = state.checkpoint();
                if let Some(kind) = state.peek_kind() {
                    match kind {
                        RstTokenType::Comment => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::Comment);
                        }
                        RstTokenType::Heading1 => {
                            state.bump();
                            // 解析标题文本
                            self.parse_inlines_until_newline(state);
                            state.finish_at(item_checkpoint, ET::Heading1);
                        }
                        RstTokenType::Heading2 => {
                            state.bump();
                            // 解析标题文本
                            self.parse_inlines_until_newline(state);
                            state.finish_at(item_checkpoint, ET::Heading2);
                        }
                        RstTokenType::Heading3 => {
                            state.bump();
                            // 解析标题文本
                            self.parse_inlines_until_newline(state);
                            state.finish_at(item_checkpoint, ET::Heading3);
                        }
                        RstTokenType::Heading4 => {
                            state.bump();
                            // 解析标题文本
                            self.parse_inlines_until_newline(state);
                            state.finish_at(item_checkpoint, ET::Heading4);
                        }
                        RstTokenType::Heading5 => {
                            state.bump();
                            // 解析标题文本
                            self.parse_inlines_until_newline(state);
                            state.finish_at(item_checkpoint, ET::Heading5);
                        }
                        RstTokenType::Heading6 => {
                            state.bump();
                            // 解析标题文本
                            self.parse_inlines_until_newline(state);
                            state.finish_at(item_checkpoint, ET::Heading6);
                        }
                        RstTokenType::Directive => {
                            state.bump();
                            // 解析指令参数和选项
                            while state.not_at_end() {
                                if let Some(kind) = state.peek_kind() {
                                    match kind {
                                        RstTokenType::DirectiveArgument => {
                                            state.bump();
                                        }
                                        RstTokenType::DirectiveOption => {
                                            state.bump();
                                        }
                                        RstTokenType::Newline => {
                                            state.bump();
                                            // 检查是否有指令内容
                                            let mut indent_level = 0;
                                            while state.not_at_end() {
                                                if let Some(RstTokenType::Whitespace) = state.peek_kind() {
                                                    state.bump();
                                                    indent_level += 1;
                                                }
                                                else if indent_level > 0 {
                                                    // 指令内容
                                                    self.parse_inlines_until_newline(state);
                                                }
                                                else {
                                                    break;
                                                }
                                            }
                                            break;
                                        }
                                        _ => {
                                            break;
                                        }
                                    }
                                }
                                else {
                                    break;
                                }
                            }
                            state.finish_at(item_checkpoint, ET::Directive);
                        }
                        RstTokenType::BulletListMarker => {
                            let list_checkpoint = item_checkpoint;
                            while state.not_at_end() {
                                if let Some(RstTokenType::BulletListMarker) = state.peek_kind() {
                                    let li_checkpoint = state.checkpoint();
                                    state.bump(); // Marker
                                    self.parse_inlines_until_newline(state);
                                    state.finish_at(li_checkpoint, ET::ListItem);

                                    if let Some(RstTokenType::Newline) = state.peek_kind() {
                                        let nl_checkpoint = state.checkpoint();
                                        state.bump();
                                        // 检查是否有嵌套列表
                                        let mut indent_level = 0;
                                        while state.not_at_end() {
                                            if let Some(RstTokenType::Whitespace) = state.peek_kind() {
                                                state.bump();
                                                indent_level += 1;
                                            }
                                            else if let Some(RstTokenType::BulletListMarker) = state.peek_kind() {
                                                // 递归解析嵌套列表
                                                let nested_list_checkpoint = state.checkpoint();
                                                while state.not_at_end() {
                                                    if let Some(RstTokenType::BulletListMarker) = state.peek_kind() {
                                                        let nested_li_checkpoint = state.checkpoint();
                                                        state.bump(); // Marker
                                                        self.parse_inlines_until_newline(state);
                                                        state.finish_at(nested_li_checkpoint, ET::ListItem);

                                                        if let Some(RstTokenType::Newline) = state.peek_kind() {
                                                            let nested_nl_checkpoint = state.checkpoint();
                                                            state.bump();
                                                            let mut nested_indent_level = 0;
                                                            while state.not_at_end() {
                                                                if let Some(RstTokenType::Whitespace) = state.peek_kind() {
                                                                    state.bump();
                                                                    nested_indent_level += 1;
                                                                }
                                                                else {
                                                                    break;
                                                                }
                                                            }
                                                            if nested_indent_level <= indent_level || !matches!(state.peek_kind(), Some(RstTokenType::BulletListMarker)) {
                                                                state.restore(nested_nl_checkpoint);
                                                                break;
                                                            }
                                                        }
                                                        else {
                                                            break;
                                                        }
                                                    }
                                                    else {
                                                        break;
                                                    }
                                                }
                                                state.finish_at(nested_list_checkpoint, ET::BulletList);
                                                break;
                                            }
                                            else {
                                                state.restore(nl_checkpoint);
                                                break;
                                            }
                                        }
                                    }

                                    if let Some(RstTokenType::Newline) = state.peek_kind() {
                                        let nl_checkpoint = state.checkpoint();
                                        state.bump();
                                        if !matches!(state.peek_kind(), Some(RstTokenType::BulletListMarker)) {
                                            state.restore(nl_checkpoint);
                                            break;
                                        }
                                    }
                                    else {
                                        break;
                                    }
                                }
                                else {
                                    break;
                                }
                            }
                            state.finish_at(list_checkpoint, ET::BulletList);
                        }
                        RstTokenType::EnumeratedListMarker => {
                            let list_checkpoint = item_checkpoint;
                            while state.not_at_end() {
                                if let Some(RstTokenType::EnumeratedListMarker) = state.peek_kind() {
                                    let li_checkpoint = state.checkpoint();
                                    state.bump(); // Marker
                                    self.parse_inlines_until_newline(state);
                                    state.finish_at(li_checkpoint, ET::ListItem);

                                    if let Some(RstTokenType::Newline) = state.peek_kind() {
                                        let nl_checkpoint = state.checkpoint();
                                        state.bump();
                                        // 检查是否有嵌套列表
                                        let mut indent_level = 0;
                                        while state.not_at_end() {
                                            if let Some(RstTokenType::Whitespace) = state.peek_kind() {
                                                state.bump();
                                                indent_level += 1;
                                            }
                                            else if let Some(RstTokenType::EnumeratedListMarker) = state.peek_kind() {
                                                // 递归解析嵌套列表
                                                let nested_list_checkpoint = state.checkpoint();
                                                while state.not_at_end() {
                                                    if let Some(RstTokenType::EnumeratedListMarker) = state.peek_kind() {
                                                        let nested_li_checkpoint = state.checkpoint();
                                                        state.bump(); // Marker
                                                        self.parse_inlines_until_newline(state);
                                                        state.finish_at(nested_li_checkpoint, ET::ListItem);

                                                        if let Some(RstTokenType::Newline) = state.peek_kind() {
                                                            let nested_nl_checkpoint = state.checkpoint();
                                                            state.bump();
                                                            let mut nested_indent_level = 0;
                                                            while state.not_at_end() {
                                                                if let Some(RstTokenType::Whitespace) = state.peek_kind() {
                                                                    state.bump();
                                                                    nested_indent_level += 1;
                                                                }
                                                                else {
                                                                    break;
                                                                }
                                                            }
                                                            if nested_indent_level <= indent_level || !matches!(state.peek_kind(), Some(RstTokenType::EnumeratedListMarker)) {
                                                                state.restore(nested_nl_checkpoint);
                                                                break;
                                                            }
                                                        }
                                                        else {
                                                            break;
                                                        }
                                                    }
                                                    else {
                                                        break;
                                                    }
                                                }
                                                state.finish_at(nested_list_checkpoint, ET::EnumeratedList);
                                                break;
                                            }
                                            else {
                                                state.restore(nl_checkpoint);
                                                break;
                                            }
                                        }
                                    }

                                    if let Some(RstTokenType::Newline) = state.peek_kind() {
                                        let nl_checkpoint = state.checkpoint();
                                        state.bump();
                                        if !matches!(state.peek_kind(), Some(RstTokenType::EnumeratedListMarker)) {
                                            state.restore(nl_checkpoint);
                                            break;
                                        }
                                    }
                                    else {
                                        break;
                                    }
                                }
                                else {
                                    break;
                                }
                            }
                            state.finish_at(list_checkpoint, ET::EnumeratedList);
                        }
                        RstTokenType::DefinitionDefinition => {
                            state.bump();
                            self.parse_inlines_until_newline(state);
                            state.finish_at(item_checkpoint, ET::DefinitionDefinition);
                        }
                        RstTokenType::Table => {
                            let table_checkpoint = item_checkpoint;
                            state.bump();
                            // 解析表格行和单元格
                            self.parse_table(state);
                            state.finish_at(table_checkpoint, ET::Table);
                        }
                        RstTokenType::CodeBlock => {
                            state.bump();
                            // 解析代码块内容
                            while state.not_at_end() {
                                if let Some(RstTokenType::Newline) = state.peek_kind() {
                                    state.bump();
                                    // 检查代码块结束
                                    let mut is_end = true;
                                    for _ in 0..3 {
                                        if let Some(RstTokenType::Text) = state.peek_kind() {
                                            state.bump();
                                        }
                                        else {
                                            is_end = false;
                                            break;
                                        }
                                    }
                                    if is_end {
                                        break;
                                    }
                                }
                                else {
                                    state.bump();
                                }
                            }
                            state.finish_at(item_checkpoint, ET::CodeBlock);
                        }
                        RstTokenType::FootnoteDefinition => {
                            state.bump();
                            // 解析脚注定义内容
                            self.parse_inlines_until_newline(state);
                            state.finish_at(item_checkpoint, ET::FootnoteDefinition);
                        }
                        RstTokenType::HorizontalRule => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::HorizontalRule);
                        }
                        RstTokenType::Newline | RstTokenType::Whitespace => {
                            state.bump();
                        }
                        _ => {
                            self.parse_paragraph(state);
                        }
                    }
                }
                else {
                    state.advance();
                }
            }

            let root = state.finish_at(checkpoint, ET::Root);
            Ok(root)
        })
    }
}

impl<'config> RstParser<'config> {
    fn is_block_start(&self, kind: RstTokenType) -> bool {
        matches!(
            kind,
            RstTokenType::Comment
                | RstTokenType::Heading1
                | RstTokenType::Heading2
                | RstTokenType::Heading3
                | RstTokenType::Heading4
                | RstTokenType::Heading5
                | RstTokenType::Heading6
                | RstTokenType::Directive
                | RstTokenType::BulletListMarker
                | RstTokenType::EnumeratedListMarker
                | RstTokenType::DefinitionDefinition
                | RstTokenType::Table
                | RstTokenType::CodeBlock
                | RstTokenType::HorizontalRule
        )
    }

    fn parse_paragraph<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, RstLanguage, S>) {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            if let Some(next_kind) = state.peek_kind() {
                if next_kind == RstTokenType::Newline {
                    let cp = state.checkpoint();
                    state.bump();
                    if let Some(after_nl) = state.peek_kind() {
                        if after_nl == RstTokenType::Newline || self.is_block_start(after_nl) {
                            state.restore(cp);
                            break;
                        }
                    }
                    else {
                        break;
                    }
                }
                else if self.is_block_start(next_kind) {
                    break;
                }
            }
            self.parse_inline(state);
        }
        state.finish_at(checkpoint, ET::Paragraph);
    }

    fn parse_inlines_until_newline<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, RstLanguage, S>) {
        while state.not_at_end() {
            if let Some(kind) = state.peek_kind() {
                if kind == RstTokenType::Newline {
                    break;
                }
            }
            self.parse_inline(state);
        }
    }

    fn parse_inline<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, RstLanguage, S>) {
        let checkpoint = state.checkpoint();
        if let Some(kind) = state.peek_kind() {
            match kind {
                RstTokenType::Emphasis => {
                    state.bump(); // Start marker
                    while state.not_at_end() && state.peek_kind() != Some(RstTokenType::Emphasis) && state.peek_kind() != Some(RstTokenType::Newline) {
                        self.parse_inline(state);
                    }
                    if state.peek_kind() == Some(RstTokenType::Emphasis) {
                        state.bump(); // End marker
                    }
                    state.finish_at(checkpoint, ET::Emphasis);
                }
                RstTokenType::Strong => {
                    state.bump(); // Start marker
                    while state.not_at_end() && state.peek_kind() != Some(RstTokenType::Strong) && state.peek_kind() != Some(RstTokenType::Newline) {
                        self.parse_inline(state);
                    }
                    if state.peek_kind() == Some(RstTokenType::Strong) {
                        state.bump(); // End marker
                    }
                    state.finish_at(checkpoint, ET::Strong);
                }
                RstTokenType::Literal => {
                    state.bump();
                    // 解析字面量内容
                    while state.not_at_end() && state.peek_kind() != Some(RstTokenType::Literal) && state.peek_kind() != Some(RstTokenType::Newline) {
                        state.bump();
                    }
                    if state.peek_kind() == Some(RstTokenType::Literal) {
                        state.bump(); // End marker
                    }
                    state.finish_at(checkpoint, ET::Literal);
                }
                RstTokenType::Link => {
                    state.bump();
                    // 解析链接内容
                    while state.not_at_end() && state.peek_kind() != Some(RstTokenType::Link) && state.peek_kind() != Some(RstTokenType::Newline) {
                        self.parse_inline(state);
                    }
                    if state.peek_kind() == Some(RstTokenType::Link) {
                        state.bump(); // End marker
                    }
                    state.finish_at(checkpoint, ET::Link);
                }
                RstTokenType::FootnoteReference => {
                    state.bump();
                    state.finish_at(checkpoint, ET::FootnoteReference);
                }
                RstTokenType::SubstitutionReference => {
                    state.bump();
                    state.finish_at(checkpoint, ET::SubstitutionReference);
                }
                RstTokenType::Role => {
                    state.bump();
                    state.finish_at(checkpoint, ET::Role);
                }
                _ => {
                    state.bump();
                }
            }
        }
        else {
            state.advance();
        }
    }

    /// Parses complex table structures
    fn parse_table<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, RstLanguage, S>) {
        while state.not_at_end() {
            if let Some(RstTokenType::Newline) = state.peek_kind() {
                state.bump();
                if let Some(RstTokenType::Table) = state.peek_kind() {
                    state.bump();
                    // Parse table row
                    let row_checkpoint = state.checkpoint();
                    // Process table row content
                    self.parse_table_row(state);
                    state.finish_at(row_checkpoint, ET::TableRow);
                }
                else {
                    break;
                }
            }
            else {
                break;
            }
        }
    }

    /// Parses a single table row
    fn parse_table_row<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, RstLanguage, S>) {
        // Skip any leading whitespace
        while state.not_at_end() {
            if let Some(RstTokenType::Whitespace) = state.peek_kind() {
                state.bump();
            }
            else {
                break;
            }
        }

        // Process table cells
        while state.not_at_end() {
            if let Some(RstTokenType::Text) = state.peek_kind() {
                let cell_checkpoint = state.checkpoint();
                // Parse cell content until next | or end of line
                while state.not_at_end() {
                    if let Some(kind) = state.peek_kind() {
                        if kind == RstTokenType::Newline {
                            break;
                        }
                        state.bump();
                    }
                    else {
                        break;
                    }
                }
                state.finish_at(cell_checkpoint, ET::TableCell);
            }
            else if let Some(RstTokenType::Newline) = state.peek_kind() {
                break;
            }
            else {
                state.bump();
            }
        }
    }
}
