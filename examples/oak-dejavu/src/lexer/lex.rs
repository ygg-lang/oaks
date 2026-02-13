use crate::{
    DejavuLanguage,
    lexer::{DejavuKeywords, token_type::DejavuSyntaxKind},
};
use oak_core::{
    LexerState, OakError,
    lexer::{CommentConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;
use unicode_ident::{is_xid_continue, is_xid_start};

type State<'a, S> = LexerState<'a, S, DejavuLanguage>;

static VK_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static VK_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "#", block_start: "/*", block_end: "*/", nested_blocks: true });

impl crate::lexer::DejavuLexer<'_> {
    /// Runs the lexer on the given state.
    pub(crate) fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        match self._config.syntax_mode {
            crate::language::SyntaxMode::Programming => self.run_programming(state),
            crate::language::SyntaxMode::Template => self.run_template(state),
        }
    }

    fn run_template<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        let start = state.get_position();
        let end = state.get_source().len();
        self.lex_interpolation(state, start, end, true);
        Ok(())
    }

    fn run_programming<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let start_pos = state.get_position();

            if self.lex_whitespace(state) || self.lex_comments(state) {
                continue;
            }

            let matched = self.lex_string_literal(state) || self.lex_char_literal(state) || self.lex_number_literal(state) || self.lex_identifier_or_keyword(state) || self.lex_operators(state) || self.lex_single_char_tokens(state);

            if !matched {
                if let Some(c) = state.current() {
                    let char_len = c.len_utf8();
                    state.add_token(DejavuSyntaxKind::Error, start_pos, start_pos + char_len);
                    state.advance(char_len);
                }
            }
        }

        Ok(())
    }

    fn lex_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let range = if VK_WHITESPACE.unicode_whitespace { state.take_while(|c| c.is_whitespace()) } else { state.skip_ascii_whitespace() };

        if range.end > start {
            state.add_token(DejavuSyntaxKind::Whitespace, start, range.end);
            true
        }
        else {
            false
        }
    }

    fn lex_comments<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        VK_COMMENT.scan(state, DejavuSyntaxKind::LineComment, DejavuSyntaxKind::BlockComment)
    }

    fn lex_string_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        self.lex_symmetric_string(state, '"', DejavuSyntaxKind::StringLiteral)
    }

    fn lex_char_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        self.lex_symmetric_string(state, '\'', DejavuSyntaxKind::CharLiteral)
    }

    fn lex_symmetric_string<S: Source + ?Sized>(&self, state: &mut State<'_, S>, quote: char, kind: DejavuSyntaxKind) -> bool {
        let start = state.get_position();
        let mut prefix: String = String::new();

        // 1. Try to scan prefix (Identifier)
        if let Some(c) = state.current() {
            if c != quote && (c == '_' || is_xid_start(c)) {
                let p_start = start;
                state.advance(c.len_utf8());
                while let Some(nc) = state.current() {
                    if is_xid_continue(nc) {
                        state.advance(nc.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                let p_end = state.get_position();
                // Check if the next character is the quote
                if state.current() == Some(quote) {
                    prefix = state.get_text_in((p_start..p_end).into()).into_owned();
                }
                else {
                    // Not a tagged string, backtrack
                    state.set_position(start);
                }
            }
        }

        let mut quote_count = 0;

        // 2. Count starting quotes
        while let Some(c) = state.current() {
            if c == quote {
                quote_count += 1;
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        if quote_count == 0 {
            state.set_position(start);
            return false;
        }

        // Rule: N=2 is an empty string. Others follow symmetric delimiter principle.
        if quote_count == 2 {
            state.add_token(kind, start, state.get_position());
            return true;
        }

        // 3. Symmetric rule: always look for the NEXT sequence of quote_count quotes.
        let mut current_consecutive = 0;
        let content_start = state.get_position();

        while let Some(c) = state.current() {
            if c == quote {
                current_consecutive += 1;
                state.advance(c.len_utf8());
                if current_consecutive == quote_count {
                    let end = state.get_position();
                    let content_end = end - quote_count * quote.len_utf8();

                    state.add_token(kind, start, end);

                    if content_start < content_end {
                        // Only raise interpolation if:
                        // 1. No prefix (Default Slot String)
                        // 2. Prefix is 's' (Explicit Slot String)
                        // 3. Prefix is 'f' (Format String)
                        // 4. Prefix is 't' (Template String)
                        let interpolation_enabled = prefix.is_empty() || prefix == "s" || prefix == "f" || prefix == "t";
                        self.lex_interpolation(state, content_start, content_end, interpolation_enabled);
                    }
                    return true;
                }
            }
            else {
                current_consecutive = 0;
                state.advance(c.len_utf8());
            }
        }

        // Unterminated string
        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_interpolation<S: Source + ?Sized>(&self, state: &mut State<'_, S>, start: usize, end: usize, interpolation_enabled: bool) {
        let mut current = start;
        let original_pos = state.get_position();

        state.set_position(start);

        while state.get_position() < end {
            if interpolation_enabled && (state.starts_with("\\{") || state.starts_with("\\}")) {
                state.advance(2);
                continue;
            }
            if interpolation_enabled && (state.starts_with("\\<") || state.starts_with("\\%") || state.starts_with("\\#")) {
                state.advance(2);
                continue;
            }

            if interpolation_enabled && state.starts_with("<#") {
                let part_end = state.get_position();
                if current < part_end {
                    state.add_token(DejavuSyntaxKind::StringPart, current, part_end)
                }

                let comment_start = state.get_position();
                state.advance(2); // skip <#
                state.add_token(DejavuSyntaxKind::TemplateCommentStart, comment_start, state.get_position());

                // Find matching #>
                while state.get_position() < end {
                    if state.starts_with("#>") {
                        let comment_end = state.get_position();
                        state.advance(2);
                        state.add_token(DejavuSyntaxKind::TemplateCommentEnd, comment_end, state.get_position());
                        break;
                    }
                    if let Some(c) = state.current() { state.advance(c.len_utf8()) } else { break }
                }
                current = state.get_position();
                continue;
            }

            if interpolation_enabled && state.starts_with("<%") {
                let part_end = state.get_position();
                if current < part_end {
                    state.add_token(DejavuSyntaxKind::StringPart, current, part_end)
                }

                let control_start = state.get_position();
                state.advance(2); // skip <%
                state.add_token(DejavuSyntaxKind::TemplateControlStart, control_start, state.get_position());

                let content_start = state.get_position();
                // Find matching %>
                while state.get_position() < end {
                    if state.starts_with("%>") {
                        break;
                    }
                    if let Some(c) = state.current() {
                        state.advance(c.len_utf8())
                    }
                    else {
                        break
                    }
                }
                let content_end = state.get_position();

                if content_start < content_end {
                    let mut sub_state = state.sub_state(content_start, content_end);
                    let _ = self.run_programming(&mut sub_state);
                }

                if state.starts_with("%>") {
                    let control_end = state.get_position();
                    state.advance(2);
                    state.add_token(DejavuSyntaxKind::TemplateControlEnd, control_end, state.get_position());
                }
                current = state.get_position();
                continue;
            }

            if interpolation_enabled && state.starts_with("{") {
                let part_end = state.get_position();
                if current < part_end {
                    state.add_token(DejavuSyntaxKind::StringPart, current, part_end)
                }

                let interp_start = state.get_position();
                state.advance(1); // skip {
                state.add_token(DejavuSyntaxKind::InterpolationStart, interp_start, state.get_position());

                let content_start = state.get_position();
                // Find matching }
                let mut depth = 1;
                while depth > 0 && state.get_position() < end {
                    if let Some(c) = state.current() {
                        if c == '{' {
                            depth += 1;
                        }
                        else if c == '}' {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        state.advance(c.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                let content_end = state.get_position();

                if content_start < content_end {
                    let mut sub_state = state.sub_state(content_start, content_end);
                    let _ = self.run_programming(&mut sub_state);
                }

                if state.at_char('}') {
                    let interp_end = state.get_position();
                    state.advance(1);
                    state.add_token(DejavuSyntaxKind::InterpolationEnd, interp_end, state.get_position());
                }
                current = state.get_position();
            }
            else if let Some(c) = state.current() {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        if current < end {
            state.add_token(DejavuSyntaxKind::StringPart, current, end);
        }

        state.set_position(original_pos);
    }

    fn lex_number_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                // 继续读取数字
                while let Some(ch) = state.current() {
                    if ch.is_ascii_digit() || ch == '.' || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                }

                state.add_token(DejavuSyntaxKind::IntegerLiteral, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            // Check if the first character is valid for an identifier
            if ch == '_' || is_xid_start(ch) {
                state.advance(ch.len_utf8());

                // Continue reading while we have valid identifier continuation characters
                while let Some(ch) = state.current() {
                    if is_xid_continue(ch) { state.advance(ch.len_utf8()) } else { break }
                }

                let end = state.get_position();
                let text = state.get_text_in((start..end).into());
                let token_kind = match &*text {
                    "namespace" => DejavuSyntaxKind::Keyword(DejavuKeywords::Namespace),
                    "using" => DejavuSyntaxKind::Keyword(DejavuKeywords::Using),
                    "class" => DejavuSyntaxKind::Keyword(DejavuKeywords::Class),
                    "singleton" => DejavuSyntaxKind::Keyword(DejavuKeywords::Singleton),
                    "trait" => DejavuSyntaxKind::Keyword(DejavuKeywords::Trait),
                    "flags" => DejavuSyntaxKind::Keyword(DejavuKeywords::Flags),
                    "enums" => DejavuSyntaxKind::Keyword(DejavuKeywords::Enums),
                    "union" => DejavuSyntaxKind::Keyword(DejavuKeywords::Union),
                    "micro" => DejavuSyntaxKind::Keyword(DejavuKeywords::Micro),
                    "mezzo" => DejavuSyntaxKind::Keyword(DejavuKeywords::Mezzo),
                    "macro" => DejavuSyntaxKind::Keyword(DejavuKeywords::Macro),
                    "widget" => DejavuSyntaxKind::Keyword(DejavuKeywords::Widget),
                    "let" => DejavuSyntaxKind::Keyword(DejavuKeywords::Let),
                    "if" => DejavuSyntaxKind::Keyword(DejavuKeywords::If),
                    "else" => DejavuSyntaxKind::Keyword(DejavuKeywords::Else),
                    "match" => DejavuSyntaxKind::Keyword(DejavuKeywords::Match),
                    "case" => DejavuSyntaxKind::Keyword(DejavuKeywords::Case),
                    "when" => DejavuSyntaxKind::Keyword(DejavuKeywords::When),
                    "try" => DejavuSyntaxKind::Keyword(DejavuKeywords::Try),
                    "lambda" => DejavuSyntaxKind::Keyword(DejavuKeywords::Lambda),
                    "catch" => DejavuSyntaxKind::Keyword(DejavuKeywords::Catch),
                    "while" => DejavuSyntaxKind::Keyword(DejavuKeywords::While),
                    "loop" => DejavuSyntaxKind::Keyword(DejavuKeywords::Loop),
                    "for" => DejavuSyntaxKind::Keyword(DejavuKeywords::For),
                    "in" => DejavuSyntaxKind::Keyword(DejavuKeywords::In),
                    "return" => DejavuSyntaxKind::Keyword(DejavuKeywords::Return),
                    "break" => DejavuSyntaxKind::Keyword(DejavuKeywords::Break),
                    "continue" => DejavuSyntaxKind::Keyword(DejavuKeywords::Continue),
                    "true" => DejavuSyntaxKind::Keyword(DejavuKeywords::True),
                    "false" => DejavuSyntaxKind::Keyword(DejavuKeywords::False),
                    "null" => DejavuSyntaxKind::Keyword(DejavuKeywords::Null),
                    "mut" => DejavuSyntaxKind::Keyword(DejavuKeywords::Mut),
                    "is" => DejavuSyntaxKind::Keyword(DejavuKeywords::Is),
                    "type" => DejavuSyntaxKind::Keyword(DejavuKeywords::Type),
                    "yield" => DejavuSyntaxKind::Keyword(DejavuKeywords::Yield),
                    "raise" => DejavuSyntaxKind::Keyword(DejavuKeywords::Raise),
                    "effect" => DejavuSyntaxKind::Keyword(DejavuKeywords::Effect),
                    "resume" => DejavuSyntaxKind::Keyword(DejavuKeywords::Resume),
                    "from" => DejavuSyntaxKind::Keyword(DejavuKeywords::From),
                    "as" => DejavuSyntaxKind::Keyword(DejavuKeywords::As),
                    "_" => DejavuSyntaxKind::Underscore,
                    _ => DejavuSyntaxKind::Identifier,
                };

                state.add_token(token_kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_operators<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        DejavuSyntaxKind::PlusEq
                    }
                    else if let Some('+') = state.current() {
                        state.advance('+'.len_utf8());
                        DejavuSyntaxKind::PlusPlus
                    }
                    else {
                        DejavuSyntaxKind::Plus
                    }
                }
                '-' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        DejavuSyntaxKind::MinusEq
                    }
                    else if let Some('-') = state.current() {
                        state.advance('-'.len_utf8());
                        DejavuSyntaxKind::MinusMinus
                    }
                    else if let Some('>') = state.current() {
                        state.advance('>'.len_utf8());
                        DejavuSyntaxKind::Arrow
                    }
                    else {
                        DejavuSyntaxKind::Minus
                    }
                }
                '*' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        DejavuSyntaxKind::StarEq
                    }
                    else {
                        DejavuSyntaxKind::Star
                    }
                }
                '/' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        DejavuSyntaxKind::SlashEq
                    }
                    else {
                        DejavuSyntaxKind::Slash
                    }
                }
                '%' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        DejavuSyntaxKind::PercentEq
                    }
                    else {
                        DejavuSyntaxKind::Percent
                    }
                }
                '=' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        DejavuSyntaxKind::EqEq
                    }
                    else if let Some('>') = state.current() {
                        state.advance('>'.len_utf8());
                        DejavuSyntaxKind::Arrow
                    }
                    else {
                        DejavuSyntaxKind::Eq
                    }
                }
                '!' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        DejavuSyntaxKind::NotEq
                    }
                    else {
                        DejavuSyntaxKind::Bang
                    }
                }
                '<' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        DejavuSyntaxKind::LessEq
                    }
                    else if let Some('<') = state.current() {
                        state.advance('<'.len_utf8());
                        DejavuSyntaxKind::LeftShift
                    }
                    else {
                        DejavuSyntaxKind::LessThan
                    }
                }
                '>' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        DejavuSyntaxKind::GreaterEq
                    }
                    else if let Some('>') = state.current() {
                        state.advance('>'.len_utf8());
                        DejavuSyntaxKind::RightShift
                    }
                    else {
                        DejavuSyntaxKind::GreaterThan
                    }
                }
                '&' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('&') = state.current() {
                        state.advance('&'.len_utf8());
                        DejavuSyntaxKind::AndAnd
                    }
                    else {
                        DejavuSyntaxKind::Ampersand
                    }
                }
                '|' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('|') = state.current() {
                        state.advance('|'.len_utf8());
                        DejavuSyntaxKind::OrOr
                    }
                    else if let Some('>') = state.current() {
                        state.advance('>'.len_utf8());
                        DejavuSyntaxKind::PipeGreater
                    }
                    else {
                        DejavuSyntaxKind::Pipe
                    }
                }
                '^' => {
                    state.advance(ch.len_utf8());
                    DejavuSyntaxKind::Caret
                }
                '~' => {
                    state.advance(ch.len_utf8());
                    DejavuSyntaxKind::Tilde
                }
                '.' => {
                    state.advance(ch.len_utf8());
                    DejavuSyntaxKind::Dot
                }
                ':' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some(':') = state.current() {
                        state.advance(':'.len_utf8());
                        DejavuSyntaxKind::ColonColon
                    }
                    else if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        DejavuSyntaxKind::ColonEq
                    }
                    else {
                        DejavuSyntaxKind::Colon
                    }
                }
                _ => return false,
            };
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_single_char_tokens<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            match ch {
                '@' => {
                    state.advance(ch.len_utf8());
                    state.add_token(DejavuSyntaxKind::At, start, state.get_position());
                    return true;
                }
                '↯' => {
                    state.advance(ch.len_utf8());
                    state.add_token(DejavuSyntaxKind::Bolt, start, state.get_position());
                    return true;
                }
                _ => {}
            }
            let kind = match ch {
                '(' => DejavuSyntaxKind::LeftParen,
                ')' => DejavuSyntaxKind::RightParen,
                '{' => DejavuSyntaxKind::LeftBrace,
                '}' => DejavuSyntaxKind::RightBrace,
                '[' => DejavuSyntaxKind::LeftBracket,
                ']' => DejavuSyntaxKind::RightBracket,
                ',' => DejavuSyntaxKind::Comma,
                ';' => DejavuSyntaxKind::Semicolon,
                '$' => DejavuSyntaxKind::Dollar,
                '?' => DejavuSyntaxKind::Question,
                _ => return false,
            };
            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }
}
