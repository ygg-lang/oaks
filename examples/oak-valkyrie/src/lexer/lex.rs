use crate::{ValkyrieLanguage, kind::ValkyrieSyntaxKind, lexer::ValkyrieKeywords};
use oak_core::{
    LexerState, OakError,
    lexer::{CommentConfig, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;
use unicode_ident::{is_xid_continue, is_xid_start};

type State<'a, S> = LexerState<'a, S, ValkyrieLanguage>;

static VK_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static VK_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "/*", block_end: "*/", nested_blocks: true });
static VK_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static VK_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

impl crate::lexer::ValkyrieLexer<'_> {
    /// Runs the lexer on the given state.
    pub(crate) fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_char_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            // If no lexer matched, add an error token and advance by one character to avoid infinite loop
            if let Some(c) = state.current() {
                let char_len = c.len_utf8();
                state.add_token(ValkyrieSyntaxKind::Error, safe_point, safe_point + char_len);
                state.advance(char_len);
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        VK_WHITESPACE.scan(state, ValkyrieSyntaxKind::Whitespace)
    }

    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        VK_COMMENT.scan(state, ValkyrieSyntaxKind::LineComment, ValkyrieSyntaxKind::BlockComment)
    }

    fn lex_string_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        VK_STRING.scan(state, ValkyrieSyntaxKind::StringLiteral)
    }

    fn lex_char_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        VK_CHAR.scan(state, ValkyrieSyntaxKind::CharLiteral)
    }

    fn lex_number_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                // 继续读取数字
                while let Some(ch) = state.current() {
                    if ch.is_ascii_digit() || ch == '.' || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
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
                    if is_xid_continue(ch) {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start..state.get_position()).into());
                let token_kind = match &*text {
                    "namespace" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Namespace),
                    "using" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Using),
                    "class" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Class),
                    "union" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Union),
                    "trait" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Trait),
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
                    "catch" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Catch),
                    "while" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::While),
                    "for" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::For),
                    "return" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Return),
                    "break" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Break),
                    "continue" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Continue),
                    "true" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::True),
                    "false" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::False),
                    "null" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Null),
                    "mut" => ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Mut),
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
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::PlusEq
                    }
                    else if let Some('+') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::PlusPlus
                    }
                    else {
                        ValkyrieSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::MinusEq
                    }
                    else if let Some('-') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::MinusMinus
                    }
                    else if let Some('>') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::Arrow
                    }
                    else {
                        ValkyrieSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::StarEq
                    }
                    else {
                        ValkyrieSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::SlashEq
                    }
                    else {
                        ValkyrieSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::PercentEq
                    }
                    else {
                        ValkyrieSyntaxKind::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::EqEq
                    }
                    else if let Some('>') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::Arrow
                    }
                    else {
                        ValkyrieSyntaxKind::Eq
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::NotEq
                    }
                    else {
                        ValkyrieSyntaxKind::Bang
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::LessEq
                    }
                    else if let Some('<') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::LeftShift
                    }
                    else {
                        ValkyrieSyntaxKind::LessThan
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::GreaterEq
                    }
                    else if let Some('>') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::RightShift
                    }
                    else {
                        ValkyrieSyntaxKind::GreaterThan
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::AndAnd
                    }
                    else {
                        ValkyrieSyntaxKind::Ampersand
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.current() {
                        state.advance(1);
                        ValkyrieSyntaxKind::OrOr
                    }
                    else {
                        ValkyrieSyntaxKind::Pipe
                    }
                }
                '^' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Caret
                }
                '~' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Tilde
                }
                '.' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Dot
                }
                ':' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Colon
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
            let kind = match ch {
                '(' => ValkyrieSyntaxKind::LeftParen,
                ')' => ValkyrieSyntaxKind::RightParen,
                '{' => ValkyrieSyntaxKind::LeftBrace,
                '}' => ValkyrieSyntaxKind::RightBrace,
                '[' => ValkyrieSyntaxKind::LeftBracket,
                ']' => ValkyrieSyntaxKind::RightBracket,
                ',' => ValkyrieSyntaxKind::Comma,
                ';' => ValkyrieSyntaxKind::Semicolon,
                '@' => ValkyrieSyntaxKind::At,
                '#' => ValkyrieSyntaxKind::Hash,
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
