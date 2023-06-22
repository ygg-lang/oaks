use crate::{kind::HtmlSyntaxKind, language::HtmlLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{LexOutput, StringConfig},
    source::{Source, TextEdit},
};
use std::{simd::prelude::*, sync::LazyLock};

type State<'a, S> = LexerState<'a, S, HtmlLanguage>;

// HTML 静态配置

static HTML_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"', '\''], escape: None });

#[derive(Clone, Debug)]
pub struct HtmlLexer<'config> {
    _config: &'config HtmlLanguage,
}

impl<'config> Lexer<HtmlLanguage> for HtmlLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<HtmlLanguage>) -> LexOutput<HtmlLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> HtmlLexer<'config> {
    pub fn new(config: &'config HtmlLanguage) -> Self {
        Self { _config: config }
    }

    /// 主要的词法分析循环
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if let Some(ch) = state.peek() {
                match ch {
                    ' ' | '\t' | '\n' | '\r' => {
                        self.skip_whitespace(state);
                    }
                    '<' => {
                        if let Some(next) = state.peek_next_n(1) {
                            if next == '!' {
                                if state.starts_with("<!--") {
                                    self.lex_comment(state);
                                }
                                else if state.starts_with("<![CDATA[") {
                                    self.lex_cdata(state);
                                }
                                else {
                                    // Try Doctype
                                    if !self.lex_doctype(state) {
                                        // Fallback to tag operator (TagOpen) or Text?
                                        // Original loop: tries doctype, cdata, then tag_operators.
                                        // If doctype fails (e.g. <!FOO>), tag_operators will see < and consume it as TagOpen.
                                        self.lex_tag_operators(state);
                                    }
                                }
                            }
                            else if next == '?' {
                                self.lex_processing_instruction(state);
                            }
                            else {
                                self.lex_tag_operators(state);
                            }
                        }
                        else {
                            self.lex_tag_operators(state);
                        }
                    }
                    '/' | '>' => {
                        if self.lex_tag_operators(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '&' => {
                        self.lex_entity_reference(state);
                    }
                    '"' | '\'' => {
                        self.lex_string_literal(state);
                    }
                    'a'..='z' | 'A'..='Z' | '_' | ':' => {
                        self.lex_identifier(state);
                    }
                    '=' => {
                        self.lex_single_char_tokens(state);
                    }
                    _ => {
                        if self.lex_text(state) {
                            continue;
                        }

                        // 安全点检查，防止无限循环
                        state.advance(ch.len_utf8());
                        state.add_token(HtmlSyntaxKind::Error, safe_point, state.get_position());
                    }
                }
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let bytes = state.rest_bytes();
        let mut i = 0;
        let len = bytes.len();
        const LANES: usize = 32;

        while i + LANES <= len {
            let chunk = Simd::<u8, LANES>::from_slice(unsafe { bytes.get_unchecked(i..i + LANES) });
            let is_le_space = chunk.simd_le(Simd::splat(32));

            if !is_le_space.all() {
                let not_space = !is_le_space;
                let idx = not_space.first_set().unwrap();
                i += idx;
                state.advance(i);
                state.add_token(HtmlSyntaxKind::Whitespace, start, state.get_position());
                return true;
            }
            i += LANES;
        }

        while i < len {
            if !unsafe { *bytes.get_unchecked(i) }.is_ascii_whitespace() {
                break;
            }
            i += 1;
        }

        if i > 0 {
            state.advance(i);
            state.add_token(HtmlSyntaxKind::Whitespace, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if !state.starts_with("<!--") {
            return false;
        }

        let start = state.get_position();
        let len = {
            let rest = state.rest();
            match rest.find("-->") {
                Some(end_at) => end_at + "-->".len(),
                None => rest.len(),
            }
        };
        state.advance(len);
        state.add_token(HtmlSyntaxKind::Comment, start, state.get_position());
        true
    }

    fn lex_doctype<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

    fn lex_cdata<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

    fn lex_processing_instruction<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

    fn lex_tag_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

    fn lex_entity_reference<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        HTML_STRING.scan(state, HtmlSyntaxKind::AttributeValue)
    }

    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        let bytes = state.rest_bytes();
        let mut i = 0;
        let len = bytes.len();
        const LANES: usize = 32;

        while i + LANES <= len {
            let chunk = Simd::<u8, LANES>::from_slice(unsafe { bytes.get_unchecked(i..i + LANES) });

            let is_lt = chunk.simd_eq(Simd::splat(b'<'));
            let is_amp = chunk.simd_eq(Simd::splat(b'&'));
            let is_le_space = chunk.simd_le(Simd::splat(32));

            let stop = is_lt | is_amp | is_le_space;

            if stop.any() {
                let idx = stop.first_set().unwrap();
                i += idx;
                state.advance(i);
                state.add_token(HtmlSyntaxKind::Text, start_pos, state.get_position());
                return true;
            }
            i += LANES;
        }

        while i < len {
            let ch = unsafe { *bytes.get_unchecked(i) };
            if ch == b'<' || ch == b'&' || ch.is_ascii_whitespace() {
                break;
            }
            i += 1;
        }

        if i > 0 {
            state.advance(i);
            state.add_token(HtmlSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}
