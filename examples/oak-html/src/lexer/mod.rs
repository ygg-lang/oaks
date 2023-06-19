use crate::{kind::HtmlSyntaxKind, language::HtmlLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentBlock, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, HtmlLanguage>;

// HTML 静态配置
static HTML_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

static HTML_COMMENT: LazyLock<CommentBlock> =
    LazyLock::new(|| CommentBlock { block_markers: &[("<!--", "-->")], nested_blocks: false });

static HTML_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"', '\''], escape: None });

#[derive(Clone)]
pub struct HtmlLexer<'config> {
    config: &'config HtmlLanguage,
}

impl<'config> Lexer<HtmlLanguage> for HtmlLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<HtmlLanguage>,
    ) -> LexOutput<HtmlLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> HtmlLexer<'config> {
    pub fn new(config: &'config HtmlLanguage) -> Self {
        Self { config }
    }

    /// 主要的词法分析循环
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_doctype(state) {
                continue;
            }

            if self.lex_cdata(state) {
                continue;
            }

            if self.lex_processing_instruction(state) {
                continue;
            }

            if self.lex_tag_operators(state) {
                continue;
            }

            if self.lex_entity_reference(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            if self.lex_text(state) {
                continue;
            }

            // 安全点检查，防止无限循环
            state.safe_check(safe_point);
        }

        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match HTML_WHITESPACE.scan(state.rest(), state.get_position(), HtmlSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        match HTML_COMMENT.scan(state.rest(), state.get_position(), HtmlSyntaxKind::Comment) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn lex_doctype<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            if let Some('!') = state.peek_next_n(1) {
                if let Some('D') = state.peek_next_n(2) {
                    let doctype_start = "DOCTYPE";
                    let mut matches = true;

                    for (i, expected_ch) in doctype_start.chars().enumerate() {
                        if let Some(actual_ch) = state.peek_next_n(2 + i) {
                            if actual_ch.to_ascii_uppercase() != expected_ch {
                                matches = false;
                                break;
                            }
                        }
                        else {
                            matches = false;
                            break;
                        }
                    }

                    if matches {
                        state.advance(2 + doctype_start.len()); // Skip <!DOCTYPE

                        // Find doctype end >
                        while state.not_at_end() {
                            if let Some('>') = state.peek() {
                                state.advance(1); // Skip >
                                state.add_token(HtmlSyntaxKind::Doctype, start_pos, state.get_position());
                                return true;
                            }
                            if let Some(ch) = state.peek() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }

                        // Unclosed doctype
                        state.add_token(HtmlSyntaxKind::Error, start_pos, state.get_position());
                        return true;
                    }
                }
            }
        }

        false
    }

    fn lex_cdata<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            if let Some('!') = state.peek_next_n(1) {
                if let Some('[') = state.peek_next_n(2) {
                    let cdata_start = "CDATA[";
                    let mut matches = true;

                    for (i, expected_ch) in cdata_start.chars().enumerate() {
                        if let Some(actual_ch) = state.peek_next_n(3 + i) {
                            if actual_ch != expected_ch {
                                matches = false;
                                break;
                            }
                        }
                        else {
                            matches = false;
                            break;
                        }
                    }

                    if matches {
                        state.advance(3 + cdata_start.len()); // Skip <![CDATA[

                        // Find CDATA end ]]>
                        while state.not_at_end() {
                            if let Some(']') = state.peek() {
                                if let Some(']') = state.peek_next_n(1) {
                                    if let Some('>') = state.peek_next_n(2) {
                                        state.advance(3); // Skip ]]>
                                        state.add_token(HtmlSyntaxKind::CData, start_pos, state.get_position());
                                        return true;
                                    }
                                }
                            }
                            if let Some(ch) = state.peek() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }

                        // Unclosed CDATA
                        state.add_token(HtmlSyntaxKind::Error, start_pos, state.get_position());
                        return true;
                    }
                }
            }
        }

        false
    }

    fn lex_processing_instruction<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            if let Some('?') = state.peek_next_n(1) {
                state.advance(2); // Skip <?

                // Find processing instruction end ?>
                while state.not_at_end() {
                    if let Some('?') = state.peek() {
                        if let Some('>') = state.peek_next_n(1) {
                            state.advance(2); // Skip ?>
                            state.add_token(HtmlSyntaxKind::ProcessingInstruction, start_pos, state.get_position());
                            return true;
                        }
                    }
                    if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // Unclosed processing instruction
                state.add_token(HtmlSyntaxKind::Error, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_tag_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        match state.peek() {
            Some('<') => {
                if let Some('/') = state.peek_next_n(1) {
                    state.advance(2);
                    state.add_token(HtmlSyntaxKind::TagSlashOpen, start_pos, state.get_position());
                    true
                }
                else {
                    state.advance(1);
                    state.add_token(HtmlSyntaxKind::TagOpen, start_pos, state.get_position());
                    true
                }
            }
            Some('/') => {
                if let Some('>') = state.peek_next_n(1) {
                    state.advance(2);
                    state.add_token(HtmlSyntaxKind::TagSelfClose, start_pos, state.get_position());
                    true
                }
                else {
                    false
                }
            }
            Some('>') => {
                state.advance(1);
                state.add_token(HtmlSyntaxKind::TagClose, start_pos, state.get_position());
                true
            }
            _ => false,
        }
    }

    fn lex_entity_reference<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('&') = state.peek() {
            state.advance(1);

            if let Some('#') = state.peek() {
                state.advance(1);

                // Character reference &#123; or &#x1A;
                if let Some('x') = state.peek() {
                    state.advance(1);
                    // Hexadecimal character reference
                    let mut has_digits = false;
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() {
                            state.advance(1);
                            has_digits = true;
                        }
                        else {
                            break;
                        }
                    }

                    if has_digits && state.peek() == Some(';') {
                        state.advance(1);
                        state.add_token(HtmlSyntaxKind::CharRef, start_pos, state.get_position());
                        return true;
                    }
                }
                else {
                    // Decimal character reference
                    let mut has_digits = false;
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                            has_digits = true;
                        }
                        else {
                            break;
                        }
                    }

                    if has_digits && state.peek() == Some(';') {
                        state.advance(1);
                        state.add_token(HtmlSyntaxKind::CharRef, start_pos, state.get_position());
                        return true;
                    }
                }
            }
            else {
                // Named entity reference &name;
                let mut has_name = false;
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() {
                        state.advance(1);
                        has_name = true;
                    }
                    else {
                        break;
                    }
                }

                if has_name && state.peek() == Some(';') {
                    state.advance(1);
                    state.add_token(HtmlSyntaxKind::EntityRef, start_pos, state.get_position());
                    return true;
                }
            }

            // Invalid entity reference
            state.add_token(HtmlSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        match HTML_STRING.scan(state.rest(), 0, HtmlSyntaxKind::AttributeValue) {
            Some(mut token) => {
                // Adjust token span to absolute position
                token.span.start += state.get_position();
                token.span.end += state.get_position();
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn lex_identifier<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == ':' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '.' || ch == ':' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(HtmlSyntaxKind::TagName, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        let kind = match state.peek() {
            Some('=') => HtmlSyntaxKind::Equal,
            Some('"') => HtmlSyntaxKind::Quote,
            Some('\'') => HtmlSyntaxKind::Quote,
            Some('!') => return false, // 已在其他地方处理
            Some('?') => return false, // 已在其他地方处理
            Some('&') => return false, // 已在其他地方处理
            Some(';') => return false, // 已在其他地方处理
            _ => return false,
        };

        if let Some(ch) = state.peek() {
            state.advance(ch.len_utf8());
            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_text<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();
        let mut has_text = false;

        while let Some(ch) = state.peek() {
            match ch {
                '<' | '&' => break,
                _ if ch.is_whitespace() => break,
                _ => {
                    state.advance(ch.len_utf8());
                    has_text = true;
                }
            }
        }

        if has_text {
            state.add_token(HtmlSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}
