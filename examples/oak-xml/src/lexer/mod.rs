use crate::{kind::XmlSyntaxKind, language::XmlLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentBlock, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, XmlLanguage>;

// XML 静态配置
static XML_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

static XML_COMMENT: LazyLock<CommentBlock> =
    LazyLock::new(|| CommentBlock { block_markers: &[("<!--", "-->")], nested_blocks: false });

static XML_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"', '\''], escape: None });

impl<'config> Lexer<XmlLanguage> for XmlLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<XmlLanguage>,
    ) -> LexOutput<XmlLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

#[derive(Clone)]
pub struct XmlLexer<'config> {
    config: &'config XmlLanguage,
}

impl<'config> XmlLexer<'config> {
    pub fn new(config: &'config XmlLanguage) -> Self {
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

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(XmlSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match XML_WHITESPACE.scan(state.rest(), state.get_position(), XmlSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    /// 解析XML注释 <!-- ... -->
    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        match XML_COMMENT.scan(state.rest(), state.get_position(), XmlSyntaxKind::Comment) {
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
                // Check for DOCTYPE keyword
                let doctype_keyword = "DOCTYPE";
                let mut matches = true;
                for (i, expected_ch) in doctype_keyword.chars().enumerate() {
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
                    state.advance(2 + doctype_keyword.len()); // Skip <!DOCTYPE

                    let mut bracket_depth = 0;
                    // Find DOCTYPE end
                    while state.not_at_end() {
                        match state.peek() {
                            Some('[') => {
                                bracket_depth += 1;
                                state.advance(1);
                            }
                            Some(']') => {
                                bracket_depth -= 1;
                                state.advance(1);
                            }
                            Some('>') => {
                                if bracket_depth == 0 {
                                    state.advance(1); // Skip >
                                    state.add_token(XmlSyntaxKind::DoctypeDeclaration, start_pos, state.get_position());
                                    return true;
                                }
                                else {
                                    state.advance(1);
                                }
                            }
                            Some(ch) => {
                                state.advance(ch.len_utf8());
                            }
                            None => break,
                        }
                    }

                    // Unclosed DOCTYPE
                    state.add_token(XmlSyntaxKind::Error, start_pos, state.get_position());
                    return true;
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
                    // Check CDATA start tag
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
                                        state.add_token(XmlSyntaxKind::CData, start_pos, state.get_position());
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
                        state.add_token(XmlSyntaxKind::Error, start_pos, state.get_position());
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
                            state.add_token(XmlSyntaxKind::ProcessingInstruction, start_pos, state.get_position());
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
                state.add_token(XmlSyntaxKind::Error, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_tag_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        match state.peek() {
            Some('<') => {
                state.advance(1);
                if state.peek() == Some('/') {
                    state.advance(1);
                    state.add_token(XmlSyntaxKind::LeftAngleSlash, start_pos, state.get_position());
                }
                else {
                    state.add_token(XmlSyntaxKind::LeftAngle, start_pos, state.get_position());
                }
                true
            }
            Some('/') => {
                if state.peek_next_n(1) == Some('>') {
                    state.advance(2);
                    state.add_token(XmlSyntaxKind::SlashRightAngle, start_pos, state.get_position());
                    true
                }
                else {
                    false
                }
            }
            Some('>') => {
                state.advance(1);
                state.add_token(XmlSyntaxKind::RightAngle, start_pos, state.get_position());
                true
            }
            Some('=') => {
                state.advance(1);
                state.add_token(XmlSyntaxKind::Equals, start_pos, state.get_position());
                true
            }
            _ => false,
        }
    }

    fn lex_entity_reference<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('&') {
            state.advance(1);

            // Check for character reference &#...;
            if state.peek() == Some('#') {
                state.advance(1);
                let mut has_digits = false;

                // Hexadecimal character reference &#x...;
                if state.peek() == Some('x') {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() {
                            state.advance(1);
                            has_digits = true;
                        }
                        else {
                            break;
                        }
                    }
                }
                else {
                    // Decimal character reference &#...;
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                            has_digits = true;
                        }
                        else {
                            break;
                        }
                    }
                }

                if has_digits && state.peek() == Some(';') {
                    state.advance(1);
                    state.add_token(XmlSyntaxKind::CharacterReference, start_pos, state.get_position());
                    return true;
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
                    state.add_token(XmlSyntaxKind::EntityReference, start_pos, state.get_position());
                    return true;
                }
            }

            // Invalid entity reference
            state.add_token(XmlSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        match XML_STRING.scan(state.rest(), 0, XmlSyntaxKind::StringLiteral) {
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

                state.add_token(XmlSyntaxKind::Identifier, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        match state.peek() {
            Some('"') => {
                state.advance(1);
                state.add_token(XmlSyntaxKind::Quote, start_pos, state.get_position());
                true
            }
            Some('\'') => {
                state.advance(1);
                state.add_token(XmlSyntaxKind::SingleQuote, start_pos, state.get_position());
                true
            }
            Some('!') => {
                state.advance(1);
                state.add_token(XmlSyntaxKind::Exclamation, start_pos, state.get_position());
                true
            }
            Some('?') => {
                state.advance(1);
                state.add_token(XmlSyntaxKind::Question, start_pos, state.get_position());
                true
            }
            Some('&') => {
                state.advance(1);
                state.add_token(XmlSyntaxKind::Ampersand, start_pos, state.get_position());
                true
            }
            Some(';') => {
                state.advance(1);
                state.add_token(XmlSyntaxKind::Semicolon, start_pos, state.get_position());
                true
            }
            _ => false,
        }
    }

    fn lex_text<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // Stop at special characters
            match ch {
                ' ' | '\t' | '\n' | '\r' | '<' | '>' | '=' | '"' | '\'' | '!' | '?' | '&' | ';' => break,
                _ => {
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(XmlSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}
