#![doc = include_str!("readme.md")]
pub mod token_type;
use crate::language::CrystalLanguage;
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};
pub use token_type::CrystalTokenType;

type State<'a, S> = LexerState<'a, S, CrystalLanguage>;

/// Crystal 词法分析器
#[derive(Clone)]
pub struct CrystalLexer<'config> {
    #[allow(dead_code)]
    config: &'config CrystalLanguage,
}

impl<'config> Lexer<CrystalLanguage> for CrystalLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], mut cache: &'a mut impl LexerCache<CrystalLanguage>) -> LexOutput<CrystalLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, &mut cache)
    }
}

impl<'config> CrystalLexer<'config> {
    pub fn new(config: &'config CrystalLanguage) -> Self {
        Self { config }
    }

    /// 主要词法分析循环
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_keyword_or_identifier(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // 如果没有匹配任何规则，跳过当前字符并标记错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(CrystalTokenType::Error, start_pos, state.get_position())
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(CrystalTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(CrystalTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(CrystalTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            // 单行注释，读取到行尾
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8())
            }

            state.add_token(CrystalTokenType::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1)
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }

                state.add_token(CrystalTokenType::String, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理数字
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == '_' { state.advance(1) } else { break }
                }

                state.add_token(CrystalTokenType::Number, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理关键字或标识符
    fn lex_keyword_or_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '?' || ch == '!' { state.advance(ch.len_utf8()) } else { break }
                }

                let end_pos = state.get_position();
                let text = state.get_text_in(oak_core::Range { start: start_pos, end: end_pos });
                let token_kind = match text.as_ref() {
                    // Crystal 关键字
                    "class" => CrystalTokenType::ClassKeyword,
                    "module" => CrystalTokenType::ModuleKeyword,
                    "def" => CrystalTokenType::DefKeyword,
                    "end" => CrystalTokenType::EndKeyword,
                    "if" => CrystalTokenType::IfKeyword,
                    "else" => CrystalTokenType::ElseKeyword,
                    "elsif" => CrystalTokenType::ElsifKeyword,
                    "unless" => CrystalTokenType::UnlessKeyword,
                    "case" => CrystalTokenType::CaseKeyword,
                    "when" => CrystalTokenType::WhenKeyword,
                    "then" => CrystalTokenType::ThenKeyword,
                    "while" => CrystalTokenType::WhileKeyword,
                    "until" => CrystalTokenType::UntilKeyword,
                    "for" => CrystalTokenType::ForKeyword,
                    "in" => CrystalTokenType::InKeyword,
                    "do" => CrystalTokenType::DoKeyword,
                    "begin" => CrystalTokenType::BeginKeyword,
                    "rescue" => CrystalTokenType::RescueKeyword,
                    "ensure" => CrystalTokenType::EnsureKeyword,
                    "break" => CrystalTokenType::BreakKeyword,
                    "next" => CrystalTokenType::NextKeyword,
                    "return" => CrystalTokenType::ReturnKeyword,
                    "yield" => CrystalTokenType::YieldKeyword,
                    "super" => CrystalTokenType::SuperKeyword,
                    "self" => CrystalTokenType::SelfKeyword,
                    "true" => CrystalTokenType::TrueKeyword,
                    "false" => CrystalTokenType::FalseKeyword,
                    "nil" => CrystalTokenType::NilKeyword,
                    "and" => CrystalTokenType::AndKeyword,
                    "or" => CrystalTokenType::OrKeyword,
                    "not" => CrystalTokenType::NotKeyword,
                    _ => CrystalTokenType::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理操作符
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::PlusEqual
                    }
                    else {
                        CrystalTokenType::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::MinusEqual
                    }
                    else {
                        CrystalTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            CrystalTokenType::StarStarEqual
                        }
                        else {
                            CrystalTokenType::StarStar
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::StarEqual
                    }
                    else {
                        CrystalTokenType::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::SlashEqual
                    }
                    else {
                        CrystalTokenType::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::PercentEqual
                    }
                    else {
                        CrystalTokenType::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::EqualEqual
                    }
                    else if let Some('~') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::Match
                    }
                    else {
                        CrystalTokenType::Equal
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::NotEqual
                    }
                    else if let Some('~') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::NotMatch
                    }
                    else {
                        CrystalTokenType::Not
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            CrystalTokenType::Spaceship
                        }
                        else {
                            CrystalTokenType::LessEqual
                        }
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            CrystalTokenType::LeftShiftEqual
                        }
                        else {
                            CrystalTokenType::LeftShift
                        }
                    }
                    else {
                        CrystalTokenType::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            CrystalTokenType::RightShiftEqual
                        }
                        else {
                            CrystalTokenType::RightShift
                        }
                    }
                    else {
                        CrystalTokenType::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            CrystalTokenType::LogicalAndEqual
                        }
                        else {
                            CrystalTokenType::LogicalAnd
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::AndEqual
                    }
                    else {
                        CrystalTokenType::BitwiseAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            CrystalTokenType::LogicalOrEqual
                        }
                        else {
                            CrystalTokenType::LogicalOr
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::OrEqual
                    }
                    else {
                        CrystalTokenType::BitwiseOr
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::XorEqual
                    }
                    else {
                        CrystalTokenType::BitwiseXor
                    }
                }
                '~' => {
                    state.advance(1);
                    CrystalTokenType::BitwiseNot
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理分隔符
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => {
                    state.advance(1);
                    CrystalTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    CrystalTokenType::RightParen
                }
                '{' => {
                    state.advance(1);
                    CrystalTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    CrystalTokenType::RightBrace
                }
                '[' => {
                    state.advance(1);
                    CrystalTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    CrystalTokenType::RightBracket
                }
                ',' => {
                    state.advance(1);
                    CrystalTokenType::Comma
                }
                ';' => {
                    state.advance(1);
                    CrystalTokenType::Semicolon
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        if let Some('.') = state.peek() {
                            state.advance(1);
                            CrystalTokenType::DotDotDot
                        }
                        else {
                            CrystalTokenType::DotDot
                        }
                    }
                    else {
                        CrystalTokenType::Dot
                    }
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::DoubleColon
                    }
                    else {
                        CrystalTokenType::At // In Crystal, colon can be at the end of a symbol or for named arguments
                    }
                }
                '?' => {
                    state.advance(1);
                    CrystalTokenType::Question
                }
                '@' => {
                    state.advance(1);
                    if let Some('@') = state.peek() {
                        state.advance(1);
                        CrystalTokenType::DoubleAt
                    }
                    else {
                        CrystalTokenType::At
                    }
                }
                '$' => {
                    state.advance(1);
                    CrystalTokenType::Dollar
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}
