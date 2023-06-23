use crate::{
    ValkyrieLanguage,
    lexer::{ValkyrieKeywords, token_type::ValkyrieSyntaxKind},
};
use oak_core::{
    LexerState, OakError,
    lexer::{CommentConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;
use unicode_ident::{is_xid_continue, is_xid_start};

type State<'a, S> = LexerState<'a, S, ValkyrieLanguage>;

static VK_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static VK_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "#", block_start: "/*", block_end: "*/", nested_blocks: true });

impl crate::lexer::ValkyrieLexer<'_> {
    /// Runs the lexer on the given state.
    pub(crate) fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        match self._config.syntax_mode {
            crate::language::SyntaxMode::Programming => self.run_programming(state),
        }
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
                    state.add_token(ValkyrieSyntaxKind::Error, start_pos, start_pos + char_len);
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
            state.add_token(ValkyrieSyntaxKind::Whitespace, start, range.end);
            true
        }
        else {
            false
        }
    }

    fn lex_comments<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        VK_COMMENT.scan(state, ValkyrieSyntaxKind::LineComment, ValkyrieSyntaxKind::BlockComment)
    }

    fn lex_string_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        self.lex_symmetric_string(state, '"', ValkyrieSyntaxKind::StringLiteral)
    }

    fn lex_char_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        self.lex_symmetric_string(state, '\'', ValkyrieSyntaxKind::CharLiteral)
    }

    fn lex_symmetric_string<S: Source + ?Sized>(&self, state: &mut State<'_, S>, quote: char, kind: ValkyrieSyntaxKind) -> bool {
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
                    state.add_token(ValkyrieSyntaxKind::StringPart, current, part_end)
                }

                let comment_start = state.get_position();
                state.advance(2); // skip <#
                state.add_token(ValkyrieSyntaxKind::TemplateCommentStart, comment_start, state.get_position());

                // Find matching #>
                while state.get_position() < end {
                    if state.starts_with("#>") {
                        let comment_end = state.get_position();
                        state.advance(2);
                        state.add_token(ValkyrieSyntaxKind::TemplateCommentEnd, comment_end, state.get_position());
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
                    state.add_token(ValkyrieSyntaxKind::StringPart, current, part_end)
                }

                let control_start = state.get_position();
                state.advance(2); // skip <%
                state.add_token(ValkyrieSyntaxKind::TemplateControlStart, control_start, state.get_position());

                // Find matching %>
                while state.get_position() < end {
                    if state.starts_with("%>") {
                        let control_end = state.get_position();
                        state.advance(2);
                        state.add_token(ValkyrieSyntaxKind::TemplateControlEnd, control_end, state.get_position());
                        break;
                    }
                    if let Some(c) = state.current() { state.advance(c.len_utf8()) } else { break }
                }
                current = state.get_position();
                continue;
            }

            if interpolation_enabled && state.starts_with("{") {
                let part_end = state.get_position();
                if current < part_end {
                    state.add_token(ValkyrieSyntaxKind::StringPart, current, part_end)
                }

                let interp_start = state.get_position();
                state.advance(1); // skip {
                state.add_token(ValkyrieSyntaxKind::InterpolationStart, interp_start, state.get_position());

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
                                let interp_end = state.get_position();
                                state.advance(1);
                                state.add_token(ValkyrieSyntaxKind::InterpolationEnd, interp_end, state.get_position());
                                break;
                            }
                        }
                        state.advance(c.len_utf8());
                    }
                    else {
                        break;
                    }
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
            state.add_token(ValkyrieSyntaxKind::StringPart, current, end);
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

                state.add_token(ValkyrieSyntaxKind::IntegerLiteral, start, state.get_position());
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
                    "namespace" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Namespace),
                    "using" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Using),
                    "class" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Class),
                    "singleton" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Singleton),
                    "trait" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Trait),
                    "flags" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Flags),
                    "enums" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Enums),
                    "union" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Union),
                    "micro" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Micro),
                    "mezzo" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Mezzo),
                    "macro" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Macro),
                    "widget" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Widget),
                    "let" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Let),
                    "if" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::If),
                    "else" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Else),
                    "match" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Match),
                    "case" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Case),
                    "when" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::When),
                    "try" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Try),
                    "lambda" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Lambda),
                    "catch" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Catch),
                    "while" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::While),
                    "loop" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Loop),
                    "for" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::For),
                    "in" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::In),
                    "return" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Return),
                    "break" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Break),
                    "continue" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Continue),
                    "true" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::True),
                    "false" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::False),
                    "null" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Null),
                    "mut" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Mut),
                    "is" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Is),
                    "type" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Type),
                    "yield" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Yield),
                    "raise" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Raise),
                    "effect" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Effect),
                    "resume" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Resume),
                    "from" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::From),
                    "as" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::As),
                    "_" => ValkyrieSyntaxKind::Underscore,
                    _ => ValkyrieSyntaxKind::Identifier,
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
                        ValkyrieSyntaxKind::PlusEq
                    }
                    else if let Some('+') = state.current() {
                        state.advance('+'.len_utf8());
                        ValkyrieSyntaxKind::PlusPlus
                    }
                    else {
                        ValkyrieSyntaxKind::Plus
                    }
                }
                '-' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        ValkyrieSyntaxKind::MinusEq
                    }
                    else if let Some('-') = state.current() {
                        state.advance('-'.len_utf8());
                        ValkyrieSyntaxKind::MinusMinus
                    }
                    else if let Some('>') = state.current() {
                        state.advance('>'.len_utf8());
                        ValkyrieSyntaxKind::Arrow
                    }
                    else {
                        ValkyrieSyntaxKind::Minus
                    }
                }
                '*' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        ValkyrieSyntaxKind::StarEq
                    }
                    else {
                        ValkyrieSyntaxKind::Star
                    }
                }
                '/' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        ValkyrieSyntaxKind::SlashEq
                    }
                    else {
                        ValkyrieSyntaxKind::Slash
                    }
                }
                '%' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        ValkyrieSyntaxKind::PercentEq
                    }
                    else {
                        ValkyrieSyntaxKind::Percent
                    }
                }
                '=' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        ValkyrieSyntaxKind::EqEq
                    }
                    else if let Some('>') = state.current() {
                        state.advance('>'.len_utf8());
                        ValkyrieSyntaxKind::Arrow
                    }
                    else {
                        ValkyrieSyntaxKind::Eq
                    }
                }
                '!' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        ValkyrieSyntaxKind::NotEq
                    }
                    else {
                        ValkyrieSyntaxKind::Bang
                    }
                }
                '<' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        ValkyrieSyntaxKind::LessEq
                    }
                    else if let Some('<') = state.current() {
                        state.advance('<'.len_utf8());
                        ValkyrieSyntaxKind::LeftShift
                    }
                    else {
                        ValkyrieSyntaxKind::LessThan
                    }
                }
                '>' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        ValkyrieSyntaxKind::GreaterEq
                    }
                    else if let Some('>') = state.current() {
                        state.advance('>'.len_utf8());
                        ValkyrieSyntaxKind::RightShift
                    }
                    else {
                        ValkyrieSyntaxKind::GreaterThan
                    }
                }
                '&' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('&') = state.current() {
                        state.advance('&'.len_utf8());
                        ValkyrieSyntaxKind::AndAnd
                    }
                    else {
                        ValkyrieSyntaxKind::Ampersand
                    }
                }
                '|' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some('|') = state.current() {
                        state.advance('|'.len_utf8());
                        ValkyrieSyntaxKind::OrOr
                    }
                    else if let Some('>') = state.current() {
                        state.advance('>'.len_utf8());
                        ValkyrieSyntaxKind::PipeGreater
                    }
                    else {
                        ValkyrieSyntaxKind::Pipe
                    }
                }
                '^' => {
                    state.advance(ch.len_utf8());
                    ValkyrieSyntaxKind::Caret
                }
                '~' => {
                    state.advance(ch.len_utf8());
                    ValkyrieSyntaxKind::Tilde
                }
                '.' => {
                    state.advance(ch.len_utf8());
                    ValkyrieSyntaxKind::Dot
                }
                ':' => {
                    let ch_len = ch.len_utf8();
                    state.advance(ch_len);
                    if let Some(':') = state.current() {
                        state.advance(':'.len_utf8());
                        ValkyrieSyntaxKind::ColonColon
                    }
                    else if let Some('=') = state.current() {
                        state.advance('='.len_utf8());
                        ValkyrieSyntaxKind::ColonEq
                    }
                    else {
                        ValkyrieSyntaxKind::Colon
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
                    state.add_token(ValkyrieSyntaxKind::At, start, state.get_position());
                    return true;
                }
                '↯' => {
                    state.advance(ch.len_utf8());
                    state.add_token(ValkyrieSyntaxKind::Bolt, start, state.get_position());
                    return true;
                }
                _ => {}
            }
            let kind = match ch {
                '(' => ValkyrieSyntaxKind::LeftParen,
                ')' => ValkyrieSyntaxKind::RightParen,
                '{' => ValkyrieSyntaxKind::LeftBrace,
                '}' => ValkyrieSyntaxKind::RightBrace,
                '[' => ValkyrieSyntaxKind::LeftBracket,
                ']' => ValkyrieSyntaxKind::RightBracket,
                ',' => ValkyrieSyntaxKind::Comma,
                ';' => ValkyrieSyntaxKind::Semicolon,
                '$' => ValkyrieSyntaxKind::Dollar,
                '?' => ValkyrieSyntaxKind::Question,
                _ => return false,
            };
            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }
}
