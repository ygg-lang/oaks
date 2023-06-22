use crate::{kind::GoSyntaxKind, language::GoLanguage};
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, GoLanguage>;

#[derive(Clone)]
pub struct GoLexer<'config> {
    _config: &'config GoLanguage,
}

impl<'config> Lexer<GoLanguage> for GoLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<GoLanguage>) -> LexOutput<GoLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> GoLexer<'config> {
    pub fn new(config: &'config GoLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_literal(state) {
                continue;
            }

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            // Fallback
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(GoSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }
        if state.get_position() > start {
            state.add_token(GoSyntaxKind::Whitespace, start, state.get_position());
            return true;
        }
        false
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.consume_if_starts_with("//") {
            while let Some(ch) = state.peek() {
                if ch == '\n' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(GoSyntaxKind::Comment, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("/*") {
            while let Some(ch) = state.peek() {
                if state.consume_if_starts_with("*/") {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(GoSyntaxKind::Comment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in(oak_core::Range { start, end: state.get_position() });
                let kind = match text.as_ref() {
                    "package" => GoSyntaxKind::Package,
                    "import" => GoSyntaxKind::Import,
                    "func" => GoSyntaxKind::Func,
                    "var" => GoSyntaxKind::Var,
                    "const" => GoSyntaxKind::Const,
                    "type" => GoSyntaxKind::Type,
                    "struct" => GoSyntaxKind::Struct,
                    "interface" => GoSyntaxKind::Interface,
                    "map" => GoSyntaxKind::Map,
                    "chan" => GoSyntaxKind::Chan,
                    "if" => GoSyntaxKind::If,
                    "else" => GoSyntaxKind::Else,
                    "for" => GoSyntaxKind::For,
                    "range" => GoSyntaxKind::Range,
                    "return" => GoSyntaxKind::Return,
                    "break" => GoSyntaxKind::Break,
                    "continue" => GoSyntaxKind::Continue,
                    "goto" => GoSyntaxKind::Goto,
                    "switch" => GoSyntaxKind::Switch,
                    "case" => GoSyntaxKind::Case,
                    "default" => GoSyntaxKind::Default,
                    "defer" => GoSyntaxKind::Defer,
                    "go" => GoSyntaxKind::Go,
                    "select" => GoSyntaxKind::Select,
                    "fallthrough" => GoSyntaxKind::Fallthrough,
                    "true" => GoSyntaxKind::BoolLiteral,
                    "false" => GoSyntaxKind::BoolLiteral,
                    "nil" => GoSyntaxKind::NilLiteral,
                    _ => GoSyntaxKind::Identifier,
                };

                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            // String literal
            if ch == '"' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch == '"' {
                        state.advance(ch.len_utf8());
                        break;
                    }
                    if ch == '\\' {
                        state.advance(ch.len_utf8());
                        if let Some(next) = state.peek() {
                            state.advance(next.len_utf8());
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(GoSyntaxKind::StringLiteral, start, state.get_position());
                return true;
            }
            // Raw string literal
            if ch == '`' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch == '`' {
                        state.advance(ch.len_utf8());
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(GoSyntaxKind::StringLiteral, start, state.get_position());
                return true;
            }
            // Rune literal
            if ch == '\'' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch == '\'' {
                        state.advance(ch.len_utf8());
                        break;
                    }
                    if ch == '\\' {
                        state.advance(ch.len_utf8());
                        if let Some(next) = state.peek() {
                            state.advance(next.len_utf8());
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(GoSyntaxKind::RuneLiteral, start, state.get_position());
                return true;
            }
            // Number literal
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());
                let mut has_dot = false;
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                    }
                    else if ch == '.' && !has_dot {
                        has_dot = true;
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                let kind = if has_dot { GoSyntaxKind::FloatLiteral } else { GoSyntaxKind::IntLiteral };
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_operator_or_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let kind = if state.consume_if_starts_with(":=") {
            GoSyntaxKind::ColonAssign
        }
        else if state.consume_if_starts_with("...") {
            GoSyntaxKind::Ellipsis
        }
        else if state.consume_if_starts_with("<<=") {
            GoSyntaxKind::LeftShiftAssign
        }
        else if state.consume_if_starts_with(">>=") {
            GoSyntaxKind::RightShiftAssign
        }
        else if state.consume_if_starts_with("&^=") {
            GoSyntaxKind::AmpersandCaretAssign
        }
        else if state.consume_if_starts_with("==") {
            GoSyntaxKind::Equal
        }
        else if state.consume_if_starts_with("!=") {
            GoSyntaxKind::NotEqual
        }
        else if state.consume_if_starts_with("<=") {
            GoSyntaxKind::LessEqual
        }
        else if state.consume_if_starts_with(">=") {
            GoSyntaxKind::GreaterEqual
        }
        else if state.consume_if_starts_with("&&") {
            GoSyntaxKind::LogicalAnd
        }
        else if state.consume_if_starts_with("||") {
            GoSyntaxKind::LogicalOr
        }
        else if state.consume_if_starts_with("<<") {
            GoSyntaxKind::LeftShift
        }
        else if state.consume_if_starts_with(">>") {
            GoSyntaxKind::RightShift
        }
        else if state.consume_if_starts_with("&^") {
            GoSyntaxKind::AmpersandCaret
        }
        else if state.consume_if_starts_with("++") {
            GoSyntaxKind::Increment
        }
        else if state.consume_if_starts_with("--") {
            GoSyntaxKind::Decrement
        }
        else if state.consume_if_starts_with("+=") {
            GoSyntaxKind::PlusAssign
        }
        else if state.consume_if_starts_with("-=") {
            GoSyntaxKind::MinusAssign
        }
        else if state.consume_if_starts_with("*=") {
            GoSyntaxKind::StarAssign
        }
        else if state.consume_if_starts_with("/=") {
            GoSyntaxKind::SlashAssign
        }
        else if state.consume_if_starts_with("%=") {
            GoSyntaxKind::PercentAssign
        }
        else if state.consume_if_starts_with("&=") {
            GoSyntaxKind::AmpersandAssign
        }
        else if state.consume_if_starts_with("|=") {
            GoSyntaxKind::PipeAssign
        }
        else if state.consume_if_starts_with("^=") {
            GoSyntaxKind::CaretAssign
        }
        else if state.consume_if_starts_with("<-") {
            GoSyntaxKind::Arrow
        }
        else if state.consume_if_starts_with("{") {
            GoSyntaxKind::LeftBrace
        }
        else if state.consume_if_starts_with("}") {
            GoSyntaxKind::RightBrace
        }
        else if state.consume_if_starts_with("(") {
            GoSyntaxKind::LeftParen
        }
        else if state.consume_if_starts_with(")") {
            GoSyntaxKind::RightParen
        }
        else if state.consume_if_starts_with("[") {
            GoSyntaxKind::LeftBracket
        }
        else if state.consume_if_starts_with("]") {
            GoSyntaxKind::RightBracket
        }
        else if state.consume_if_starts_with(".") {
            GoSyntaxKind::Dot
        }
        else if state.consume_if_starts_with(",") {
            GoSyntaxKind::Comma
        }
        else if state.consume_if_starts_with(";") {
            GoSyntaxKind::Semicolon
        }
        else if state.consume_if_starts_with(":") {
            GoSyntaxKind::Colon
        }
        else if state.consume_if_starts_with("+") {
            GoSyntaxKind::Plus
        }
        else if state.consume_if_starts_with("-") {
            GoSyntaxKind::Minus
        }
        else if state.consume_if_starts_with("*") {
            GoSyntaxKind::Star
        }
        else if state.consume_if_starts_with("/") {
            GoSyntaxKind::Slash
        }
        else if state.consume_if_starts_with("%") {
            GoSyntaxKind::Percent
        }
        else if state.consume_if_starts_with("&") {
            GoSyntaxKind::Ampersand
        }
        else if state.consume_if_starts_with("|") {
            GoSyntaxKind::Pipe
        }
        else if state.consume_if_starts_with("^") {
            GoSyntaxKind::Caret
        }
        else if state.consume_if_starts_with("!") {
            GoSyntaxKind::LogicalNot
        }
        else if state.consume_if_starts_with("<") {
            GoSyntaxKind::Less
        }
        else if state.consume_if_starts_with(">") {
            GoSyntaxKind::Greater
        }
        else if state.consume_if_starts_with("=") {
            GoSyntaxKind::Assign
        }
        else {
            return false;
        };

        state.add_token(kind, start, state.get_position());
        true
    }
}
