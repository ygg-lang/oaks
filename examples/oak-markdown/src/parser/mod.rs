pub mod element_type;

use crate::{language::MarkdownLanguage, lexer::token_type::MarkdownTokenType, parser::element_type::MarkdownElementType as ET};
use oak_core::{GreenNode, OakError, Parser, ParserState, source::Source};

pub(crate) type State<'a, S> = ParserState<'a, MarkdownLanguage, S>;

/// Parser for Markdown language.
pub struct MarkdownParser<'config> {
    pub(crate) config: &'config MarkdownLanguage,
}

impl<'config> MarkdownParser<'config> {
    /// Creates a new MarkdownParser with the given configuration.
    pub fn new(config: &'config MarkdownLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, MarkdownLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            let item_checkpoint = state.checkpoint();
            if let Some(kind) = state.peek_kind() {
                match kind {
                    MarkdownTokenType::FrontMatter => {
                        if self.config.allow_front_matter {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::FrontMatter);
                        }
                        else {
                            state.bump(); // Treat as text if not allowed? Or let it be handled by default?
                        }
                    }
                    MarkdownTokenType::MathBlock => {
                        if self.config.allow_math {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::MathBlock);
                        }
                        else {
                            state.bump();
                        }
                    }
                    MarkdownTokenType::HtmlTag | MarkdownTokenType::HtmlComment => {
                        if self.config.allow_html {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::from(kind));
                        }
                        else {
                            state.bump();
                        }
                    }
                    MarkdownTokenType::XmlTag | MarkdownTokenType::XmlComment => {
                        if self.config.allow_xml {
                            state.bump();
                            state.finish_at(item_checkpoint, ET::from(kind));
                        }
                        else {
                            state.bump();
                        }
                    }
                    MarkdownTokenType::FootnoteDefinition => {
                        state.bump();
                        // 消耗直到行尾
                        while state.not_at_end() {
                            if let Some(next_kind) = state.peek_kind() {
                                if next_kind == MarkdownTokenType::Newline {
                                    break;
                                }
                            }
                            state.bump();
                        }
                        state.finish_at(item_checkpoint, ET::FootnoteDefinition);
                    }
                    MarkdownTokenType::Heading1 | MarkdownTokenType::Heading2 | MarkdownTokenType::Heading3 | MarkdownTokenType::Heading4 | MarkdownTokenType::Heading5 | MarkdownTokenType::Heading6 => {
                        // 消耗标记和后续所有内容直到换行
                        state.bump();
                        while state.not_at_end() {
                            if let Some(next_kind) = state.peek_kind() {
                                if next_kind == MarkdownTokenType::Newline {
                                    break;
                                }
                            }
                            state.bump();
                        }
                        state.finish_at(item_checkpoint, ET::from(kind));
                    }
                    MarkdownTokenType::ListMarker => {
                        // 列表聚合逻辑：收集连续的列表项
                        let mut is_ordered = false;
                        if let Some(text) = state.peek_text() {
                            if text.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                                is_ordered = true;
                            }
                        }

                        let list_checkpoint = item_checkpoint;
                        while state.not_at_end() {
                            if let Some(MarkdownTokenType::ListMarker) = state.peek_kind() {
                                // 检查当前项是否与列表类型一致
                                let current_is_ordered = if let Some(text) = state.peek_text() { text.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) } else { false };

                                if current_is_ordered != is_ordered && state.checkpoint() != list_checkpoint {
                                    // 类型不一致且不是第一项，结束当前列表
                                    break;
                                }

                                let li_checkpoint = state.checkpoint();
                                state.bump(); // 消耗标记并存入树
                                while state.not_at_end() {
                                    if let Some(next_kind) = state.peek_kind() {
                                        if next_kind == MarkdownTokenType::Newline {
                                            break;
                                        }
                                    }
                                    state.bump();
                                }
                                state.finish_at(li_checkpoint, ET::ListItem);

                                // 消耗可能的换行，准备看下一个是否还是列表项
                                if let Some(MarkdownTokenType::Newline) = state.peek_kind() {
                                    let nl_checkpoint = state.checkpoint();
                                    state.bump();
                                    if !matches!(state.peek_kind(), Some(MarkdownTokenType::ListMarker)) {
                                        // 如果下一行不是列表项，或者我们要结束列表，回退换行（除非它是列表的一部分）
                                        // 这里简单处理：如果下一行不是列表项，就结束
                                        break;
                                    }
                                    // 检查下一行列表项类型是否一致
                                    let next_is_ordered = if let Some(text) = state.peek_text() { text.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) } else { false };
                                    if next_is_ordered != is_ordered {
                                        // 下一项类型不一致，不消耗这个换行，留给下一个列表
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
                    MarkdownTokenType::BlockquoteMarker => {
                        // 消耗 > 标记
                        state.bump();
                        // 收集引用内容直到遇到非引用的新行
                        while state.not_at_end() {
                            if let Some(next_kind) = state.peek_kind() {
                                if next_kind == MarkdownTokenType::Newline {
                                    state.bump();
                                    if let Some(after_nl) = state.peek_kind() {
                                        if after_nl != MarkdownTokenType::BlockquoteMarker && after_nl != MarkdownTokenType::Whitespace {
                                            break;
                                        }
                                    }
                                    else {
                                        break;
                                    }
                                }
                                else if next_kind == MarkdownTokenType::Heading1
                                    || next_kind == MarkdownTokenType::Heading2
                                    || next_kind == MarkdownTokenType::Heading3
                                    || next_kind == MarkdownTokenType::Heading4
                                    || next_kind == MarkdownTokenType::Heading5
                                    || next_kind == MarkdownTokenType::Heading6
                                    || next_kind == MarkdownTokenType::HorizontalRule
                                    || next_kind == MarkdownTokenType::CodeFence
                                    || next_kind == MarkdownTokenType::MathBlock
                                    || next_kind == MarkdownTokenType::FrontMatter
                                    || next_kind == MarkdownTokenType::FootnoteDefinition
                                {
                                    break;
                                }
                            }
                            state.bump();
                        }
                        state.finish_at(item_checkpoint, ET::Blockquote);
                    }
                    MarkdownTokenType::CodeFence => {
                        // 消耗开始围栏
                        state.bump();
                        // 消耗可能的语言标识
                        if let Some(MarkdownTokenType::CodeLanguage) = state.peek_kind() {
                            state.bump();
                        }
                        // 收集代码内容直到遇到结束围栏
                        while state.not_at_end() {
                            if let Some(next_kind) = state.peek_kind() {
                                if next_kind == MarkdownTokenType::CodeFence {
                                    state.bump();
                                    break;
                                }
                            }
                            state.bump();
                        }
                        state.finish_at(item_checkpoint, ET::CodeBlock);
                    }
                    MarkdownTokenType::HorizontalRule => {
                        state.bump();
                        state.finish_at(item_checkpoint, ET::HorizontalRule);
                    }
                    MarkdownTokenType::Pipe => {
                        // 表格聚合：消耗连续的包含 | 的行
                        while state.not_at_end() {
                            // 消耗当前行直到换行
                            while state.not_at_end() {
                                if let Some(next_kind) = state.peek_kind() {
                                    if next_kind == MarkdownTokenType::Newline {
                                        break;
                                    }
                                }
                                state.bump();
                            }

                            // 消耗换行并检查下一行
                            if let Some(MarkdownTokenType::Newline) = state.peek_kind() {
                                let checkpoint_before_nl = state.checkpoint();
                                state.bump();

                                // 检查下一行是否以 | 开头
                                let mut is_table_line = false;
                                while state.not_at_end() {
                                    if let Some(kind) = state.peek_kind() {
                                        if kind == MarkdownTokenType::Whitespace {
                                            state.bump();
                                        }
                                        else if kind == MarkdownTokenType::Pipe {
                                            is_table_line = true;
                                            break;
                                        }
                                        else {
                                            break;
                                        }
                                    }
                                    else {
                                        break;
                                    }
                                }

                                if is_table_line {
                                    // 是表格行，继续循环
                                    continue;
                                }
                                else {
                                    // 不是表格行，回退到换行前并退出
                                    state.restore(checkpoint_before_nl);
                                    break;
                                }
                            }
                            else {
                                break;
                            }
                        }
                        state.finish_at(item_checkpoint, ET::Table);
                    }
                    MarkdownTokenType::Newline | MarkdownTokenType::Whitespace => {
                        state.bump();
                    }
                    _ => {
                        // 收集段落内容：直到遇到两个换行或另一个块级元素
                        while state.not_at_end() {
                            if let Some(next_kind) = state.peek_kind() {
                                if next_kind == MarkdownTokenType::Newline {
                                    let _cp = state.checkpoint();
                                    state.bump();
                                    // 检查是否是连续换行
                                    if let Some(after_nl) = state.peek_kind() {
                                        if after_nl == MarkdownTokenType::Newline {
                                            state.bump();
                                            break;
                                        }
                                        // 或者是块级元素
                                        if matches!(
                                            after_nl,
                                            MarkdownTokenType::Heading1
                                                | MarkdownTokenType::Heading2
                                                | MarkdownTokenType::Heading3
                                                | MarkdownTokenType::Heading4
                                                | MarkdownTokenType::Heading5
                                                | MarkdownTokenType::Heading6
                                                | MarkdownTokenType::BlockquoteMarker
                                                | MarkdownTokenType::CodeFence
                                                | MarkdownTokenType::ListMarker
                                                | MarkdownTokenType::HorizontalRule
                                                | MarkdownTokenType::MathBlock
                                                | MarkdownTokenType::FrontMatter
                                                | MarkdownTokenType::FootnoteDefinition
                                        ) {
                                            break;
                                        }
                                    }
                                    else {
                                        break;
                                    }
                                }
                                else if matches!(
                                    next_kind,
                                    MarkdownTokenType::Heading1
                                        | MarkdownTokenType::Heading2
                                        | MarkdownTokenType::Heading3
                                        | MarkdownTokenType::Heading4
                                        | MarkdownTokenType::Heading5
                                        | MarkdownTokenType::Heading6
                                        | MarkdownTokenType::BlockquoteMarker
                                        | MarkdownTokenType::CodeFence
                                        | MarkdownTokenType::ListMarker
                                        | MarkdownTokenType::HorizontalRule
                                        | MarkdownTokenType::MathBlock
                                        | MarkdownTokenType::FrontMatter
                                        | MarkdownTokenType::FootnoteDefinition
                                ) {
                                    break;
                                }
                            }
                            state.bump();
                        }
                        state.finish_at(item_checkpoint, ET::Paragraph);
                    }
                }
            }
            else {
                state.advance();
            }
        }

        let root = state.finish_at(checkpoint, ET::Root);
        Ok(root)
    }
}

impl<'config> Parser<MarkdownLanguage> for MarkdownParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::ParseCache<MarkdownLanguage>) -> oak_core::ParseOutput<'a, MarkdownLanguage> {
        let lexer = crate::lexer::MarkdownLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
