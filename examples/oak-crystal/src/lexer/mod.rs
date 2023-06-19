use crate::{kind::CrystalSyntaxKind, language::CrystalLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, lexer::LexOutput, source::Source};

type State<S> = LexerState<S, CrystalLanguage>;

/// Crystal 词法分析器
pub struct CrystalLexer<'config> {
    config: &'config CrystalLanguage,
}

impl<'config> CrystalLexer<'config> {
    pub fn new(config: &'config CrystalLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(CrystalSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(CrystalSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(CrystalSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            // 单行注释，读取到行尾
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(CrystalSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串
    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
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
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(CrystalSyntaxKind::String, start_pos, state.get_position());
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
    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                state.add_token(CrystalSyntaxKind::Number, start_pos, state.get_position());
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
    fn lex_keyword_or_identifier<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '?' || ch == '!' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let token_kind = match text {
                    // Crystal 关键字
                    "class" => CrystalSyntaxKind::ClassKeyword,
                    "module" => CrystalSyntaxKind::ModuleKeyword,
                    "def" => CrystalSyntaxKind::DefKeyword,
                    "end" => CrystalSyntaxKind::EndKeyword,
                    "if" => CrystalSyntaxKind::IfKeyword,
                    "else" => CrystalSyntaxKind::ElseKeyword,
                    "elsif" => CrystalSyntaxKind::ElsifKeyword,
                    "unless" => CrystalSyntaxKind::UnlessKeyword,
                    "case" => CrystalSyntaxKind::CaseKeyword,
                    "when" => CrystalSyntaxKind::WhenKeyword,
                    "then" => CrystalSyntaxKind::ThenKeyword,
                    "while" => CrystalSyntaxKind::WhileKeyword,
                    "until" => CrystalSyntaxKind::UntilKeyword,
                    "for" => CrystalSyntaxKind::ForKeyword,
                    "in" => CrystalSyntaxKind::InKeyword,
                    "do" => CrystalSyntaxKind::DoKeyword,
                    "begin" => CrystalSyntaxKind::BeginKeyword,
                    "rescue" => CrystalSyntaxKind::RescueKeyword,
                    "ensure" => CrystalSyntaxKind::EnsureKeyword,
                    "break" => CrystalSyntaxKind::BreakKeyword,
                    "next" => CrystalSyntaxKind::NextKeyword,
                    "return" => CrystalSyntaxKind::ReturnKeyword,
                    "yield" => CrystalSyntaxKind::YieldKeyword,
                    "super" => CrystalSyntaxKind::SuperKeyword,
                    "self" => CrystalSyntaxKind::SelfKeyword,
                    "true" => CrystalSyntaxKind::TrueKeyword,
                    "false" => CrystalSyntaxKind::FalseKeyword,
                    "nil" => CrystalSyntaxKind::NilKeyword,
                    "and" => CrystalSyntaxKind::AndKeyword,
                    "or" => CrystalSyntaxKind::OrKeyword,
                    "not" => CrystalSyntaxKind::NotKeyword,
                    _ => CrystalSyntaxKind::Identifier,
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
    fn lex_operator<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::PlusEqual
                    }
                    else {
                        CrystalSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::MinusEqual
                    }
                    else {
                        CrystalSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            CrystalSyntaxKind::StarStarEqual
                        }
                        else {
                            CrystalSyntaxKind::StarStar
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::StarEqual
                    }
                    else {
                        CrystalSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::SlashEqual
                    }
                    else {
                        CrystalSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::PercentEqual
                    }
                    else {
                        CrystalSyntaxKind::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::EqualEqual
                    }
                    else if let Some('~') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::Match
                    }
                    else {
                        CrystalSyntaxKind::Equal
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::NotEqual
                    }
                    else if let Some('~') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::NotMatch
                    }
                    else {
                        CrystalSyntaxKind::Not
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            CrystalSyntaxKind::Spaceship
                        }
                        else {
                            CrystalSyntaxKind::LessEqual
                        }
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            CrystalSyntaxKind::LeftShiftEqual
                        }
                        else {
                            CrystalSyntaxKind::LeftShift
                        }
                    }
                    else {
                        CrystalSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            CrystalSyntaxKind::RightShiftEqual
                        }
                        else {
                            CrystalSyntaxKind::RightShift
                        }
                    }
                    else {
                        CrystalSyntaxKind::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            CrystalSyntaxKind::LogicalAndEqual
                        }
                        else {
                            CrystalSyntaxKind::LogicalAnd
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::AndEqual
                    }
                    else {
                        CrystalSyntaxKind::BitwiseAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            CrystalSyntaxKind::LogicalOrEqual
                        }
                        else {
                            CrystalSyntaxKind::LogicalOr
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::OrEqual
                    }
                    else {
                        CrystalSyntaxKind::BitwiseOr
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::XorEqual
                    }
                    else {
                        CrystalSyntaxKind::BitwiseXor
                    }
                }
                '~' => {
                    state.advance(1);
                    CrystalSyntaxKind::BitwiseNot
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
    fn lex_delimiter<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => {
                    state.advance(1);
                    CrystalSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    CrystalSyntaxKind::RightParen
                }
                '{' => {
                    state.advance(1);
                    CrystalSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    CrystalSyntaxKind::RightBrace
                }
                '[' => {
                    state.advance(1);
                    CrystalSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    CrystalSyntaxKind::RightBracket
                }
                ',' => {
                    state.advance(1);
                    CrystalSyntaxKind::Comma
                }
                ';' => {
                    state.advance(1);
                    CrystalSyntaxKind::Semicolon
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        if let Some('.') = state.peek() {
                            state.advance(1);
                            CrystalSyntaxKind::DotDotDot
                        }
                        else {
                            CrystalSyntaxKind::DotDot
                        }
                    }
                    else {
                        CrystalSyntaxKind::Dot
                    }
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::DoubleColon
                    }
                    else {
                        CrystalSyntaxKind::Colon
                    }
                }
                '?' => {
                    state.advance(1);
                    CrystalSyntaxKind::Question
                }
                '@' => {
                    state.advance(1);
                    if let Some('@') = state.peek() {
                        state.advance(1);
                        CrystalSyntaxKind::DoubleAt
                    }
                    else {
                        CrystalSyntaxKind::At
                    }
                }
                '$' => {
                    state.advance(1);
                    CrystalSyntaxKind::Dollar
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

impl<'config> Lexer<CrystalLanguage> for CrystalLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        _changed: usize,
        _cache: IncrementalCache<CrystalLanguage>,
    ) -> LexOutput<CrystalLanguage> {
        let mut state = LexerState::new_with_cache(source, _changed, _cache);

        while state.not_at_end() {
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_keyword_or_identifier(&mut state) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果没有匹配到任何模式，处理错误字符并前进
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(CrystalSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        state.finish(Ok(()))
    }
}
