use crate::{kind::WgslSyntaxKind, language::WgslLanguage};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, WgslLanguage>;

pub struct WgslLexer<'config> {
    #[allow(dead_code)]
    config: &'config WgslLanguage,
}

impl<'config> WgslLexer<'config> {
    pub fn new(config: &'config WgslLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
            state.add_token(WgslSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(WgslSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(WgslSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 单行注释 //
        if let Some('/') = state.peek() {
            if state.peek_next_n(1) == Some('/') {
                state.advance(2);

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(WgslSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        // 多行注释 /* */
        if let Some('/') = state.peek() {
            if state.peek_next_n(1) == Some('*') {
                state.advance(2);
                let mut depth = 1;

                while depth > 0 && state.not_at_end() {
                    if let Some('/') = state.peek() {
                        if state.peek_next_n(1) == Some('*') {
                            state.advance(2);
                            depth += 1;
                            continue;
                        }
                    }

                    if let Some('*') = state.peek() {
                        state.advance(1);
                        if state.peek_next_n(1) == Some('/') {
                            state.advance(1);
                            depth -= 1;
                        }
                    }
                    else if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(WgslSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1); // 跳过开始的引号

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1); // 跳过结束的引号
                    state.add_token(WgslSyntaxKind::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.advance(1); // 跳过转义字符
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8());
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(WgslSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理数字字面量
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 十六进制
                if ch == '0' && state.peek_next_n(1) == Some('x') {
                    state.advance(2); // 跳过 "0x"

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

                    if has_digits {
                        // 检查后缀
                        if let Some('u') = state.peek() {
                            state.advance(1);
                        }
                        else if let Some('i') = state.peek() {
                            state.advance(1);
                        }

                        state.add_token(WgslSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                        return true;
                    }
                }

                // 十进制数
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let mut is_float = false;

                // 小数
                if let Some('.') = state.peek() {
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1);
                            is_float = true;
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                // 科学计数
                if let Some('e') | Some('E') = state.peek() {
                    let exp_start = state.get_position();
                    state.advance(1);
                    is_float = true;

                    if let Some('+') | Some('-') = state.peek() {
                        state.advance(1);
                    }

                    let mut has_exp_digits = false;
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                            has_exp_digits = true;
                        }
                        else {
                            break;
                        }
                    }

                    if !has_exp_digits {
                        state.set_position(exp_start);
                        is_float = false;
                    }
                }

                // 后缀
                if is_float {
                    if let Some('f') = state.peek() {
                        state.advance(1);
                    }
                    state.add_token(WgslSyntaxKind::FloatLiteral, start_pos, state.get_position());
                }
                else {
                    if let Some('u') = state.peek() {
                        state.advance(1);
                    }
                    else if let Some('i') = state.peek() {
                        state.advance(1);
                    }
                    state.add_token(WgslSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                }

                return true;
            }
        }

        false
    }

    /// 处理标识符或关键字
    fn lex_ident_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end_pos = state.get_position();
                let text = state.get_text_in(oak_core::Range { start: start_pos, end: end_pos });
                let kind = match text.as_ref() {
                    "i32" => WgslSyntaxKind::I32Kw,
                    "u32" => WgslSyntaxKind::U32Kw,
                    "f32" => WgslSyntaxKind::F32Kw,
                    "f16" => WgslSyntaxKind::F16Kw,
                    "bool" => WgslSyntaxKind::BoolKw,
                    "vec2" => WgslSyntaxKind::Vec2Kw,
                    "vec3" => WgslSyntaxKind::Vec3Kw,
                    "vec4" => WgslSyntaxKind::Vec4Kw,
                    "mat2x2" => WgslSyntaxKind::Mat2x2Kw,
                    "mat3x3" => WgslSyntaxKind::Mat3x3Kw,
                    "mat4x4" => WgslSyntaxKind::Mat4x4Kw,
                    "array" => WgslSyntaxKind::ArrayKw,
                    "ptr" => WgslSyntaxKind::PtrKw,
                    "atomic" => WgslSyntaxKind::AtomicKw,
                    "texture_1d" => WgslSyntaxKind::Texture1dKw,
                    "texture_2d" => WgslSyntaxKind::Texture2dKw,
                    "texture_3d" => WgslSyntaxKind::Texture3dKw,
                    "texture_cube" => WgslSyntaxKind::TextureCubeKw,
                    "sampler" => WgslSyntaxKind::SamplerKw,
                    "if" => WgslSyntaxKind::IfKw,
                    "else" => WgslSyntaxKind::ElseKw,
                    "switch" => WgslSyntaxKind::SwitchKw,
                    "case" => WgslSyntaxKind::CaseKw,
                    "default" => WgslSyntaxKind::DefaultKw,
                    "loop" => WgslSyntaxKind::LoopKw,
                    "for" => WgslSyntaxKind::ForKw,
                    "while" => WgslSyntaxKind::WhileKw,
                    "break" => WgslSyntaxKind::BreakKw,
                    "continue" => WgslSyntaxKind::ContinueKw,
                    "return" => WgslSyntaxKind::ReturnKw,
                    "discard" => WgslSyntaxKind::DiscardKw,
                    "fn" => WgslSyntaxKind::FnKw,
                    "var" => WgslSyntaxKind::VarKw,
                    "let" => WgslSyntaxKind::LetKw,
                    "const" => WgslSyntaxKind::ConstKw,
                    "override" => WgslSyntaxKind::OverrideKw,
                    "struct" => WgslSyntaxKind::StructKw,
                    "alias" => WgslSyntaxKind::AliasKw,
                    "uniform" => WgslSyntaxKind::UniformKw,
                    "storage" => WgslSyntaxKind::StorageKw,
                    "workgroup" => WgslSyntaxKind::WorkgroupKw,
                    "private" => WgslSyntaxKind::PrivateKw,
                    "function" => WgslSyntaxKind::FunctionKw,
                    "read" => WgslSyntaxKind::ReadKw,
                    "write" => WgslSyntaxKind::WriteKw,
                    "read_write" => WgslSyntaxKind::ReadWriteKw,
                    "vertex" => WgslSyntaxKind::VertexKw,
                    "fragment" => WgslSyntaxKind::FragmentKw,
                    "compute" => WgslSyntaxKind::ComputeKw,
                    "true" => WgslSyntaxKind::BoolLiteral,
                    "false" => WgslSyntaxKind::BoolLiteral,
                    _ => WgslSyntaxKind::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标点符号和操作符
    fn lex_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '<' if state.peek_next_n(1) == Some('<') && state.peek_next_n(2) == Some('=') => {
                    state.advance(3);
                    WgslSyntaxKind::LeftShiftAssign
                }
                '>' if state.peek_next_n(1) == Some('>') && state.peek_next_n(2) == Some('=') => {
                    state.advance(3);
                    WgslSyntaxKind::RightShiftAssign
                }
                '+' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslSyntaxKind::PlusAssign
                }
                '-' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslSyntaxKind::MinusAssign
                }
                '*' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslSyntaxKind::StarAssign
                }
                '/' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslSyntaxKind::SlashAssign
                }
                '%' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslSyntaxKind::PercentAssign
                }
                '&' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslSyntaxKind::AmpersandAssign
                }
                '|' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslSyntaxKind::PipeAssign
                }
                '^' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslSyntaxKind::CaretAssign
                }
                '=' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslSyntaxKind::EqEq
                }
                '!' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslSyntaxKind::BangEq
                }
                '<' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslSyntaxKind::Le
                }
                '>' if state.peek_next_n(1) == Some('=') => {
                    state.advance(2);
                    WgslSyntaxKind::Ge
                }
                '<' if state.peek_next_n(1) == Some('<') => {
                    state.advance(2);
                    WgslSyntaxKind::LeftShift
                }
                '>' if state.peek_next_n(1) == Some('>') => {
                    state.advance(2);
                    WgslSyntaxKind::RightShift
                }
                '&' if state.peek_next_n(1) == Some('&') => {
                    state.advance(2);
                    WgslSyntaxKind::AmpersandAmpersand
                }
                '|' if state.peek_next_n(1) == Some('|') => {
                    state.advance(2);
                    WgslSyntaxKind::PipePipe
                }
                '-' if state.peek_next_n(1) == Some('>') => {
                    state.advance(2);
                    WgslSyntaxKind::Arrow
                }
                '+' => {
                    state.advance(1);
                    WgslSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    WgslSyntaxKind::Minus
                }
                '*' => {
                    state.advance(1);
                    WgslSyntaxKind::Star
                }
                '/' => {
                    state.advance(1);
                    WgslSyntaxKind::Slash
                }
                '%' => {
                    state.advance(1);
                    WgslSyntaxKind::Percent
                }
                '=' => {
                    state.advance(1);
                    WgslSyntaxKind::Assign
                }
                '<' => {
                    state.advance(1);
                    WgslSyntaxKind::Lt
                }
                '>' => {
                    state.advance(1);
                    WgslSyntaxKind::Gt
                }
                '!' => {
                    state.advance(1);
                    WgslSyntaxKind::Bang
                }
                '&' => {
                    state.advance(1);
                    WgslSyntaxKind::Ampersand
                }
                '|' => {
                    state.advance(1);
                    WgslSyntaxKind::Pipe
                }
                '^' => {
                    state.advance(1);
                    WgslSyntaxKind::Caret
                }
                '~' => {
                    state.advance(1);
                    WgslSyntaxKind::Tilde
                }
                '(' => {
                    state.advance(1);
                    WgslSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    WgslSyntaxKind::RightParen
                }
                '{' => {
                    state.advance(1);
                    WgslSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    WgslSyntaxKind::RightBrace
                }
                '[' => {
                    state.advance(1);
                    WgslSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    WgslSyntaxKind::RightBracket
                }
                ',' => {
                    state.advance(1);
                    WgslSyntaxKind::Comma
                }
                ';' => {
                    state.advance(1);
                    WgslSyntaxKind::Semicolon
                }
                ':' => {
                    state.advance(1);
                    WgslSyntaxKind::Colon
                }
                '.' => {
                    state.advance(1);
                    WgslSyntaxKind::Dot
                }
                '@' => {
                    state.advance(1);
                    WgslSyntaxKind::At
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理普通文本
    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            state.advance(ch.len_utf8());
            state.add_token(WgslSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
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

            if self.lex_ident_or_keyword(state) {
                continue;
            }

            if self.lex_punctuation(state) {
                continue;
            }

            if self.lex_text(state) {
                continue;
            }

            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(WgslSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        let eof_pos = state.get_position();
        state.add_token(WgslSyntaxKind::Eof, eof_pos, eof_pos);

        Ok(())
    }
}

impl<'config> Lexer<WgslLanguage> for WgslLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<WgslLanguage>) -> LexOutput<WgslLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}
