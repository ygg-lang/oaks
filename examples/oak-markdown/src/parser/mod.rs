/// Element types for the Markdown language.
pub mod element_type;

use crate::{language::MarkdownLanguage, lexer::token_type::MarkdownTokenType, parser::element_type::MarkdownElementType as ET};
use oak_core::{Parser, ParserState, source::Source};

/// Parser for Markdown language.
pub struct MarkdownParser<'config> {
    pub(crate) config: &'config MarkdownLanguage,
}

impl<'config> MarkdownParser<'config> {
    /// Creates a new MarkdownParser with the given configuration.
    pub fn new(config: &'config MarkdownLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<MarkdownLanguage> for MarkdownParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::ParseCache<MarkdownLanguage>) -> oak_core::ParseOutput<'a, MarkdownLanguage> {
        let lexer = crate::lexer::MarkdownLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
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
                                self.parse_paragraph(state);
                            }
                        }
                        MarkdownTokenType::MathBlock => {
                            if self.config.allow_math {
                                state.bump();
                                state.finish_at(item_checkpoint, ET::MathBlock);
                            }
                            else {
                                self.parse_paragraph(state);
                            }
                        }
                        MarkdownTokenType::HtmlTag | MarkdownTokenType::HtmlComment => {
                            if self.config.allow_html {
                                state.bump();
                                state.finish_at(item_checkpoint, ET::from(kind));
                            }
                            else {
                                self.parse_paragraph(state);
                            }
                        }
                        MarkdownTokenType::XmlTag | MarkdownTokenType::XmlComment => {
                            if self.config.allow_xml {
                                state.bump();
                                state.finish_at(item_checkpoint, ET::from(kind));
                            }
                            else {
                                self.parse_paragraph(state);
                            }
                        }
                        MarkdownTokenType::FootnoteDefinition => {
                            state.bump();
                            self.parse_inlines_until_newline(state);
                            state.finish_at(item_checkpoint, ET::FootnoteDefinition);
                        }
                        MarkdownTokenType::Heading1 | MarkdownTokenType::Heading2 | MarkdownTokenType::Heading3 | MarkdownTokenType::Heading4 | MarkdownTokenType::Heading5 | MarkdownTokenType::Heading6 => {
                            state.bump();
                            self.parse_inlines_until_newline(state);
                            state.finish_at(item_checkpoint, ET::from(kind));
                        }
                        MarkdownTokenType::ListMarker => {
                            let list_checkpoint = item_checkpoint;
                            let mut is_ordered = false;
                            if let Some(text) = state.peek_text() {
                                if text.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                                    is_ordered = true;
                                }
                            }

                            while state.not_at_end() {
                                if let Some(MarkdownTokenType::ListMarker) = state.peek_kind() {
                                    let current_is_ordered = if let Some(text) = state.peek_text() { text.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) } else { false };

                                    if current_is_ordered != is_ordered && state.checkpoint() != list_checkpoint {
                                        break;
                                    }

                                    let li_checkpoint = state.checkpoint();
                                    state.bump(); // Marker
                                    self.parse_inlines_until_newline(state);
                                    state.finish_at(li_checkpoint, ET::ListItem);

                                    if let Some(MarkdownTokenType::Newline) = state.peek_kind() {
                                        let nl_checkpoint = state.checkpoint();
                                        state.bump();
                                        if !matches!(state.peek_kind(), Some(MarkdownTokenType::ListMarker)) {
                                            state.restore(nl_checkpoint);
                                            break;
                                        }
                                        let next_is_ordered = if let Some(text) = state.peek_text() { text.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) } else { false };
                                        if next_is_ordered != is_ordered {
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
                            state.bump();
                            while state.not_at_end() {
                                if let Some(next_kind) = state.peek_kind() {
                                    if next_kind == MarkdownTokenType::Newline {
                                        state.bump();
                                        if let Some(after_nl) = state.peek_kind() {
                                            if after_nl == MarkdownTokenType::BlockquoteMarker {
                                                state.bump();
                                                continue;
                                            }
                                            if after_nl != MarkdownTokenType::Whitespace && after_nl != MarkdownTokenType::Text {
                                                break;
                                            }
                                        }
                                        else {
                                            break;
                                        }
                                    }
                                    else if self.is_block_start(next_kind) && next_kind != MarkdownTokenType::BlockquoteMarker {
                                        break;
                                    }
                                }
                                self.parse_inline(state);
                            }
                            state.finish_at(item_checkpoint, ET::Blockquote);
                        }
                        MarkdownTokenType::CodeFence => {
                            state.bump();
                            if let Some(MarkdownTokenType::CodeLanguage) = state.peek_kind() {
                                state.bump();
                            }
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
                            while state.not_at_end() {
                                while state.not_at_end() {
                                    if let Some(next_kind) = state.peek_kind() {
                                        if next_kind == MarkdownTokenType::Newline {
                                            break;
                                        }
                                    }
                                    state.bump();
                                }
                                if let Some(MarkdownTokenType::Newline) = state.peek_kind() {
                                    let checkpoint_before_nl = state.checkpoint();
                                    state.bump();
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
                                        continue;
                                    }
                                    else {
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

impl<'config> MarkdownParser<'config> {
    fn is_block_start(&self, kind: MarkdownTokenType) -> bool {
        matches!(
            kind,
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
        )
    }

    fn parse_paragraph<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, MarkdownLanguage, S>) {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            if let Some(next_kind) = state.peek_kind() {
                if next_kind == MarkdownTokenType::Newline {
                    let cp = state.checkpoint();
                    state.bump();
                    if let Some(after_nl) = state.peek_kind() {
                        if after_nl == MarkdownTokenType::Newline || self.is_block_start(after_nl) {
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

    fn parse_inlines_until_newline<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, MarkdownLanguage, S>) {
        while state.not_at_end() {
            if let Some(kind) = state.peek_kind() {
                if kind == MarkdownTokenType::Newline {
                    break;
                }
            }
            self.parse_inline(state);
        }
    }

    fn parse_inline<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, MarkdownLanguage, S>) {
        let checkpoint = state.checkpoint();
        if let Some(kind) = state.peek_kind() {
            match kind {
                MarkdownTokenType::Emphasis | MarkdownTokenType::Strong | MarkdownTokenType::Strikethrough => {
                    let marker_kind = kind;
                    state.bump(); // Start marker
                    while state.not_at_end() && state.peek_kind() != Some(marker_kind) && state.peek_kind() != Some(MarkdownTokenType::Newline) {
                        self.parse_inline(state);
                    }
                    if state.peek_kind() == Some(marker_kind) {
                        state.bump(); // End marker
                    }
                    state.finish_at(checkpoint, ET::from(marker_kind));
                }
                MarkdownTokenType::Link | MarkdownTokenType::Image => {
                    let is_image = kind == MarkdownTokenType::Image;
                    state.bump(); // [ or ![
                    // Parse link text
                    while state.not_at_end() && state.peek_text().as_deref() != Some("]") && state.peek_kind() != Some(MarkdownTokenType::Newline) {
                        self.parse_inline(state);
                    }
                    if state.peek_text().as_deref() == Some("]") {
                        state.bump();
                    }
                    // Parse URL if present (
                    if state.peek_text().as_deref() == Some("(") {
                        state.bump();
                        while state.not_at_end() && state.peek_text().as_deref() != Some(")") && state.peek_kind() != Some(MarkdownTokenType::Newline) {
                            state.bump();
                        }
                        if state.peek_text().as_deref() == Some(")") {
                            state.bump();
                        }
                    }
                    state.finish_at(checkpoint, if is_image { ET::Image } else { ET::Link });
                }
                MarkdownTokenType::InlineCode | MarkdownTokenType::MathInline | MarkdownTokenType::Superscript | MarkdownTokenType::Subscript | MarkdownTokenType::FootnoteReference => {
                    state.bump();
                    state.finish_at(checkpoint, ET::from(kind));
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
