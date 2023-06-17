use crate::{kind::AdaSyntaxKind, language::AdaLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, AdaLanguage>;

pub struct AdaLexer<'config> {
    config: &'config AdaLanguage,
}

impl<'config> AdaLexer<'config> {
    pub fn new(config: &'config AdaLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State, source: &SourceText) -> bool {
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
            state.add_token(AdaSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(AdaSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(AdaSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('-') = state.peek() {
            if let Some('-') = state.peek_next_n(1) {
                state.advance(2); // 跳过 --

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(AdaSyntaxKind::Comment, start_pos, state.get_position());
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

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1); // 跳过开始引号
            let mut found_end = false;

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1); // 跳过结束引号
                    found_end = true;
                    break;
                }
                else if ch == '\\' {
                    state.advance(1); // 跳过转义字符
                    if state.peek().is_some() {
                        state.advance(1); // 跳过被转义的字符
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            if found_end {
                state.add_token(AdaSyntaxKind::StringLiteral, start_pos, state.get_position());
                true
            }
            else {
                // 回退到开始位置
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理字符字面量
    fn lex_character(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1); // 跳过开始引号
            let mut found_end = false;

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1); // 跳过转义字符
                    if state.peek().is_some() {
                        state.advance(1); // 跳过被转义的字符
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }

                if let Some('\'') = state.peek() {
                    state.advance(1); // 跳过结束引号
                    found_end = true;
                }
            }

            if found_end {
                state.add_token(AdaSyntaxKind::CharacterLiteral, start_pos, state.get_position());
                true
            }
            else {
                // 回退到开始位置
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理数字字面量
    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || ch == '.' || ch == '_' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(AdaSyntaxKind::NumberLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");

            let kind = match text.to_lowercase().as_str() {
                "abort" => AdaSyntaxKind::AbortKeyword,
                "abs" => AdaSyntaxKind::AbsKeyword,
                "abstract" => AdaSyntaxKind::AbstractKeyword,
                "accept" => AdaSyntaxKind::AcceptKeyword,
                "access" => AdaSyntaxKind::AccessKeyword,
                "aliased" => AdaSyntaxKind::AliasedKeyword,
                "all" => AdaSyntaxKind::AllKeyword,
                "and" => AdaSyntaxKind::AndKeyword,
                "array" => AdaSyntaxKind::ArrayKeyword,
                "at" => AdaSyntaxKind::AtKeyword,
                "begin" => AdaSyntaxKind::BeginKeyword,
                "body" => AdaSyntaxKind::BodyKeyword,
                "case" => AdaSyntaxKind::CaseKeyword,
                "constant" => AdaSyntaxKind::ConstantKeyword,
                "declare" => AdaSyntaxKind::DeclareKeyword,
                "delay" => AdaSyntaxKind::DelayKeyword,
                "delta" => AdaSyntaxKind::DeltaKeyword,
                "digits" => AdaSyntaxKind::DigitsKeyword,
                "do" => AdaSyntaxKind::DoKeyword,
                "else" => AdaSyntaxKind::ElseKeyword,
                "elsif" => AdaSyntaxKind::ElsifKeyword,
                "end" => AdaSyntaxKind::EndKeyword,
                "entry" => AdaSyntaxKind::EntryKeyword,
                "exception" => AdaSyntaxKind::ExceptionKeyword,
                "exit" => AdaSyntaxKind::ExitKeyword,
                "for" => AdaSyntaxKind::ForKeyword,
                "function" => AdaSyntaxKind::FunctionKeyword,
                "generic" => AdaSyntaxKind::GenericKeyword,
                "goto" => AdaSyntaxKind::GotoKeyword,
                "if" => AdaSyntaxKind::IfKeyword,
                "in" => AdaSyntaxKind::InKeyword,
                "interface" => AdaSyntaxKind::InterfaceKeyword,
                "is" => AdaSyntaxKind::IsKeyword,
                "limited" => AdaSyntaxKind::LimitedKeyword,
                "loop" => AdaSyntaxKind::LoopKeyword,
                "mod" => AdaSyntaxKind::ModKeyword,
                "new" => AdaSyntaxKind::NewKeyword,
                "not" => AdaSyntaxKind::NotKeyword,
                "null" => AdaSyntaxKind::NullKeyword,
                "of" => AdaSyntaxKind::OfKeyword,
                "or" => AdaSyntaxKind::OrKeyword,
                "others" => AdaSyntaxKind::OthersKeyword,
                "out" => AdaSyntaxKind::OutKeyword,
                "overriding" => AdaSyntaxKind::OverridingKeyword,
                "package" => AdaSyntaxKind::PackageKeyword,
                "pragma" => AdaSyntaxKind::PragmaKeyword,
                "private" => AdaSyntaxKind::PrivateKeyword,
                "procedure" => AdaSyntaxKind::ProcedureKeyword,
                "protected" => AdaSyntaxKind::ProtectedKeyword,
                "raise" => AdaSyntaxKind::RaiseKeyword,
                "range" => AdaSyntaxKind::RangeKeyword,
                "record" => AdaSyntaxKind::RecordKeyword,
                "rem" => AdaSyntaxKind::RemKeyword,
                "renames" => AdaSyntaxKind::RenamesKeyword,
                "requeue" => AdaSyntaxKind::RequeueKeyword,
                "return" => AdaSyntaxKind::ReturnKeyword,
                "reverse" => AdaSyntaxKind::ReverseKeyword,
                "select" => AdaSyntaxKind::SelectKeyword,
                "separate" => AdaSyntaxKind::SeparateKeyword,
                "subtype" => AdaSyntaxKind::SubtypeKeyword,
                "synchronized" => AdaSyntaxKind::SynchronizedKeyword,
                "tagged" => AdaSyntaxKind::TaggedKeyword,
                "task" => AdaSyntaxKind::TaskKeyword,
                "terminate" => AdaSyntaxKind::TerminateKeyword,
                "then" => AdaSyntaxKind::ThenKeyword,
                "type" => AdaSyntaxKind::TypeKeyword,
                "until" => AdaSyntaxKind::UntilKeyword,
                "use" => AdaSyntaxKind::UseKeyword,
                "when" => AdaSyntaxKind::WhenKeyword,
                "while" => AdaSyntaxKind::WhileKeyword,
                "with" => AdaSyntaxKind::WithKeyword,
                "xor" => AdaSyntaxKind::XorKeyword,
                _ => AdaSyntaxKind::Identifier,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理操作符和分隔符
    fn lex_operator_or_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => {
                    state.advance(1);
                    Some(AdaSyntaxKind::LeftParen)
                }
                ')' => {
                    state.advance(1);
                    Some(AdaSyntaxKind::RightParen)
                }
                '[' => {
                    state.advance(1);
                    Some(AdaSyntaxKind::LeftBracket)
                }
                ']' => {
                    state.advance(1);
                    Some(AdaSyntaxKind::RightBracket)
                }
                '{' => {
                    state.advance(1);
                    Some(AdaSyntaxKind::LeftBrace)
                }
                '}' => {
                    state.advance(1);
                    Some(AdaSyntaxKind::RightBrace)
                }
                ',' => {
                    state.advance(1);
                    Some(AdaSyntaxKind::Comma)
                }
                ';' => {
                    state.advance(1);
                    Some(AdaSyntaxKind::Semicolon)
                }
                ':' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        Some(AdaSyntaxKind::Assignment)
                    }
                    else {
                        state.advance(1);
                        Some(AdaSyntaxKind::Colon)
                    }
                }
                '.' => {
                    if let Some('.') = state.peek_next_n(1) {
                        state.advance(2);
                        Some(AdaSyntaxKind::DotDot)
                    }
                    else {
                        state.advance(1);
                        Some(AdaSyntaxKind::Dot)
                    }
                }
                '+' => {
                    state.advance(1);
                    Some(AdaSyntaxKind::Plus)
                }
                '-' => {
                    state.advance(1);
                    Some(AdaSyntaxKind::Minus)
                }
                '*' => {
                    if let Some('*') = state.peek_next_n(1) {
                        state.advance(2);
                        Some(AdaSyntaxKind::Power)
                    }
                    else {
                        state.advance(1);
                        Some(AdaSyntaxKind::Multiply)
                    }
                }
                '/' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        Some(AdaSyntaxKind::NotEqual)
                    }
                    else {
                        state.advance(1);
                        Some(AdaSyntaxKind::Divide)
                    }
                }
                '=' => {
                    if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        Some(AdaSyntaxKind::Arrow)
                    }
                    else {
                        state.advance(1);
                        Some(AdaSyntaxKind::Equal)
                    }
                }
                '<' => match state.peek_next_n(1) {
                    Some('=') => {
                        state.advance(2);
                        Some(AdaSyntaxKind::LessEqual)
                    }
                    Some('<') => {
                        state.advance(2);
                        Some(AdaSyntaxKind::LeftShift)
                    }
                    Some('>') => {
                        state.advance(2);
                        Some(AdaSyntaxKind::Box)
                    }
                    _ => {
                        state.advance(1);
                        Some(AdaSyntaxKind::Less)
                    }
                },
                '>' => match state.peek_next_n(1) {
                    Some('=') => {
                        state.advance(2);
                        Some(AdaSyntaxKind::GreaterEqual)
                    }
                    Some('>') => {
                        state.advance(2);
                        Some(AdaSyntaxKind::RightShift)
                    }
                    _ => {
                        state.advance(1);
                        Some(AdaSyntaxKind::Greater)
                    }
                },
                '&' => {
                    state.advance(1);
                    Some(AdaSyntaxKind::Ampersand)
                }
                '|' => {
                    state.advance(1);
                    Some(AdaSyntaxKind::Pipe)
                }
                _ => None,
            };

            if let Some(kind) = token_kind {
                state.add_token(kind, start_pos, state.get_position());
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
}

impl<'config> Lexer<AdaLanguage> for AdaLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<AdaSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state, source) {
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

            if self.lex_character(&mut state) {
                continue;
            }

            if self.lex_number(&mut state, source) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator_or_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(AdaSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF 标记
        let eof_pos = state.get_position();
        state.add_token(AdaSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
