use crate::{RustToken, kind::RustSyntaxKind, language::RustLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, RustLanguage>;

static RS_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static RS_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static RS_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static RS_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: None });

#[derive(Clone)]
pub struct RustLexer<'config> {
    config: &'config RustLanguage,
}

impl<'config> Lexer<RustLanguage> for RustLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<RustLanguage>,
    ) -> LexOutput<RustLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> RustLexer<'config> {
    pub fn new(config: &'config RustLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
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

            if self.lex_lifetime(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            state.safe_check(safe_point);
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(RustSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match RS_WHITESPACE.scan(state.rest(), state.get_position(), RustSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();
        // line comment: // ... until newline
        if rest.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(RustSyntaxKind::Comment, start, state.get_position());
            return true;
        }
        // block comment: /* ... */ with nesting support
        if rest.starts_with("/*") {
            state.advance(2);
            let mut depth = 1usize;
            while let Some(ch) = state.peek() {
                if ch == '/' && state.peek_next_n(1) == Some('*') {
                    state.advance(2);
                    depth += 1;
                    continue;
                }
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    continue;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(RustSyntaxKind::Comment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        // raw byte string: br"..." or br#"..."#
        if state.current() == Some('b') && state.peek_next_n(1) == Some('r') {
            state.advance(2); // consume 'b' 'r'
            let mut hashes = 0usize;
            while state.peek() == Some('#') {
                state.advance(1);
                hashes += 1;
            }
            if state.peek() == Some('"') {
                state.advance(1);
            }
            loop {
                let ch_opt = state.peek();
                if ch_opt.is_none() {
                    break;
                }
                let ch = ch_opt.unwrap();
                if ch == '"' {
                    state.advance(1);
                    let mut ok = true;
                    for _ in 0..hashes {
                        if state.peek() == Some('#') {
                            state.advance(1);
                        }
                        else {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        break;
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(RustSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }

        // raw string: r"..." or r#"..."#
        if state.current() == Some('r') {
            state.advance(1);
            let mut hashes = 0usize;
            while state.peek() == Some('#') {
                state.advance(1);
                hashes += 1;
            }
            if state.peek() == Some('"') {
                state.advance(1);
            }
            loop {
                match state.peek() {
                    Some('"') => {
                        state.advance(1);
                        let mut ok = true;
                        for _ in 0..hashes {
                            if state.peek() == Some('#') {
                                state.advance(1);
                            }
                            else {
                                ok = false;
                                break;
                            }
                        }
                        if ok {
                            break;
                        }
                    }
                    Some(ch) => {
                        state.advance(ch.len_utf8());
                    }
                    None => break,
                }
            }
            state.add_token(RustSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }

        // byte string: b"..."
        if state.current() == Some('b') && state.peek_next_n(1) == Some('"') {
            state.advance(2); // consume b"
            let mut escaped = false;
            while let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                if escaped {
                    escaped = false;
                    continue;
                }
                if ch == '\\' {
                    escaped = true;
                    continue;
                }
                if ch == '"' {
                    break;
                }
            }
            state.add_token(RustSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }

        // normal string: "..."
        if state.current() == Some('"') {
            state.advance(1);
            let mut escaped = false;
            while let Some(ch) = state.peek() {
                if ch == '"' && !escaped {
                    state.advance(1); // consume closing quote
                    break;
                }
                state.advance(ch.len_utf8());
                if escaped {
                    escaped = false;
                    continue;
                }
                if ch == '\\' {
                    escaped = true;
                    continue;
                }
                if ch == '\n' || ch == '\r' {
                    break;
                }
            }
            state.add_token(RustSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_char_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if state.current() != Some('\'') {
            return false;
        }
        // try parse `'x'` or `'\n'` etc.; if fails, revert
        state.advance(1); // opening '
        if let Some('\\') = state.peek() {
            state.advance(1);
            if let Some(c) = state.peek() {
                state.advance(c.len_utf8());
            }
        }
        else if let Some(c) = state.peek() {
            state.advance(c.len_utf8());
        }
        else {
            state.set_position(start);
            return false;
        }
        if state.peek() == Some('\'') {
            state.advance(1);
            state.add_token(RustSyntaxKind::CharLiteral, start, state.get_position());
            return true;
        }
        state.set_position(start);
        false
    }

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };
        if !first.is_ascii_digit() {
            return false;
        }
        let mut is_float = false;
        if first == '0' {
            match state.peek_next_n(1) {
                Some('x') | Some('X') => {
                    state.advance(2);
                    while let Some(c) = state.peek() {
                        if c.is_ascii_hexdigit() || c == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                Some('b') | Some('B') => {
                    state.advance(2);
                    while let Some(c) = state.peek() {
                        if c == '0' || c == '1' || c == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                Some('o') | Some('O') => {
                    state.advance(2);
                    while let Some(c) = state.peek() {
                        if ('0'..='7').contains(&c) || c == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                _ => {
                    state.advance(1);
                    while let Some(c) = state.peek() {
                        if c.is_ascii_digit() || c == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }
        else {
            state.advance(1);
            while let Some(c) = state.peek() {
                if c.is_ascii_digit() || c == '_' {
                    state.advance(1);
                }
                else {
                    break;
                }
            }
        }
        // fractional part
        if state.peek() == Some('.') {
            let n1 = state.peek_next_n(1);
            if n1.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_float = true;
                state.advance(1); // consume '.'
                while let Some(c) = state.peek() {
                    if c.is_ascii_digit() || c == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
            }
        }
        // exponent
        if let Some(c) = state.peek() {
            if c == 'e' || c == 'E' {
                let n1 = state.peek_next_n(1);
                if n1 == Some('+') || n1 == Some('-') || n1.map(|d| d.is_ascii_digit()).unwrap_or(false) {
                    is_float = true;
                    state.advance(1);
                    if let Some(sign) = state.peek() {
                        if sign == '+' || sign == '-' {
                            state.advance(1);
                        }
                    }
                    while let Some(d) = state.peek() {
                        if d.is_ascii_digit() || d == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }
        // suffix letters (e.g., u32, i64, usize, f32, f64)
        while let Some(c) = state.peek() {
            if c.is_ascii_alphabetic() {
                state.advance(1);
            }
            else {
                break;
            }
        }
        let end = state.get_position();
        state.add_token(if is_float { RustSyntaxKind::FloatLiteral } else { RustSyntaxKind::IntegerLiteral }, start, end);
        true
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };
        if !(ch.is_ascii_alphabetic() || ch == '_') {
            return false;
        }
        state.advance(1);
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }
        let end = state.get_position();
        let text = state.get_text_in((start..end).into());
        let kind = match text {
            "as" => RustSyntaxKind::As,
            "async" => RustSyntaxKind::Async,
            "await" => RustSyntaxKind::Await,
            "break" => RustSyntaxKind::Break,
            "const" => RustSyntaxKind::Const,
            "continue" => RustSyntaxKind::Continue,
            "crate" => RustSyntaxKind::Crate,
            "dyn" => RustSyntaxKind::Dyn,
            "else" => RustSyntaxKind::Else,
            "enum" => RustSyntaxKind::Enum,
            "extern" => RustSyntaxKind::Extern,
            "false" => RustSyntaxKind::False,
            "fn" => RustSyntaxKind::Fn,
            "for" => RustSyntaxKind::For,
            "if" => RustSyntaxKind::If,
            "impl" => RustSyntaxKind::Impl,
            "in" => RustSyntaxKind::In,
            "let" => RustSyntaxKind::Let,
            "loop" => RustSyntaxKind::Loop,
            "match" => RustSyntaxKind::Match,
            "mod" => RustSyntaxKind::Mod,
            "move" => RustSyntaxKind::Move,
            "mut" => RustSyntaxKind::Mut,
            "pub" => RustSyntaxKind::Pub,
            "ref" => RustSyntaxKind::Ref,
            "return" => RustSyntaxKind::Return,
            "self" => RustSyntaxKind::SelfValue,
            "Self" => RustSyntaxKind::SelfType,
            "static" => RustSyntaxKind::Static,
            "struct" => RustSyntaxKind::Struct,
            "super" => RustSyntaxKind::Super,
            "trait" => RustSyntaxKind::Trait,
            "true" => RustSyntaxKind::True,
            "type" => RustSyntaxKind::Type,
            "unsafe" => RustSyntaxKind::Unsafe,
            "use" => RustSyntaxKind::Use,
            "where" => RustSyntaxKind::Where,
            "while" => RustSyntaxKind::While,
            _ => RustSyntaxKind::Identifier,
        };
        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_lifetime<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if state.peek() != Some('\'') {
            return false;
        }
        let next = state.peek_next_n(1);
        if !(next.is_some() && (next.unwrap().is_ascii_alphabetic() || next == Some('_'))) {
            return false;
        }
        // consume `'`
        state.advance(1);
        // consume lifetime name
        let mut consumed = false;
        while let Some(c) = state.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                state.advance(1);
                consumed = true;
            }
            else {
                break;
            }
        }
        if consumed {
            state.add_token(RustSyntaxKind::Lifetime, start, state.get_position());
            return true;
        }
        state.set_position(start);
        false
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();
        // prefer longest matches first
        let patterns: &[(&str, RustSyntaxKind)] = &[
            ("<<=", RustSyntaxKind::ShlEq),
            (">>=", RustSyntaxKind::ShrEq),
            ("..=", RustSyntaxKind::DotDotEq),
            ("...", RustSyntaxKind::DotDotDot),
            ("::", RustSyntaxKind::PathSep),
            ("->", RustSyntaxKind::RArrow),
            ("=>", RustSyntaxKind::FatArrow),
            ("&&", RustSyntaxKind::AndAnd),
            ("||", RustSyntaxKind::OrOr),
            ("<<", RustSyntaxKind::Shl),
            (">>", RustSyntaxKind::Shr),
            ("==", RustSyntaxKind::EqEq),
            ("!=", RustSyntaxKind::Ne),
            (">=", RustSyntaxKind::Ge),
            ("<=", RustSyntaxKind::Le),
            ("+=", RustSyntaxKind::PlusEq),
            ("-=", RustSyntaxKind::MinusEq),
            ("*=", RustSyntaxKind::StarEq),
            ("/=", RustSyntaxKind::SlashEq),
            ("%=", RustSyntaxKind::PercentEq),
            ("^=", RustSyntaxKind::CaretEq),
            ("&=", RustSyntaxKind::AndEq),
            ("|=", RustSyntaxKind::OrEq),
            ("..", RustSyntaxKind::DotDot),
        ];
        for (pat, kind) in patterns {
            if rest.starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }
        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => Some(RustSyntaxKind::Plus),
                '-' => Some(RustSyntaxKind::Minus),
                '*' => Some(RustSyntaxKind::Star),
                '/' => Some(RustSyntaxKind::Slash),
                '%' => Some(RustSyntaxKind::Percent),
                '^' => Some(RustSyntaxKind::Caret),
                '!' => Some(RustSyntaxKind::Not),
                '&' => Some(RustSyntaxKind::And),
                '|' => Some(RustSyntaxKind::Or),
                '=' => Some(RustSyntaxKind::Eq),
                '>' => Some(RustSyntaxKind::Gt),
                '<' => Some(RustSyntaxKind::Lt),
                '.' => Some(RustSyntaxKind::Dot),
                ':' => Some(RustSyntaxKind::Colon),
                _ => None,
            };
            if let Some(k) = kind {
                state.advance(ch.len_utf8());
                state.add_token(k, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => RustSyntaxKind::LeftParen,
                ')' => RustSyntaxKind::RightParen,
                '{' => RustSyntaxKind::LeftBrace,
                '}' => RustSyntaxKind::RightBrace,
                '[' => RustSyntaxKind::LeftBracket,
                ']' => RustSyntaxKind::RightBracket,
                ',' => RustSyntaxKind::Comma,
                ';' => RustSyntaxKind::Semicolon,
                '@' => RustSyntaxKind::At,
                '_' => RustSyntaxKind::Underscore,
                '#' => RustSyntaxKind::Pound,
                '$' => RustSyntaxKind::Dollar,
                '?' => RustSyntaxKind::Question,
                _ => return false,
            };
            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }
}
