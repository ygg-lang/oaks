/// Element types for the AsciiDoc language.
pub mod element_type;

use crate::{language::AsciidocLanguage, lexer::token_type::AsciidocTokenType, parser::element_type::AsciidocElementType as ET};
use oak_core::{Parser, ParserState, source::Source};

/// Parser for AsciiDoc language.
pub struct AsciidocParser<'config> {
    pub(crate) config: &'config AsciidocLanguage,
}

impl<'config> AsciidocParser<'config> {
    /// Creates a new AsciidocParser with the given configuration.
    pub fn new(config: &'config AsciidocLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<AsciidocLanguage> for AsciidocParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::ParseCache<AsciidocLanguage>) -> oak_core::ParseOutput<'a, AsciidocLanguage> {
        let lexer = crate::lexer::AsciidocLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                let item_checkpoint = state.checkpoint();
                if let Some(kind) = state.peek_kind() {
                    match kind {
                        AsciidocTokenType::Comment => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::Comment);
                        }
                        AsciidocTokenType::Heading => {
                            state.bump();
                            // 解析标题文本
                            self.parse_inlines_until_newline(state);
                            state.finish_at(item_checkpoint, ET::Heading);
                        }
                        AsciidocTokenType::BlockDelimiter => {
                            state.bump();
                            // 解析块内容
                            while state.not_at_end() {
                                if let Some(AsciidocTokenType::BlockDelimiter) = state.peek_kind() {
                                    state.bump();
                                    break;
                                }
                                self.parse_inline(state);
                            }
                            state.finish_at(item_checkpoint, ET::Block);
                        }
                        AsciidocTokenType::ListItemMarker => {
                            let list_checkpoint = item_checkpoint;
                            while state.not_at_end() {
                                if let Some(AsciidocTokenType::ListItemMarker) = state.peek_kind() {
                                    let li_checkpoint = state.checkpoint();
                                    state.bump(); // Marker
                                    self.parse_inlines_until_newline(state);
                                    state.finish_at(li_checkpoint, ET::ListItem);

                                    if let Some(AsciidocTokenType::Newline) = state.peek_kind() {
                                        let nl_checkpoint = state.checkpoint();
                                        state.bump();
                                        // 检查是否有嵌套列表
                                        let mut indent_level = 0;
                                        while state.not_at_end() {
                                            if let Some(AsciidocTokenType::Whitespace) = state.peek_kind() {
                                                state.bump();
                                                indent_level += 1;
                                            }
                                            else if let Some(AsciidocTokenType::ListItemMarker) = state.peek_kind() {
                                                // 递归解析嵌套列表
                                                let nested_list_checkpoint = state.checkpoint();
                                                while state.not_at_end() {
                                                    if let Some(AsciidocTokenType::ListItemMarker) = state.peek_kind() {
                                                        let nested_li_checkpoint = state.checkpoint();
                                                        state.bump(); // Marker
                                                        self.parse_inlines_until_newline(state);
                                                        state.finish_at(nested_li_checkpoint, ET::ListItem);

                                                        if let Some(AsciidocTokenType::Newline) = state.peek_kind() {
                                                            let nested_nl_checkpoint = state.checkpoint();
                                                            state.bump();
                                                            let mut nested_indent_level = 0;
                                                            while state.not_at_end() {
                                                                if let Some(AsciidocTokenType::Whitespace) = state.peek_kind() {
                                                                    state.bump();
                                                                    nested_indent_level += 1;
                                                                }
                                                                else {
                                                                    break;
                                                                }
                                                            }
                                                            if nested_indent_level <= indent_level || !matches!(state.peek_kind(), Some(AsciidocTokenType::ListItemMarker)) {
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
                                                state.finish_at(nested_list_checkpoint, ET::List);
                                                break;
                                            }
                                            else {
                                                state.restore(nl_checkpoint);
                                                break;
                                            }
                                        }
                                    }

                                    if let Some(AsciidocTokenType::Newline) = state.peek_kind() {
                                        let nl_checkpoint = state.checkpoint();
                                        state.bump();
                                        if !matches!(state.peek_kind(), Some(AsciidocTokenType::ListItemMarker)) {
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
                            state.finish_at(list_checkpoint, ET::List);
                        }
                        AsciidocTokenType::Table => {
                            let table_checkpoint = item_checkpoint;
                            state.bump();
                            // 解析表格行
                            while state.not_at_end() {
                                // 解析表格单元格
                                while state.not_at_end() {
                                    if let Some(AsciidocTokenType::TableCell) = state.peek_kind() {
                                        let cell_checkpoint = state.checkpoint();
                                        state.bump();
                                        // 解析单元格内容
                                        self.parse_inlines_until_newline(state);
                                        state.finish_at(cell_checkpoint, ET::TableCell);
                                    }
                                    else if let Some(AsciidocTokenType::Newline) = state.peek_kind() {
                                        state.bump();
                                        break;
                                    }
                                    else {
                                        break;
                                    }
                                }

                                // 检查是否有表格分隔符
                                if state.not_at_end() && state.peek_kind() == Some(AsciidocTokenType::TableSeparator) {
                                    let separator_checkpoint = state.checkpoint();
                                    state.bump();
                                    state.finish_at(separator_checkpoint, ET::TableSeparator);
                                    // 跳过换行
                                    if state.not_at_end() && state.peek_kind() == Some(AsciidocTokenType::Newline) {
                                        state.bump();
                                    }
                                }

                                // 检查下一行是否仍然是表格
                                if state.not_at_end() && state.peek_kind() != Some(AsciidocTokenType::Table) {
                                    break;
                                }
                                else if state.not_at_end() && state.peek_kind() == Some(AsciidocTokenType::Table) {
                                    state.bump();
                                }
                            }
                            state.finish_at(table_checkpoint, ET::Table);
                        }
                        AsciidocTokenType::CodeBlock => {
                            state.bump();
                            // 检查是否有语言指定
                            if let Some(AsciidocTokenType::CodeBlockLanguage) = state.peek_kind() {
                                state.bump();
                            }
                            // 解析代码块内容
                            while state.not_at_end() {
                                if let Some(AsciidocTokenType::Newline) = state.peek_kind() {
                                    state.bump();
                                    // 检查代码块结束 (```)
                                    let code_end_checkpoint = state.checkpoint();
                                    let mut backticks = 0;
                                    // 简化代码块结束检查
                                    while backticks < 3 && state.not_at_end() {
                                        state.bump();
                                        backticks += 1;
                                    }
                                    if backticks == 3 {
                                        break;
                                    }
                                    else {
                                        // 不是代码块结束，恢复状态
                                        state.restore(code_end_checkpoint);
                                    }
                                }
                                else {
                                    state.bump();
                                }
                            }
                            state.finish_at(item_checkpoint, ET::CodeBlock);
                        }
                        AsciidocTokenType::HorizontalRule => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::HorizontalRule);
                        }
                        AsciidocTokenType::Macro => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::Macro);
                        }
                        AsciidocTokenType::Attribute => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::Attribute);
                        }
                        AsciidocTokenType::FootnoteReference => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::FootnoteReference);
                        }
                        AsciidocTokenType::FootnoteDefinition => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::FootnoteDefinition);
                        }
                        AsciidocTokenType::Include => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::Include);
                        }
                        AsciidocTokenType::Ifdef => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::Ifdef);
                        }
                        AsciidocTokenType::Ifndef => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::Ifndef);
                        }
                        AsciidocTokenType::Endif => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::Endif);
                        }
                        AsciidocTokenType::TableCaption => {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::TableCaption);
                        }
                        AsciidocTokenType::Newline | AsciidocTokenType::Whitespace => {
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

impl<'config> AsciidocParser<'config> {
    fn is_block_start(&self, kind: AsciidocTokenType) -> bool {
        matches!(
            kind,
            AsciidocTokenType::Comment
                | AsciidocTokenType::Heading
                | AsciidocTokenType::BlockDelimiter
                | AsciidocTokenType::ListItemMarker
                | AsciidocTokenType::Table
                | AsciidocTokenType::TableCaption
                | AsciidocTokenType::CodeBlock
                | AsciidocTokenType::HorizontalRule
                | AsciidocTokenType::Include
                | AsciidocTokenType::Ifdef
                | AsciidocTokenType::Ifndef
                | AsciidocTokenType::Endif
        )
    }

    fn parse_paragraph<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, AsciidocLanguage, S>) {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            if let Some(next_kind) = state.peek_kind() {
                if next_kind == AsciidocTokenType::Newline {
                    let cp = state.checkpoint();
                    state.bump();
                    if let Some(after_nl) = state.peek_kind() {
                        if after_nl == AsciidocTokenType::Newline || self.is_block_start(after_nl) {
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

    fn parse_inlines_until_newline<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, AsciidocLanguage, S>) {
        while state.not_at_end() {
            if let Some(kind) = state.peek_kind() {
                if kind == AsciidocTokenType::Newline {
                    break;
                }
            }
            self.parse_inline(state);
        }
    }

    fn parse_inline<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, AsciidocLanguage, S>) {
        let checkpoint = state.checkpoint();
        if let Some(kind) = state.peek_kind() {
            match kind {
                AsciidocTokenType::Emphasis => {
                    state.bump(); // Start marker
                    while state.not_at_end() && state.peek_kind() != Some(AsciidocTokenType::Emphasis) && state.peek_kind() != Some(AsciidocTokenType::Newline) {
                        self.parse_inline(state);
                    }
                    if state.peek_kind() == Some(AsciidocTokenType::Emphasis) {
                        state.bump(); // End marker
                    }
                    state.finish_at(checkpoint, ET::Emphasis);
                }
                AsciidocTokenType::Strong => {
                    state.bump(); // Start marker
                    while state.not_at_end() && state.peek_kind() != Some(AsciidocTokenType::Strong) && state.peek_kind() != Some(AsciidocTokenType::Newline) {
                        self.parse_inline(state);
                    }
                    if state.peek_kind() == Some(AsciidocTokenType::Strong) {
                        state.bump(); // End marker
                    }
                    state.finish_at(checkpoint, ET::Strong);
                }
                AsciidocTokenType::Monospace => {
                    state.bump();
                    // 解析字面量内容
                    while state.not_at_end() && state.peek_kind() != Some(AsciidocTokenType::Monospace) && state.peek_kind() != Some(AsciidocTokenType::Newline) {
                        state.bump();
                    }
                    if state.peek_kind() == Some(AsciidocTokenType::Monospace) {
                        state.bump(); // End marker
                    }
                    state.finish_at(checkpoint, ET::Monospace);
                }
                AsciidocTokenType::Link => {
                    state.bump();
                    state.finish_at(checkpoint, ET::Link);
                }
                AsciidocTokenType::Image => {
                    state.bump();
                    state.finish_at(checkpoint, ET::Image);
                }
                AsciidocTokenType::CrossReference => {
                    state.bump();
                    state.finish_at(checkpoint, ET::CrossReference);
                }
                AsciidocTokenType::FootnoteReference => {
                    state.bump();
                    state.finish_at(checkpoint, ET::FootnoteReference);
                }
                AsciidocTokenType::Macro => {
                    state.bump();
                    state.finish_at(checkpoint, ET::Macro);
                }
                AsciidocTokenType::Attribute => {
                    state.bump();
                    state.finish_at(checkpoint, ET::Attribute);
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
}
